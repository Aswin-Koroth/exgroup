use chrono::Local;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

/// Create a backup of the database
pub fn create_backup(conn: &Connection, backup_dir: &PathBuf) -> Result<PathBuf, String> {
    // Create backup directory if it doesn't exist
    fs::create_dir_all(backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    // Generate backup filename with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("employee_management_backup_{}.db", timestamp);
    let backup_path = backup_dir.join(backup_filename);

    // Perform backup using SQLite's backup API
    let mut backup_conn = Connection::open(&backup_path)
        .map_err(|e| format!("Failed to create backup connection: {}", e))?;

    let backup = rusqlite::backup::Backup::new(conn, &mut backup_conn)
        .map_err(|e| format!("Failed to initialize backup: {}", e))?;

    backup
        .run_to_completion(5, std::time::Duration::from_millis(250), None)
        .map_err(|e| format!("Failed to complete backup: {}", e))?;

    println!("Backup created: {:?}", backup_path);
    Ok(backup_path)
}

/// Clean old backups (keep only last N backups)
pub fn clean_old_backups(backup_dir: &PathBuf, keep_count: usize) -> Result<(), String> {
    let mut backups: Vec<PathBuf> = fs::read_dir(backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "db")
                .unwrap_or(false)
        })
        .collect();

    // Sort by modification time (newest first)
    backups.sort_by(|a, b| {
        let a_time = fs::metadata(a).and_then(|m| m.modified()).ok();
        let b_time = fs::metadata(b).and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    // Remove old backups
    for old_backup in backups.iter().skip(keep_count) {
        fs::remove_file(old_backup).map_err(|e| format!("Failed to remove old backup: {}", e))?;
        println!("Removed old backup: {:?}", old_backup);
    }

    Ok(())
}
