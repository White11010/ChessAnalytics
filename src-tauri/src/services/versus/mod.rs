//! Versus screen: fetches opponent NDJSON, runs transient Stockfish on a capped subset, compares pentagons to local DB stats.

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use tauri::{AppHandle, Emitter};

use crate::clients::lichess;
use crate::db::connection::get_conn;
use crate::db::games::model::Game;
use crate::db::users::repository as users_repo;
use crate::parsers::lichess_games;

mod compare;
mod game_plan;
mod openings;
mod progress;
mod side_summary;
mod speed;
pub mod types;

pub use types::*;

use compare::{prepare_versus_speed_slice, versus_finish_speed_slice};
use progress::emit_prog;
use speed::filter_opponent_games_for_speed;

pub(crate) static VERSUS_CANCEL: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub(crate) const LICHESS_FETCH_MAX: u32 = 500;
pub(crate) const OPP_ANALYZE_MAX: usize = 100;
pub(crate) const SELF_METRICS_LIMIT: u32 = 100;
pub(crate) const SELF_OPENINGS_RECENT_LIMIT: u32 = 2000;
pub(crate) const ANALYSIS_DEPTH: u8 = 8;
pub(crate) const MIN_OPENING_GAMES_SHOW: i64 = 3;
pub(crate) const GP_TIER1_MIN_GAMES: i64 = 3;
pub(crate) const GP_TIER23_MIN_GAMES: i64 = 5;
pub(crate) const GP_PLAN_ENTRIES_PER_LIST: usize = 2;
pub(crate) const VERSUS_OPENINGS_PER_COLOR: usize = 2;

/// Sets a flag checked between opponent analyses so long compares can abort without killing the whole app.
pub fn cancel_versus_compare() {
    VERSUS_CANCEL.store(true, Ordering::SeqCst);
}

/// Fetches up to 500 opponent games, builds bullet/blitz/rapid slices with self DB stats + transient opponent analysis.
pub async fn versus_compare(
    app: AppHandle,
    opponent_username_raw: String,
) -> Result<VersusCompareResponse, String> {
    VERSUS_CANCEL.store(false, Ordering::SeqCst);

    let opponent_display = opponent_username_raw.trim().to_string();
    let opponent_slug = opponent_display.to_lowercase();
    if opponent_slug.is_empty() {
        return Err("Opponent username is empty".into());
    }

    let conn = get_conn(&app)?;
    let user = users_repo::get_active_user(&conn)?.ok_or("Active user not found")?;

    if opponent_slug.eq_ignore_ascii_case(user.username.trim()) {
        return Err("Choose an opponent other than yourself".into());
    }

    emit_prog(&app, "fetch_opponent", 0, 1);
    let ndjson = lichess::fetch_games(&app, &opponent_slug, None, Some(LICHESS_FETCH_MAX)).await?;
    emit_prog(&app, "fetch_opponent", 1, 1);

    let parsed_games = lichess_games::parse_ndjson(&opponent_slug, &ndjson);
    let opponent_games_in_api_sample = parsed_games.len() as u32;

    let mut distinct_speeds_rated_with_moves: Vec<String> = parsed_games
        .iter()
        .filter(|g| {
            g.rated
                && g.moves
                    .as_ref()
                    .map(|m| !m.trim().is_empty())
                    .unwrap_or(false)
        })
        .map(|g| g.speed.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    distinct_speeds_rated_with_moves.sort();

    let filtered_by_speed: [Vec<Game>; 3] = [
        filter_opponent_games_for_speed(&parsed_games, "bullet"),
        filter_opponent_games_for_speed(&parsed_games, "blitz"),
        filter_opponent_games_for_speed(&parsed_games, "rapid"),
    ];
    let analyze_total: u32 = filtered_by_speed
        .iter()
        .map(|v| (v.len().min(OPP_ANALYZE_MAX)) as u32)
        .sum::<u32>()
        .max(1);

    let mut prog_current = 0u32;

    let bullet_draft = prepare_versus_speed_slice(
        &conn,
        &user,
        opponent_display.clone(),
        "bullet",
        &filtered_by_speed[0],
        &distinct_speeds_rated_with_moves,
    )?;
    let bullet =
        versus_finish_speed_slice(&app, bullet_draft, &mut prog_current, analyze_total).await?;

    let blitz_draft = prepare_versus_speed_slice(
        &conn,
        &user,
        opponent_display.clone(),
        "blitz",
        &filtered_by_speed[1],
        &distinct_speeds_rated_with_moves,
    )?;
    let blitz =
        versus_finish_speed_slice(&app, blitz_draft, &mut prog_current, analyze_total).await?;

    let rapid_draft = prepare_versus_speed_slice(
        &conn,
        &user,
        opponent_display,
        "rapid",
        &filtered_by_speed[2],
        &distinct_speeds_rated_with_moves,
    )?;
    let rapid =
        versus_finish_speed_slice(&app, rapid_draft, &mut prog_current, analyze_total).await?;

    emit_prog(&app, "analyze_opponent", analyze_total, analyze_total);

    let _ = app.emit(
        "versus://progress",
        serde_json::json!({
            "phase": "done",
            "current": 1,
            "total": 1,
        }),
    );

    Ok(VersusCompareResponse {
        opponent_games_in_api_sample,
        slices: VersusSlices {
            bullet,
            blitz,
            rapid,
        },
    })
}
