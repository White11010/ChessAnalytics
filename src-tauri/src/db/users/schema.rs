use rusqlite::Connection;

pub fn init_users_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            username TEXT NOT NULL,

            bullet_rating INTEGER,
            bullet_games INTEGER,

            blitz_rating INTEGER,
            blitz_games INTEGER,

            rapid_rating INTEGER,
            rapid_games INTEGER,

            classical_rating INTEGER,
            classical_games INTEGER,

            is_active INTEGER NOT NULL DEFAULT 0,

            UNIQUE(platform, username)
        );

        CREATE INDEX IF NOT EXISTS idx_users_active
        ON users(is_active);
        ",
    )
    .unwrap();

    ensure_users_sync_columns(conn);
}

/// Idempotent migrations for existing DB files (no migration framework in app).
pub fn ensure_users_sync_columns(conn: &Connection) {
    let mut cols: Vec<String> = conn
        .prepare("PRAGMA table_info(users)")
        .and_then(|mut s| {
            s.query_map([], |row| row.get::<_, String>(1))
                .and_then(|rows| rows.collect::<Result<Vec<_>, _>>())
        })
        .unwrap_or_default();

    cols.sort();
    cols.dedup();

    let has = |name: &str| cols.iter().any(|c| c == name);

    if !has("lichess_since_cursor_ms") {
        let _ = conn.execute(
            "ALTER TABLE users ADD COLUMN lichess_since_cursor_ms INTEGER",
            [],
        );
    }
    if !has("last_sync_completed_at_ms") {
        let _ = conn.execute(
            "ALTER TABLE users ADD COLUMN last_sync_completed_at_ms INTEGER",
            [],
        );
    }
}
