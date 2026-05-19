//! Home profile chart: user pentagon from recent rated DB games vs embedded benchmark for the same speed bucket.
use serde::Serialize;
use tauri::AppHandle;

use crate::db::connection::get_conn;
use crate::db::users::repository as users_repository;
use crate::services::benchmarks::{self};
use crate::services::versus_metrics::{MetricGameRow, PentagonDto, pentagon_from_metrics};

const THIRTY_DAYS_MS: i64 = 30 * 24 * 60 * 60 * 1000; // Rolling month matches typical current-form expectations in UI.

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileChartResponse {
    pub speed: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_used: Option<i64>,
    pub bucket_label: String,
    pub benchmark: PentagonDto,
    pub benchmark_acpl_avg: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<PentagonDto>,
    pub sample_size: u32,
}

fn now_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn rating_for_speed(user: &crate::db::users::model::User, speed: &str) -> Option<i64> {
    match speed {
        "bullet" => user.bullet_rating,
        "blitz" => user.blitz_rating,
        "rapid" => user.rapid_rating,
        _ => None,
    }
}

/// Builds benchmark + optional player pentagon for bullet/blitz/rapid from analyzed games in the last 30 days.
pub fn get_player_profile_chart(app: &AppHandle, speed: String) -> Result<PlayerProfileChartResponse, String> {
    let speed_lc = speed.to_lowercase();
    if !matches!(speed_lc.as_str(), "bullet" | "blitz" | "rapid") {
        return Err("Invalid speed".into());
    }

    let conn = get_conn(app)?;
    let user = users_repository::get_active_user(&conn)?
        .ok_or("Active user not found")?;

    let rating_used = rating_for_speed(&user, &speed_lc);
    let rating_for_bucket = rating_used.unwrap_or(1500);
    let bucket_key = benchmarks::bucket_key_for_rating(rating_for_bucket);
    let (bench_pentagon, bucket_label) = benchmarks::pentagon_and_label(&bucket_key)
        .ok_or_else(|| format!("Unknown benchmark bucket: {}", bucket_key))?;
    let benchmark = PentagonDto::from(bench_pentagon);
    let benchmark_acpl_avg = benchmarks::acpl_avg_for_rating(rating_for_bucket);

    // Exclude older games so the chart reflects recent performance, not lifetime average mixed into one polygon.
    let cutoff = now_millis() - THIRTY_DAYS_MS;

    let mut stmt = conn
        .prepare(
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
          AND g.created_at >= ?4
          AND ga.accuracy IS NOT NULL
        ",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(
            rusqlite::params![
                user.username.as_str(),
                speed_lc.as_str(),
                user.id.as_str(),
                cutoff
            ],
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
        )
        .map_err(|e| e.to_string())?;

    let mut games: Vec<MetricGameRow> = Vec::new();
    for r in rows {
        games.push(r.map_err(|e| e.to_string())?);
    }

    let sample_size = games.len() as u32;

    let player = pentagon_from_metrics(&games, &benchmark);

    Ok(PlayerProfileChartResponse {
        speed: speed_lc,
        rating_used,
        bucket_label,
        benchmark,
        benchmark_acpl_avg,
        player,
        sample_size,
    })
}
