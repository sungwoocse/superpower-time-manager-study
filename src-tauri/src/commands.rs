use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use serde::Serialize;
use tauri::State;

use crate::db::insert_usage_event;
use crate::models::UsageEvent;

pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
    pub ingest_server_error: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatus {
    pub ingest_server_error: Option<String>,
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

#[tauri::command]
pub fn app_status(state: State<'_, AppState>) -> Result<AppStatus, String> {
    current_app_status(&state.ingest_server_error)
}

pub fn current_app_status(
    ingest_server_error: &Arc<Mutex<Option<String>>>,
) -> Result<AppStatus, String> {
    let ingest_server_error = ingest_server_error
        .lock()
        .map_err(|_| "app status lock failed".to_string())?
        .clone();

    Ok(AppStatus {
        ingest_server_error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_ingest_server_startup_error_in_app_status() {
        let ingest_server_error = Arc::new(Mutex::new(Some(
            "failed to bind ingest server on 127.0.0.1:51247".to_string(),
        )));

        let status = current_app_status(&ingest_server_error).unwrap();

        assert_eq!(
            status.ingest_server_error,
            Some("failed to bind ingest server on 127.0.0.1:51247".to_string())
        );
    }
}
