pub mod commands;
pub mod db;
pub mod models;

use std::sync::Mutex;

use commands::{ingest_usage_event, AppState};
use rusqlite::Connection;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = Connection::open("time_manager.sqlite3").expect("failed to open database");
    db::init_db(&conn).expect("failed to initialize database");

    tauri::Builder::default()
        .manage(AppState {
            conn: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![ingest_usage_event])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
