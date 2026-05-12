use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use rusqlite::Connection;
use tiny_http::{Header, Method, Request, Response, Server};

use crate::db::insert_usage_event;
use crate::models::UsageEvent;

const INGEST_ADDRESS: &str = "127.0.0.1:51247";
const MAX_INGEST_BODY_BYTES: usize = 16 * 1024;
// MVP-only shared secret. Replace this with a per-install generated token.
const INGEST_TOKEN: &str = "superpower-time-manager-dev-token";

#[derive(Debug, PartialEq, Eq)]
enum BodyReadError {
    Invalid,
    ReadFailed,
    TooLarge,
}

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
    let origin = header_value(request.headers(), "origin").map(str::to_owned);
    let response = if request.method() == &Method::Options {
        Response::from_string("").with_status_code(204)
    } else if request.method() != &Method::Post || !is_usage_events_path(request.url()) {
        Response::from_string("not found").with_status_code(404)
    } else if !has_valid_ingest_token(request.headers()) {
        Response::from_string("unauthorized").with_status_code(401)
    } else {
        handle_usage_event_post(&mut request, conn)
    };

    let _ = request.respond(with_cors(response, origin.as_deref()));
}

fn handle_usage_event_post(
    request: &mut Request,
    conn: &Arc<Mutex<Connection>>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let body_length = request.body_length();
    let body = match read_limited_body(request.as_reader(), body_length) {
        Ok(body) => body,
        Err(BodyReadError::TooLarge) => {
            return Response::from_string("payload too large").with_status_code(413)
        }
        Err(BodyReadError::Invalid | BodyReadError::ReadFailed) => {
            return Response::from_string("bad request").with_status_code(400)
        }
    };

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

fn has_valid_ingest_token(headers: &[Header]) -> bool {
    header_value(headers, "x-time-manager-token") == Some(INGEST_TOKEN)
}

fn header_value<'a>(headers: &'a [Header], name: &'static str) -> Option<&'a str> {
    headers
        .iter()
        .find(|header| header.field.equiv(name))
        .map(|header| header.value.as_str())
}

fn read_limited_body<R: Read>(
    reader: R,
    body_length: Option<usize>,
) -> Result<String, BodyReadError> {
    if matches!(body_length, Some(length) if length > MAX_INGEST_BODY_BYTES) {
        return Err(BodyReadError::TooLarge);
    }

    let mut limited_reader = reader.take((MAX_INGEST_BODY_BYTES + 1) as u64);
    let mut body = Vec::new();
    limited_reader
        .read_to_end(&mut body)
        .map_err(|_| BodyReadError::ReadFailed)?;

    if body.len() > MAX_INGEST_BODY_BYTES {
        return Err(BodyReadError::TooLarge);
    }

    String::from_utf8(body).map_err(|_| BodyReadError::Invalid)
}

fn allowed_cors_origin(origin: Option<&str>) -> Option<&str> {
    origin.filter(|origin| origin.starts_with("chrome-extension://"))
}

fn with_cors(
    response: Response<std::io::Cursor<Vec<u8>>>,
    origin: Option<&str>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let response = response
        .with_header(header("Access-Control-Allow-Methods", "POST, OPTIONS"))
        .with_header(header(
            "Access-Control-Allow-Headers",
            "content-type, x-time-manager-token",
        ));

    if let Some(origin) = allowed_cors_origin(origin) {
        return response
            .with_header(header("Vary", "Origin"))
            .with_header(header_value_owned("Access-Control-Allow-Origin", origin));
    }

    response
}

fn header(name: &'static str, value: &'static str) -> Header {
    header_value_owned(name, value)
}

fn header_value_owned(name: &'static str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("HTTP header should be valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn accepts_only_usage_events_path() {
        assert!(is_usage_events_path("/usage-events"));
        assert!(!is_usage_events_path("/"));
    }

    #[test]
    fn accepts_only_matching_ingest_token() {
        let valid_headers = vec![header("x-time-manager-token", INGEST_TOKEN)];
        assert!(has_valid_ingest_token(&valid_headers));

        let invalid_headers = vec![header("x-time-manager-token", "wrong-token")];
        assert!(!has_valid_ingest_token(&invalid_headers));

        let missing_headers = vec![header("content-type", "application/json")];
        assert!(!has_valid_ingest_token(&missing_headers));
    }

    #[test]
    fn allows_only_chrome_extension_cors_origins() {
        assert_eq!(
            allowed_cors_origin(Some("chrome-extension://abcdefghijk")),
            Some("chrome-extension://abcdefghijk")
        );
        assert_eq!(allowed_cors_origin(Some("https://example.com")), None);
        assert_eq!(allowed_cors_origin(None), None);
    }

    #[test]
    fn cors_headers_do_not_allow_arbitrary_web_origins() {
        let response = with_cors(Response::from_string("ok"), Some("https://example.com"));
        assert!(response
            .headers()
            .iter()
            .all(|header| !header.field.equiv("Access-Control-Allow-Origin")));

        let response = with_cors(
            Response::from_string("ok"),
            Some("chrome-extension://abcdefghijk"),
        );
        let origin = response
            .headers()
            .iter()
            .find(|header| header.field.equiv("Access-Control-Allow-Origin"))
            .map(|header| header.value.as_str());
        assert_eq!(origin, Some("chrome-extension://abcdefghijk"));
    }

    #[test]
    fn rejects_oversized_declared_body_before_reading() {
        let body = "{}";
        let result = read_limited_body(Cursor::new(body), Some(MAX_INGEST_BODY_BYTES + 1));

        assert_eq!(result, Err(BodyReadError::TooLarge));
    }

    #[test]
    fn rejects_oversized_body_without_declared_length() {
        let body = "x".repeat(MAX_INGEST_BODY_BYTES + 1);
        let result = read_limited_body(Cursor::new(body), None);

        assert_eq!(result, Err(BodyReadError::TooLarge));
    }

    #[test]
    fn accepts_body_within_limit() {
        let result = read_limited_body(Cursor::new("{\"ok\":true}"), None);

        assert_eq!(result, Ok("{\"ok\":true}".to_string()));
    }
}
