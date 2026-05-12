use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        create table if not exists usage_events (
            id integer primary key autoincrement,
            url text not null,
            domain text not null,
            title text not null,
            browser text not null,
            event_type text not null,
            timestamp text not null,
            created_at text not null default current_timestamp
        );

        create table if not exists usage_sessions (
            id integer primary key autoincrement,
            domain text not null,
            title text not null,
            browser text not null,
            classification text not null,
            started_at text not null,
            ended_at text
        );

        create table if not exists domain_rules (
            id integer primary key autoincrement,
            domain text not null unique,
            classification text not null
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
    fn initializes_default_domain_rules() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let count: i64 = conn
            .query_row("select count(*) from domain_rules", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 4);
    }
}
