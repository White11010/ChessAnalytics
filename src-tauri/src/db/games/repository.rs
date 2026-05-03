use rusqlite::{params, Connection};

use super::model::Game;

pub fn upsert_game(conn: &Connection, game: &Game) -> rusqlite::Result<usize> {
    conn.execute(
        "
        INSERT INTO games (
            id,

            username,
            platform,

            rated,
            speed,
            time_control,
            created_at,

            player_name,
            player_id,

            opponent_name,
            opponent_id,

            white_name,
            white_id,

            black_name,
            black_id,

            white_rating,
            black_rating,

            player_rating,
            opponent_rating,

            winner,
            player_color,
            player_result,

            opening_eco,
            opening_name,

            moves,
            last_fen,
            pgn
        )
        VALUES (
            ?1, ?2, ?3,
            ?4, ?5, ?6, ?7,
            ?8, ?9,
            ?10, ?11,
            ?12, ?13,
            ?14, ?15,
            ?16, ?17,
            ?18, ?19,
            ?20, ?21, ?22,
            ?23, ?24,
            ?25, ?26, ?27
        )
        ON CONFLICT(id) DO UPDATE SET
            rated = excluded.rated,
            speed = excluded.speed,
            time_control = excluded.time_control,
            created_at = excluded.created_at,
            player_rating = excluded.player_rating,
            opponent_rating = excluded.opponent_rating,
            winner = excluded.winner,
            player_result = excluded.player_result,
            opening_eco = excluded.opening_eco,
            opening_name = excluded.opening_name,
            moves = excluded.moves,
            last_fen = excluded.last_fen,
            pgn = excluded.pgn
        ",
        params![
            game.id,
            game.username,
            game.platform,
            game.rated,
            game.speed,
            game.time_control,
            game.created_at,
            game.player_name,
            game.player_id,
            game.opponent_name,
            game.opponent_id,
            game.white_name,
            game.white_id,
            game.black_name,
            game.black_id,
            game.white_rating,
            game.black_rating,
            game.player_rating,
            game.opponent_rating,
            game.winner,
            game.player_color,
            game.player_result,
            game.opening_eco,
            game.opening_name,
            game.moves,
            game.last_fen,
            game.pgn
        ],
    )
}

pub fn get_games_by_username(
    conn: &Connection,
    username: &str,
    limit: u32,
) -> rusqlite::Result<Vec<Game>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,

            username,
            platform,

            rated,
            speed,
            time_control,
            created_at,

            player_name,
            player_id,

            opponent_name,
            opponent_id,

            white_name,
            white_id,

            black_name,
            black_id,

            white_rating,
            black_rating,

            player_rating,
            opponent_rating,

            winner,
            player_color,
            player_result,

            opening_eco,
            opening_name,

            moves,
            last_fen,
            pgn
        FROM games
        WHERE username = ?1
        ORDER BY created_at DESC
        LIMIT ?2
        ",
    )?;

    let rows = stmt.query_map(params![username, limit], |row| {
        Ok(Game {
            id: row.get(0)?,

            username: row.get(1)?,
            platform: row.get(2)?,

            rated: row.get(3)?,
            speed: row.get(4)?,
            time_control: row.get(5)?,
            created_at: row.get(6)?,

            player_name: row.get(7)?,
            player_id: row.get(8)?,

            opponent_name: row.get(9)?,
            opponent_id: row.get(10)?,

            white_name: row.get(11)?,
            white_id: row.get(12)?,

            black_name: row.get(13)?,
            black_id: row.get(14)?,

            white_rating: row.get(15)?,
            black_rating: row.get(16)?,

            player_rating: row.get(17)?,
            opponent_rating: row.get(18)?,

            winner: row.get(19)?,
            player_color: row.get(20)?,
            player_result: row.get(21)?,

            opening_eco: row.get(22)?,
            opening_name: row.get(23)?,

            moves: row.get(24)?,
            last_fen: row.get(25)?,
            pgn: row.get(26)?,
        })
    })?;

    let mut items = Vec::new();

    for row in rows {
        items.push(row?);
    }
    println!("{}", items.len());

    Ok(items)
}

pub fn delete_games_by_username(conn: &Connection, username: &str) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM games WHERE username = ?1", [username])
}
