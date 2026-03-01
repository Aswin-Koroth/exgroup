use super::DB_NAME;
use chrono::Local;
use rusqlite::Connection;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Create a backup of the database
pub fn create_backup(conn: &Connection, backup_dir: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {e}"))?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("{DB_NAME}_backup_{timestamp}.db");
    let backup_path = backup_dir.join(backup_filename);

    let mut backup_conn = Connection::open(&backup_path)
        .map_err(|e| format!("Failed to create backup connection: {e}"))?;

    let backup = rusqlite::backup::Backup::new(conn, &mut backup_conn)
        .map_err(|e| format!("Failed to initialize backup: {e}"))?;

    backup
        .run_to_completion(5, std::time::Duration::from_millis(250), None)
        .map_err(|e| format!("Failed to complete backup: {e}"))?;

    println!("Backup created: {backup_path:?}");
    Ok(backup_path)
}

/// Clean old backups (keep only last N backups)
pub fn clean_old_backups(backup_dir: &Path, keep_count: usize) -> Result<(), String> {
    let mut backups: Vec<PathBuf> = fs::read_dir(backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {e}"))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "db")
                .unwrap_or(false)
        })
        .collect();

    backups.sort_by(|a, b| {
        let a_time = fs::metadata(a).and_then(|m| m.modified()).ok();
        let b_time = fs::metadata(b).and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    // Remove old backups
    for old_backup in backups.iter().skip(keep_count) {
        fs::remove_file(old_backup).map_err(|e| format!("Failed to remove old backup: {e}"))?;
        println!("Removed old backup: {old_backup:?}");
    }

    Ok(())
}

pub fn export_to_csv(conn: &Connection, export_path: &Path) -> Result<String, String> {
    use std::io::BufWriter;

    let file = File::create(export_path).map_err(|e| format!("Failed to create file: {e}"))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "ID,Name,Father Name,Spouse Name,Current Place,Current Post,Current Address,Phone Numbers,Permanent Same As Current,Permanent Place,Permanent Post,Permanent Address,Emergency Contact Name,Emergency Contact Relation,Emergency Contact Phone,Police Station,Experience,Job Post,Employment Status,Joining Date,Exit Date,ESSID,Photo Path,Date of Birth,UAN,ESIIP,Created At,Updated At")
        .map_err(|e| format!("Failed to write header: {e}"))?;

    let mut stmt = conn
        .prepare("SELECT * FROM employees ORDER BY created_at DESC")
        .map_err(|e| format!("Failed to prepare: {e}"))?;

    let mut rows = stmt
        .query([])
        .map_err(|e| format!("Failed to query: {e}"))?;

    while let Some(row) = rows.next().map_err(|e| format!("Row error: {e}"))? {
        let line = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            csv_field(row.get::<_, Option<String>>(0).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(1).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(2).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(3).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(4).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(5).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(6).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(7).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(8).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(9).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(10).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(11).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(12).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(13).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(14).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(15).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(16).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(17).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(18).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(19).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(20).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(21).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(22).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(23).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(24).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(25).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(26).unwrap_or(None)),
            csv_field(row.get::<_, Option<String>>(27).unwrap_or(None)),
        );
        writeln!(writer, "{line}").map_err(|e| format!("Failed to write row: {e}"))?;
    }

    writer
        .flush()
        .map_err(|e| format!("Failed to flush: {e}"))?;
    Ok(export_path.to_string_lossy().to_string())
}

fn csv_field(value: Option<String>) -> String {
    match value {
        None => String::new(),
        Some(v) => {
            if v.contains(',') || v.contains('"') || v.contains('\n') {
                format!("\"{}\"", v.replace('"', "\"\""))
            } else {
                v
            }
        }
    }
}
