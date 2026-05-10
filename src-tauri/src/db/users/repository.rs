use rusqlite::{params, Connection, OptionalExtension};

use super::model::User;

pub fn upsert_user(conn: &Connection, user: &User) -> Result<(), String> {
    conn.execute(
        "
        INSERT INTO users (
            id,
            platform,
            username,

            bullet_rating,
            bullet_games,

            blitz_rating,
            blitz_games,

            rapid_rating,
            rapid_games,

            classical_rating,
            classical_games,

            is_active
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 0)

        ON CONFLICT(id) DO UPDATE SET
            platform = excluded.platform,
            username = excluded.username,

            bullet_rating = excluded.bullet_rating,
            bullet_games = excluded.bullet_games,

            blitz_rating = excluded.blitz_rating,
            blitz_games = excluded.blitz_games,

            rapid_rating = excluded.rapid_rating,
            rapid_games = excluded.rapid_games,

            classical_rating = excluded.classical_rating,
            classical_games = excluded.classical_games
        ",
        params![
            user.id,
            user.platform,
            user.username,
            user.bullet_rating,
            user.bullet_games,
            user.blitz_rating,
            user.blitz_games,
            user.rapid_rating,
            user.rapid_games,
            user.classical_rating,
            user.classical_games
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_active_user(conn: &Connection, user_id: &str) -> Result<(), String> {
    conn.execute("UPDATE users SET is_active = 0", [])
        .map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE users SET is_active = 1 WHERE id = ?1",
        params![user_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get_active_user(conn: &Connection) -> Result<Option<User>, String> {
    conn.query_row(
        "
        SELECT
            id,
            platform,
            username,

            bullet_rating,
            bullet_games,

            blitz_rating,
            blitz_games,

            rapid_rating,
            rapid_games,

            classical_rating,
            classical_games,

            lichess_since_cursor_ms,
            last_sync_completed_at_ms
        FROM users
        WHERE is_active = 1
        LIMIT 1
        ",
        [],
        |row| {
            Ok(User {
                id: row.get(0)?,
                platform: row.get(1)?,
                username: row.get(2)?,

                bullet_rating: row.get(3)?,
                bullet_games: row.get(4)?,

                blitz_rating: row.get(5)?,
                blitz_games: row.get(6)?,

                rapid_rating: row.get(7)?,
                rapid_games: row.get(8)?,

                classical_rating: row.get(9)?,
                classical_games: row.get(10)?,

                lichess_since_cursor_ms: row.get(11)?,
                last_sync_completed_at_ms: row.get(12)?,
            })
        },
    )
    .optional()
    .map_err(|e| e.to_string())
}

pub fn update_user_games_sync_metadata(
    conn: &Connection,
    user_id: &str,
    lichess_since_cursor_ms: Option<i64>,
    last_sync_completed_at_ms: i64,
) -> Result<(), String> {
    conn.execute(
        "
        UPDATE users SET
            lichess_since_cursor_ms = ?1,
            last_sync_completed_at_ms = ?2
        WHERE id = ?3
        ",
        params![lichess_since_cursor_ms, last_sync_completed_at_ms, user_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
