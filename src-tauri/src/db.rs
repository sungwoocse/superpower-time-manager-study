use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
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
            .collect::<Result<Vec<_>>>()
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
