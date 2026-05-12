pub mod commands;
pub mod db;
pub mod http;
pub mod models;

use std::sync::{Arc, Mutex};

use commands::{ingest_usage_event, AppState};
use rusqlite::Connection;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ingest_usage_event])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join("time_manager.sqlite3");
            let conn = Arc::new(Mutex::new(Connection::open(db_path)?));
            {
                let conn = conn
                    .lock()
                    .map_err(|_| std::io::Error::other("database lock failed"))?;
                db::init_db(&conn)?;
            }

            http::start_ingest_server(conn.clone()).map_err(std::io::Error::other)?;

            app.manage(AppState { conn });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
