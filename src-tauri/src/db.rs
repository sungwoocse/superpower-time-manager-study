use chrono::DateTime;
use rusqlite::{params, Connection, Result as SqlResult};
use url::Url;

use crate::models::DomainRule;

pub fn init_db(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "
        create table if not exists usage_events (
            id integer primary key autoincrement,
            url text not null,
            domain text not null,
            title text not null,
            browser text not null check (browser in ('chrome', 'edge', 'unknown')),
            event_type text not null check (event_type in ('focus', 'blur', 'idle', 'active')),
            timestamp text not null,
            created_at text not null default current_timestamp
        );

        create table if not exists usage_sessions (
            id integer primary key autoincrement,
            domain text not null,
            title text not null,
            browser text not null,
            classification text not null check (classification in ('productive', 'unproductive', 'neutral')),
            started_at text not null,
            ended_at text
        );

        create table if not exists domain_rules (
            id integer primary key autoincrement,
            domain text not null collate nocase unique,
            classification text not null check (classification in ('productive', 'unproductive', 'neutral'))
        );
        ",
    )?;

    for (domain, classification) in [
        ("chatgpt.com", "productive"),
        ("chat.openai.com", "productive"),
        ("youtube.com", "unproductive"),
        ("instagram.com", "unproductive"),
    ] {
        conn.execute(
            "insert or ignore into domain_rules (domain, classification) values (?1, ?2)",
            params![domain, classification],
        )?;
    }

    Ok(())
}

pub fn list_domain_rules(conn: &Connection) -> SqlResult<Vec<DomainRule>> {
    let mut statement =
        conn.prepare("select domain, classification from domain_rules order by domain asc")?;
    let rules = statement.query_map([], |row| {
        Ok(DomainRule {
            domain: row.get(0)?,
            classification: row.get(1)?,
        })
    })?;

    rules.collect()
}

