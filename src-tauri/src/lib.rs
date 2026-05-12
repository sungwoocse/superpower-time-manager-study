pub mod commands;
pub mod db;
pub mod http;
pub mod models;
pub mod token;

use std::sync::{Arc, Mutex};

use commands::{app_status, ingest_usage_event, AppState};
use rusqlite::Connection;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ingest_usage_event, app_status])
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
            let ingest_server_error = Arc::new(Mutex::new(None));
            {
                let conn = conn
                    .lock()
                    .map_err(|_| std::io::Error::other("database lock failed"))?;
                db::init_db(&conn)?;
            }

            let ingest_token = token::load_or_create_ingest_token(&app_data_dir)?;
            if let Err(error) = http::start_ingest_server(conn.clone(), ingest_token) {
                log::error!("{error}");
                if let Ok(mut current_error) = ingest_server_error.lock() {
                    *current_error = Some(error);
                }
            }

            app.manage(AppState {
                conn,
                ingest_server_error,
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
