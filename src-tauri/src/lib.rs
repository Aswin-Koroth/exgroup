mod db;
mod employees;
mod files;
mod state;

use std::{path::PathBuf, sync::OnceLock};

use employees::commands;
use state::AppState;
use tauri::Manager;

pub static APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            println!("Initializing database...");
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            APP_DATA_DIR
                .set(app_data_dir)
                .expect("Failed to set APP_DATA_DIR");

            match db::init_db() {
                Ok(conn) => {
                    println!("Database initialized successfully");

                    let app_state = AppState::new();
                    *app_state.db.lock().unwrap() = Some(conn);
                    app.manage(app_state);

                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {e}");
                    Err(e.into())
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_all_employees,
            commands::create_employee,
            commands::update_employee,
            commands::delete_employee,
            commands::get_db_info,
            commands::create_database_backup,
            commands::delete_employee_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
