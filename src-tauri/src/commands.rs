use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::db::insert_usage_event;
use crate::models::UsageEvent;

pub struct AppState {
    pub conn: Mutex<Connection>,
}

#[tauri::command]
pub fn ingest_usage_event(event: UsageEvent, state: State<'_, AppState>) -> Result<(), String> {
    if event.domain.trim().is_empty() || event.timestamp.trim().is_empty() {
        return Err("domain and timestamp are required".to_string());
    }

    let conn = state
        .conn
        .lock()
        .map_err(|_| "database lock failed".to_string())?;

    insert_usage_event(
        &conn,
        &event.url,
        &event.domain,
        &event.title,
        &event.browser,
        &event.event_type,
        &event.timestamp,
    )
    .map_err(|error| error.to_string())
}
