use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

use crate::APP_DATA_DIR;

pub mod backup;
mod migrations;

const DB_NAME: &str = "exgroup_app.db";
const CURRENT_VERSION: i32 = 1;

/// Get the database path in the app's data directory
pub fn get_db_path() -> Result<PathBuf, String> {
    let app_data_dir = APP_DATA_DIR
        .get()
        .ok_or_else(|| "Database path is not initialized yet".to_string())?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {e}"))?;

    Ok(app_data_dir.join(DB_NAME))
}

pub fn init_db() -> Result<Connection, String> {
    let db_path = get_db_path()?;
    let is_new_db = !db_path.exists();

    println!("Database path: {db_path:?}");

    let conn = Connection::open(&db_path).map_err(|e| format!("Failed to open database: {e}"))?;

    conn.execute("PRAGMA foreign_keys = ON", [])
        .map_err(|e| format!("Failed to enable foreign keys: {e}"))?;

    if is_new_db {
        println!("Creating new database...");
        migrations::run_initial_migration(&conn)?;
    } else {
        println!("Database exists, checking version...");
        migrations::run_migrations(&conn)?;
    }

    Ok(conn)
}

/// Get a connection to the database
pub fn get_connection() -> Result<Connection, String> {
    let db_path = get_db_path()?;
    Connection::open(&db_path).map_err(|e| format!("Failed to open database: {e}"))
}

/// Get current database version
pub fn get_db_version(conn: &Connection) -> Result<i32, String> {
    // Create version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS db_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create version table: {e}"))?;

    let version: Result<i32, rusqlite::Error> = conn.query_row(
        "SELECT version FROM db_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get(0),
    );

    Ok(version.unwrap_or(0))
}

/// Set database version
pub fn set_db_version(conn: &Connection, version: i32) -> Result<(), String> {
    conn.execute("INSERT INTO db_version (version) VALUES (?1)", [version])
        .map_err(|e| format!("Failed to set database version: {e}"))?;

    Ok(())
}