pub fn insert_usage_event(
    conn: &Connection,
    url: &str,
    domain: &str,
    title: &str,
    browser: &str,
    event_type: &str,
    timestamp: &str,
) -> Result<(), String> {
    let event = validate_usage_event(url, domain, title, browser, event_type, timestamp)?;

    conn.execute(
        "
        insert into usage_events
            (url, domain, title, browser, event_type, timestamp)
        values
            (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        params![
            event.url,
            event.domain,
            event.title,
            event.browser,
            event.event_type,
            event.timestamp
        ],
    )
    .map_err(|error| error.to_string())?;

    Ok(())
}

struct ValidatedUsageEvent {
    url: String,
    domain: String,
    title: String,
    browser: String,
    event_type: String,
    timestamp: String,
}

fn validate_usage_event(
    url: &str,
    domain: &str,
    title: &str,
    browser: &str,
    event_type: &str,
    timestamp: &str,
) -> Result<ValidatedUsageEvent, String> {
    let url = url.trim();
    let domain = normalize_domain(domain);
    let title = title.trim();
    let browser = browser.trim();
    let event_type = event_type.trim();
    let timestamp = timestamp.trim();

    if url.is_empty() {
        return Err("url is required".to_string());
    }
    if domain.is_empty() {
        return Err("domain is required".to_string());
    }
    if title.is_empty() {
        return Err("title is required".to_string());
    }
    if timestamp.is_empty() {
        return Err("timestamp is required".to_string());
    }

    let parsed_url = Url::parse(url).map_err(|_| "url must be a valid URL".to_string())?;
    if !matches!(parsed_url.scheme(), "http" | "https") {
        return Err("url scheme must be http or https".to_string());
    }

    let url_domain = parsed_url
        .host_str()
        .map(normalize_domain)
        .ok_or_else(|| "url must include a host".to_string())?;

    if domain.contains("://")
        || domain.contains('/')
        || domain.chars().any(char::is_whitespace)
        || !domain.contains('.')
    {
        return Err("domain must be a host name like example.com".to_string());
    }

    if domain != url_domain {
        return Err("domain must match url host".to_string());
    }

    DateTime::parse_from_rfc3339(timestamp).map_err(|_| "timestamp must be RFC3339".to_string())?;

    Ok(ValidatedUsageEvent {
        url: url.to_string(),
        domain,
        title: title.to_string(),
        browser: browser.to_string(),
        event_type: event_type.to_string(),
        timestamp: timestamp.to_string(),
    })
}

fn normalize_domain(domain: &str) -> String {
    let domain = domain.trim().to_lowercase();
    domain.strip_prefix("www.").unwrap_or(&domain).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_exact_default_domain_rules() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let mut stmt = conn
            .prepare("select domain, classification from domain_rules order by domain")
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .unwrap()
            .collect::<SqlResult<Vec<_>>>()
            .unwrap();

        assert_eq!(
            rows,
            vec![
                ("chat.openai.com".to_string(), "productive".to_string()),
                ("chatgpt.com".to_string(), "productive".to_string()),
                ("instagram.com".to_string(), "unproductive".to_string()),
                ("youtube.com".to_string(), "unproductive".to_string()),
            ]
        );
    }

    #[test]
    fn init_db_is_idempotent_for_default_domain_rules() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        init_db(&conn).unwrap();

        let count: i64 = conn
            .query_row("select count(*) from domain_rules", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 4);
    }

    #[test]
    fn lists_domain_rules_sorted_by_domain_after_init() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let rules = list_domain_rules(&conn).unwrap();

        let actual = rules
            .iter()
            .map(|rule| (rule.domain.as_str(), rule.classification.as_str()))
            .collect::<Vec<_>>();

        assert_eq!(
            actual,
            vec![
                ("chat.openai.com", "productive"),
                ("chatgpt.com", "productive"),
                ("instagram.com", "unproductive"),
                ("youtube.com", "unproductive"),
            ]
        );
    }

    #[test]
    fn stores_usage_event() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        insert_usage_event(
            &conn,
            "https://youtube.com/watch?v=abc",
            "youtube.com",
            "Video",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        )
        .unwrap();

        let count: i64 = conn
            .query_row("select count(*) from usage_events", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn stores_trimmed_usage_event_with_lowercase_domain() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        insert_usage_event(
            &conn,
            " https://Example.COM/docs ",
            " Example.COM ",
            " Example Docs ",
            " chrome ",
            " active ",
            " 2026-05-12T08:00:00Z ",
        )
        .unwrap();

        let row: (String, String, String, String, String, String) = conn
            .query_row(
                "select url, domain, title, browser, event_type, timestamp from usage_events",
                [],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                    ))
                },
            )
            .unwrap();

        assert_eq!(
            row,
            (
                "https://Example.COM/docs".to_string(),
                "example.com".to_string(),
                "Example Docs".to_string(),
                "chrome".to_string(),
                "active".to_string(),
                "2026-05-12T08:00:00Z".to_string(),
            )
        );
    }

    #[test]
    fn rejects_invalid_usage_event_timestamp() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = insert_usage_event(
            &conn,
            "https://example.com",
            "example.com",
            "Example",
            "chrome",
            "active",
            "2026-05-12 08:00:00",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_usage_event_url_string() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = insert_usage_event(
            &conn,
            "not a url",
            "example.com",
            "Example",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_http_usage_event_url_scheme() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = insert_usage_event(
            &conn,
            "chrome://extensions",
            "extensions",
            "Extensions",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_mismatched_usage_event_url_and_domain() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = insert_usage_event(
            &conn,
            "https://youtube.com/watch?v=abc",
            "example.com",
            "Video",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        );

        assert!(result.is_err());
    }

    #[test]
    fn stores_www_usage_event_url_as_root_domain() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        insert_usage_event(
            &conn,
            "https://www.youtube.com/watch?v=abc",
            "youtube.com",
            "Video",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        )
        .unwrap();

        let domain: String = conn
            .query_row("select domain from usage_events", [], |row| row.get(0))
            .unwrap();

        assert_eq!(domain, "youtube.com");
    }

    #[test]
    fn rejects_empty_usage_event_url_and_title() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let empty_url = insert_usage_event(
            &conn,
            " ",
            "example.com",
            "Example",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        );
        let empty_title = insert_usage_event(
            &conn,
            "https://example.com",
            "example.com",
            " ",
            "chrome",
            "active",
            "2026-05-12T08:00:00Z",
        );

        assert!(empty_url.is_err());
        assert!(empty_title.is_err());
    }

    #[test]
    fn rejects_invalid_usage_event_domain_shape() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        for domain in [
            "https://example.com",
            "example.com/path",
            "example com",
            "localhost",
        ] {
            let result = insert_usage_event(
                &conn,
                "https://example.com",
                domain,
                "Example",
                "chrome",
                "active",
                "2026-05-12T08:00:00Z",
            );

            assert!(result.is_err(), "domain should be rejected: {domain}");
        }
    }

    #[test]
    fn rejects_invalid_domain_rule_classification() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = conn.execute(
            "insert into domain_rules (domain, classification) values ('example.com', 'invalid')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_case_insensitive_duplicate_domain_rule() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = conn.execute(
            "insert into domain_rules (domain, classification) values ('CHATGPT.COM', 'productive')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_usage_event_browser() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = conn.execute(
            "insert into usage_events (url, domain, title, browser, event_type, timestamp) values ('https://example.com', 'example.com', 'Example', 'firefox', 'focus', '2026-05-12T00:00:00Z')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_usage_event_type() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let result = conn.execute(
            "insert into usage_events (url, domain, title, browser, event_type, timestamp) values ('https://example.com', 'example.com', 'Example', 'chrome', 'navigate', '2026-05-12T00:00:00Z')",
            [],
        );

        assert!(result.is_err());
    }
}
