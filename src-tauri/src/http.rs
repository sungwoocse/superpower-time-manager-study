use std::sync::{Arc, Mutex};
use std::thread;

use rusqlite::Connection;
use tiny_http::{Header, Method, Request, Response, Server};

use crate::db::insert_usage_event;
use crate::models::UsageEvent;

const INGEST_ADDRESS: &str = "127.0.0.1:51247";

pub fn is_usage_events_path(path: &str) -> bool {
    path.split('?').next() == Some("/usage-events")
}

pub fn start_ingest_server(conn: Arc<Mutex<Connection>>) -> Result<(), String> {
    let server = Server::http(INGEST_ADDRESS)
        .map_err(|error| format!("failed to bind ingest server on {INGEST_ADDRESS}: {error}"))?;

    thread::spawn(move || {
        for request in server.incoming_requests() {
            handle_request(request, &conn);
        }
    });

    Ok(())
}

fn handle_request(mut request: Request, conn: &Arc<Mutex<Connection>>) {
    let response = if request.method() == &Method::Options {
        Response::from_string("").with_status_code(204)
    } else if request.method() != &Method::Post || !is_usage_events_path(request.url()) {
        Response::from_string("not found").with_status_code(404)
    } else {
        handle_usage_event_post(&mut request, conn)
    };

    let _ = request.respond(with_cors(response));
}

fn handle_usage_event_post(
    request: &mut Request,
    conn: &Arc<Mutex<Connection>>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut body = String::new();
    if request.as_reader().read_to_string(&mut body).is_err() {
        return Response::from_string("bad request").with_status_code(400);
    }

    let Ok(event) = serde_json::from_str::<UsageEvent>(&body) else {
        return Response::from_string("bad request").with_status_code(400);
    };

    let Ok(conn) = conn.lock() else {
        return Response::from_string("bad request").with_status_code(400);
    };

    match insert_usage_event(
        &conn,
        &event.url,
        &event.domain,
        &event.title,
        &event.browser,
        &event.event_type,
        &event.timestamp,
    ) {
        Ok(()) => Response::from_string("ok").with_status_code(202),
        Err(_) => Response::from_string("bad request").with_status_code(400),
    }
}

fn with_cors(response: Response<std::io::Cursor<Vec<u8>>>) -> Response<std::io::Cursor<Vec<u8>>> {
    response
        .with_header(header("Access-Control-Allow-Origin", "*"))
        .with_header(header("Access-Control-Allow-Methods", "POST, OPTIONS"))
        .with_header(header("Access-Control-Allow-Headers", "content-type"))
}

fn header(name: &'static str, value: &'static str) -> Header {
    Header::from_bytes(name, value).expect("static CORS header should be valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_usage_events_path() {
        assert!(is_usage_events_path("/usage-events"));
        assert!(!is_usage_events_path("/"));
    }
}
