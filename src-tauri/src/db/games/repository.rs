use rusqlite::{params, Connection};

use crate::services::versus_metrics::MetricGameRow;

use super::model::{Game, GameListItem};

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

    Ok(items)
}

fn parse_pattern_tags_csv(raw: Option<String>) -> Vec<String> {
    match raw {
        None => Vec::new(),
        Some(s) if s.is_empty() => Vec::new(),
        Some(s) => s.split(',').map(str::trim).filter(|t| !t.is_empty()).map(String::from).collect(),
    }
}

pub fn get_game_list_items(
    conn: &Connection,
    username: &str,
    user_id: &str,
    limit: u32,
) -> rusqlite::Result<Vec<GameListItem>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            g.id,
            g.username,
            g.platform,
            g.rated,
            g.speed,
            g.time_control,
            g.created_at,
            g.player_name,
            g.player_id,
            g.opponent_name,
            g.opponent_id,
            g.white_name,
            g.white_id,
            g.black_name,
            g.black_id,
            g.white_rating,
            g.black_rating,
            g.player_rating,
            g.opponent_rating,
            g.winner,
            g.player_color,
            g.player_result,
            g.opening_eco,
            g.opening_name,
            g.moves,
            g.last_fen,
            g.pgn,
            ga.accuracy,
            ga.avg_centipawn_loss,
            (
                SELECT GROUP_CONCAT(tag, ',')
                FROM game_pattern_tags t
                WHERE t.game_id = g.id AND t.user_id = ?3
            )
        FROM games g
        LEFT JOIN game_analyses ga
            ON ga.game_id = g.id
            AND ga.user_id = ?3
            AND ga.status = 'done'
        WHERE g.username = ?1
        ORDER BY g.created_at DESC
        LIMIT ?2
        ",
    )?;

    let rows = stmt.query_map(params![username, limit, user_id], |row| {
        let game = Game {
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
        };
        let tags_raw: Option<String> = row.get(29)?;
        Ok(GameListItem {
            base: game,
            analysis_accuracy: row.get(27)?,
            analysis_acpl: row.get(28)?,
            pattern_tags: parse_pattern_tags_csv(tags_raw),
        })
    })?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn delete_games_by_username(conn: &Connection, username: &str) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM games WHERE username = ?1", [username])
}

pub fn get_game_by_id(conn: &Connection, id: &str) -> rusqlite::Result<Option<Game>> {
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
        WHERE id = ?1
        LIMIT 1
        ",
    )?;

    let row = stmt.query_row([id], |row| {
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
    });

    match row {
        Ok(g) => Ok(Some(g)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Last `limit` Lichess rated games at `speed` with completed analysis rows (Versus «you» pentagon slice).
pub fn versus_self_analyzed_metrics(
    conn: &Connection,
    username: &str,
    user_id: &str,
    speed: &str,
    limit: u32,
) -> rusqlite::Result<Vec<MetricGameRow>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            ga.accuracy,
            ga.avg_centipawn_loss,
            ga.max_advantage_cp,
            ga.blunders,
            g.player_result,
            EXISTS (
                SELECT 1 FROM game_pattern_tags t
                WHERE t.game_id = g.id AND t.user_id = ?3 AND t.tag = 'opening_blunder'
            ),
            EXISTS (
                SELECT 1 FROM game_pattern_tags t
                WHERE t.game_id = g.id AND t.user_id = ?3 AND t.tag = 'endgame_blunder'
            )
        FROM games g
        INNER JOIN game_analyses ga
            ON ga.game_id = g.id AND ga.user_id = ?3 AND ga.status = 'done'
        WHERE g.username = ?1
          AND g.platform = 'Lichess'
          AND g.rated = 1
          AND g.speed = ?2
          AND ga.accuracy IS NOT NULL
        ORDER BY g.created_at DESC
        LIMIT ?4
        ",
    )?;

    let rows = stmt.query_map(
        rusqlite::params![username, speed, user_id, limit],
        |row| {
            Ok(MetricGameRow {
                accuracy_raw: row.get(0)?,
                avg_centipawn_loss: row.get(1)?,
                max_adv: row.get(2)?,
                blunders: row.get(3)?,
                player_result: row.get(4)?,
                opening_blunder: row.get::<_, i64>(5)? != 0,
                endgame_blunder: row.get::<_, i64>(6)? != 0,
            })
        },
    )?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

#[derive(Debug, Clone)]
pub struct OpeningAggregateRow {
    pub opening_name: String,
    pub wins: i64,
    pub losses: i64,
    pub draws: i64,
    pub total: i64,
}

/// Aggregate opening results over the latest `recent_limit` games (before GROUP BY).
pub fn versus_opening_stats_recent(
    conn: &Connection,
    username: &str,
    speed: &str,
    recent_limit: u32,
) -> rusqlite::Result<Vec<OpeningAggregateRow>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            opening_name,
            SUM(CASE WHEN player_result = 'win' THEN 1 ELSE 0 END),
            SUM(CASE WHEN player_result = 'loss' THEN 1 ELSE 0 END),
            SUM(CASE WHEN player_result = 'draw' THEN 1 ELSE 0 END),
            COUNT(*)
        FROM (
            SELECT opening_name, player_result
            FROM games
            WHERE username = ?1
              AND platform = 'Lichess'
              AND rated = 1
              AND speed = ?2
              AND opening_name IS NOT NULL
              AND TRIM(opening_name) <> ''
            ORDER BY created_at DESC
            LIMIT ?3
        )
        GROUP BY opening_name
        ",
    )?;

    let rows = stmt.query_map(params![username, speed, recent_limit], |row| {
        Ok(OpeningAggregateRow {
            opening_name: row.get(0)?,
            wins: row.get::<_, i64>(1)?,
            losses: row.get::<_, i64>(2)?,
            draws: row.get::<_, i64>(3)?,
            total: row.get::<_, i64>(4)?,
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
