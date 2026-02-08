use super::{get_db_version, set_db_version, CURRENT_VERSION};
use rusqlite::{Connection, Result};

/// Run the initial migration (create all tables from scratch)
pub fn run_initial_migration(conn: &Connection) -> Result<(), String> {
    println!("Running initial migration...");

    // Create employees table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS employees (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            father_name TEXT,
            spouse_name TEXT,
            current_place TEXT,
            current_post TEXT,
            current_address TEXT,
            phone_numbers TEXT,
            permanent_same_as_current INTEGER DEFAULT 0,
            permanent_place TEXT,
            permanent_post TEXT,
            permanent_address TEXT,
            emergency_contact_name TEXT,
            emergency_contact_relation TEXT,
            emergency_contact_phone TEXT,
            police_station TEXT,
            experience TEXT,
            job_post TEXT,
            employment_status TEXT DEFAULT 'applied',
            joining_date TEXT,
            exit_date TEXT,
            essid TEXT,
            photo_path TEXT,
            date_of_birth TEXT,
            uan TEXT,
            esiip TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create employees table: {}", e))?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_employment_status ON employees(employment_status)",
        [],
    )
    .map_err(|e| format!("Failed to create employment_status index: {}", e))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_current_place ON employees(current_place)",
        [],
    )
    .map_err(|e| format!("Failed to create current_place index: {}", e))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_current_post ON employees(current_post)",
        [],
    )
    .map_err(|e| format!("Failed to create current_post index: {}", e))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_job_post ON employees(job_post)",
        [],
    )
    .map_err(|e| format!("Failed to create job_post index: {}", e))?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_name ON employees(name)", [])
        .map_err(|e| format!("Failed to create name index: {}", e))?;

    // Create trigger for updated_at
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_employee_timestamp
         AFTER UPDATE ON employees
         BEGIN
            UPDATE employees SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
         END",
        [],
    )
    .map_err(|e| format!("Failed to create trigger: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS db_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    // Set initial version
    set_db_version(conn, 1)?;

    println!("Initial migration completed successfully");
    Ok(())
}

/// Run migrations to bring database up to current version
pub fn run_migrations(conn: &Connection) -> Result<(), String> {
    let current_version = get_db_version(conn)?;

    println!("Current database version: {}", current_version);
    println!("Target database version: {}", CURRENT_VERSION);

    if current_version >= CURRENT_VERSION {
        println!("Database is up to date");
        return Ok(());
    }

    // Run migrations in order
    for version in (current_version + 1)..=CURRENT_VERSION {
        println!("Running migration to version {}", version);

        match version {
            1 => migration_v1(conn)?,
            // 2 => migration_v2(conn)?,
            _ => return Err(format!("Unknown migration version: {}", version)),
        }

        set_db_version(conn, version)?;
        println!("Migration to version {} completed", version);
    }

    println!("All migrations completed successfully");
    Ok(())
}

/// Migration to version 1 (same as initial migration for existing databases)
fn migration_v1(conn: &Connection) -> Result<(), String> {
    run_initial_migration(conn)
}
