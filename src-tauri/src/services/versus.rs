//! Versus screen: fetches opponent NDJSON, runs transient Stockfish on a capped subset, compares pentagons to local DB stats.
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::clients::lichess;
use crate::db::connection::get_conn;
use crate::db::games::model::Game;
use crate::db::games::repository::{self as games_repo, OpeningAggregateRow};
use crate::db::users::repository as users_repo;
use crate::parsers::lichess_games;
use crate::services::benchmarks;
use crate::services::game_analysis::service::{analyze_game_transient, TransientAnalysisResult};
use crate::services::versus_metrics::{
    MetricGameRow, PentagonDto, effective_game_accuracy, pentagon_from_metrics,
};

static VERSUS_CANCEL: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// Sets a flag checked between opponent analyses so long compares can abort without killing the whole app.
pub fn cancel_versus_compare() {
    VERSUS_CANCEL.store(true, Ordering::SeqCst);
}

const LICHESS_FETCH_MAX: u32 = 500; // Lichess export default cap: enough coverage without multi-page paging in v1.
const OPP_ANALYZE_MAX: usize = 100; // Bounds Stockfish work per speed so Versus stays responsive on mid-tier hardware.
const SELF_METRICS_LIMIT: u32 = 100; // Matches pentagon query elsewhere: ~100 recent games is stable for self slice.
const SELF_OPENINGS_RECENT_LIMIT: u32 = 2000; // Wide window for opening aggregates so rare lines still get sample counts.
const ANALYSIS_DEPTH: u8 = 8; // Shallow depth for transient opponent runs: rankable signal vs full game analysis cost.
const MIN_OPENING_GAMES_SHOW: i64 = 3; // Hide ultra-noisy opening rows; fewer games than this yields misleading %.
const MIN_OPENING_GAMES_GP: i64 = 6; // Game-plan suggestions need slightly more data than cards to avoid flip-flop advice.
const VERSUS_OPENINGS_PER_COLOR: usize = 2; // Frequent-openings block: top families per color.

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusOpeningCard {
    pub name: String,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub total: u32,
    /// Versus frequent-openings cards: `(wins + 0.5 * draws) / total` as a percentage; serialized as `winRatePct`.
    pub win_rate_pct: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusPlanEntry {
    pub title: String,
    pub subtitle: String,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusPlanSide {
    pub attack: Vec<VersusPlanEntry>,
    pub avoid: Vec<VersusPlanEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusGamePlan {
    pub opp_games_in_opening_slice: u32,
    pub as_white: VersusPlanSide,
    pub as_black: VersusPlanSide,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusSideSummary {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i64>,
    pub sample_size_metrics: u32,
    pub pentagon: Option<PentagonDto>,
    pub benchmark: PentagonDto,
    pub avg_accuracy_pct: Option<f64>,
    pub avg_acpl: Option<f64>,
    pub win_rate_pct: Option<f64>,
    pub blunders_per_game: Option<f64>,
    pub conversion_pct: Option<f64>,
    pub openings_as_white: Vec<VersusOpeningCard>,
    pub openings_as_black: Vec<VersusOpeningCard>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusDiagnostics {
    /// Games retained for this speed (`rated`, speed incl. aliases, non-empty moves).
    pub opponent_games_matching_speed: u32,
    pub opponent_analyses_attempted: u32,
    pub opponent_analyses_succeeded: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_lichess_speeds_when_no_match: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_analysis_error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusSpeedSlice {
    pub diagnostic: VersusDiagnostics,
    pub self_side: VersusSideSummary,
    pub opponent_side: VersusSideSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_plan: Option<VersusGamePlan>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusSlices {
    pub bullet: VersusSpeedSlice,
    pub blitz: VersusSpeedSlice,
    pub rapid: VersusSpeedSlice,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusCompareResponse {
    /// Games successfully parsed from the NDJSON batch (before per-speed filters).
    pub opponent_games_in_api_sample: u32,
    pub slices: VersusSlices,
}

/// Normalize Lichess `speed`/`perf.type` buckets to the app's bullet | blitz | rapid filters.
fn game_matches_requested_speed(game_speed: &str, requested: &str) -> bool {
    let gs = game_speed.trim().to_lowercase();
    let rq = requested.trim().to_lowercase();
    if gs.is_empty() {
        return false;
    }
    if gs == rq {
        return true;
    }
    // Lichess splits hyper-bullet vs bullet; players often choose "bullet" in UI for both.
    if rq == "bullet" && gs == "ultrabullet" {
        return true;
    }
    false
}

fn rating_for_speed_u(user: &crate::db::users::model::User, speed: &str) -> Option<i64> {
    match speed {
        "bullet" => user.bullet_rating,
        "blitz" => user.blitz_rating,
        "rapid" => user.rapid_rating,
        _ => None,
    }
}

fn emit_prog(app: &AppHandle, phase: &str, current: u32, total: u32) {
    let _ = app.emit(
        "versus://progress",
        serde_json::json!({
            "phase": phase,
            "current": current,
            "total": total,
        }),
    );
}

fn win_rate_pct(wins: i64, total: i64) -> f64 {
    if total <= 0 {
        return 0.0;
    }
    (100.0 * wins as f64 / total as f64).clamp(0.0, 100.0)
}

/// Opening "family" label: text before the first `:` (Lichess `Main: Variation` pattern), else full trimmed name.
fn opening_family_label(raw: &str) -> String {
    let s = raw.trim();
    let head = s.split_once(':').map(|(a, _)| a.trim()).unwrap_or(s);
    if head.is_empty() {
        s.to_string()
    } else {
        head.to_string()
    }
}

fn opening_score_pct(wins: i64, draws: i64, total: i64) -> f64 {
    if total <= 0 {
        return 0.0;
    }
    (100.0 * (wins as f64 + 0.5 * draws as f64) / total as f64).clamp(0.0, 100.0)
}

fn merge_opening_rows_by_family(rows: Vec<OpeningAggregateRow>) -> Vec<OpeningAggregateRow> {
    let mut m: HashMap<String, OpeningAggregateRow> = HashMap::new();
    for r in rows {
        let key = opening_family_label(&r.opening_name);
        let entry = m.entry(key.clone()).or_insert_with(|| OpeningAggregateRow {
            opening_name: key.clone(),
            wins: 0,
            losses: 0,
            draws: 0,
            total: 0,
        });
        entry.wins += r.wins;
        entry.losses += r.losses;
        entry.draws += r.draws;
        entry.total += r.total;
    }
    m.into_values().collect()
}

/// Per full `opening_name` from Lichess: `(wins, draws, total)`.
fn aggregate_openings(games: &[Game], player_color_filter: Option<&str>) -> HashMap<String, (i64, i64, i64)> {
    let mut m: HashMap<String, (i64, i64, i64)> = HashMap::new();
    for g in games {
        if player_color_filter.is_some_and(|c| g.player_color != c) {
            continue;
        }
        let Some(name_raw) = g.opening_name.as_ref().map(|s| s.trim().to_string()) else {
            continue;
        };
        if name_raw.is_empty() {
            continue;
        }
        let e = m.entry(name_raw).or_insert((0, 0, 0));
        e.2 += 1;
        match g.player_result.as_str() {
            "win" => e.0 += 1,
            "draw" => e.1 += 1,
            _ => {}
        }
    }
    m
}

fn rows_to_cards(rows: &[OpeningAggregateRow], take: usize) -> Vec<VersusOpeningCard> {
    let mut v: Vec<VersusOpeningCard> = rows
        .iter()
        .filter(|r| r.total >= MIN_OPENING_GAMES_SHOW)
        .map(|r| VersusOpeningCard {
            name: r.opening_name.clone(),
            wins: r.wins.max(0) as u32,
            draws: r.draws.max(0) as u32,
            losses: r.losses.max(0) as u32,
            total: r.total.max(0) as u32,
            win_rate_pct: opening_score_pct(r.wins, r.draws, r.total),
        })
        .collect();
    v.sort_by(|a, b| {
        b.total
            .cmp(&a.total)
            .then_with(|| b.win_rate_pct.partial_cmp(&a.win_rate_pct).unwrap_or(std::cmp::Ordering::Equal))
    });
    v.truncate(take);
    v
}

fn map_agg_hash_to_cards(m: HashMap<String, (i64, i64, i64)>, take: usize) -> Vec<VersusOpeningCard> {
    let rows: Vec<OpeningAggregateRow> = m
        .into_iter()
        .map(|(opening_name, (wins, draws, total))| {
            let losses = (total - wins - draws).max(0);
            OpeningAggregateRow {
                opening_name,
                wins,
                losses,
                draws,
                total,
            }
        })
        .collect();
    let merged = merge_opening_rows_by_family(rows);
    rows_to_cards(&merged, take)
}

fn pick_plan_side(map: HashMap<String, (i64, i64, i64)>) -> VersusPlanSide {
    let mut scored: Vec<(String, i64, i64, f64)> = map
        .into_iter()
        .filter(|(_, (w, _d, tot))| {
            let tot = *tot;
            let w = *w;
            tot >= MIN_OPENING_GAMES_GP && w >= 0 && tot >= w
        })
        .map(|(name, (wins, _draws, total))| {
            let pct = win_rate_pct(wins, total);
            (name, wins, total, pct)
        })
        .collect();

    scored.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
    let attack_take: Vec<_> = scored.iter().take(2).cloned().collect();
    scored.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
    let avoid_take: Vec<_> = scored.iter().take(2).cloned().collect();

    let mk_entry = |(name, _w, tot, pct): (String, i64, i64, f64), attack: bool| VersusPlanEntry {
        title: name.clone(),
        subtitle: format!(
            "{} — {}% {}",
            tot,
            (pct.round() as i32),
            if attack {
                "(lower win rate in this cohort)"
            } else {
                "(higher win rate — familiar terrain)"
            }
        ),
        tier: if attack { "attack".into() } else { "avoid".into() },
    };

    VersusPlanSide {
        attack: attack_take.into_iter().map(|t| mk_entry(t, true)).collect(),
        avoid: avoid_take.into_iter().map(|t| mk_entry(t, false)).collect(),
    }
}

fn build_game_plan(opp_filtered: &[Game]) -> VersusGamePlan {
    let white_map = aggregate_openings(opp_filtered, Some("white"));
    let black_map = aggregate_openings(opp_filtered, Some("black"));
    VersusGamePlan {
        opp_games_in_opening_slice: opp_filtered.len() as u32,
        as_white: pick_plan_side(white_map),
        as_black: pick_plan_side(black_map),
    }
}

fn mean_accuracy(rows: &[MetricGameRow]) -> Option<f64> {
    if rows.is_empty() {
        return None;
    }
    let s: f64 = rows.iter().map(|r| effective_game_accuracy(r.accuracy_raw, r.avg_centipawn_loss)).sum();
    Some(s / rows.len() as f64)
}

fn mean_acpl(rows: &[MetricGameRow]) -> Option<f64> {
    let v: Vec<f64> = rows
        .iter()
        .filter_map(|r| r.avg_centipawn_loss.filter(|x| x.is_finite()))
        .collect();
    if v.is_empty() {
        return None;
    }
    Some(v.iter().copied().sum::<f64>() / v.len() as f64)
}

fn mean_blunders(rows: &[MetricGameRow]) -> Option<f64> {
    let v: Vec<f64> = rows
        .iter()
        .filter_map(|r| r.blunders.map(|b| b as f64))
        .collect();
    if v.is_empty() {
        return None;
    }
    Some(v.iter().copied().sum::<f64>() / v.len() as f64)
}

fn win_rate(rows: &[MetricGameRow]) -> Option<f64> {
    if rows.is_empty() {
        return None;
    }
    let w = rows.iter().filter(|r| r.player_result == "win").count() as f64;
    Some(100.0 * w / rows.len() as f64)
}

fn summary_side(
    username: String,
    rating: Option<i64>,
    rows: &[MetricGameRow],
    bench: PentagonDto,
    openings_as_white: Vec<VersusOpeningCard>,
    openings_as_black: Vec<VersusOpeningCard>,
) -> VersusSideSummary {
    let pent = pentagon_from_metrics(rows, &bench);
    VersusSideSummary {
        username,
        rating,
        sample_size_metrics: rows.len() as u32,
        conversion_pct: pent.as_ref().and_then(|p| p.conversion),
        pentagon: pent,
        benchmark: bench,
        avg_accuracy_pct: mean_accuracy(rows),
        avg_acpl: mean_acpl(rows),
        win_rate_pct: win_rate(rows),
        blunders_per_game: mean_blunders(rows),
        openings_as_white,
        openings_as_black,
    }
}

fn filter_opponent_games_for_speed(parsed: &[Game], speed_lc: &str) -> Vec<Game> {
    let mut opp_games: Vec<Game> = parsed
        .iter()
        .filter(|g| {
            g.rated
                && game_matches_requested_speed(&g.speed, speed_lc)
                && g.moves.as_ref().map(|m| !m.trim().is_empty()).unwrap_or(false)
        })
        .cloned()
        .collect();
    opp_games.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    opp_games
}

struct VersusSpeedDraft {
    opponent_games_matching_speed: u32,
    speed_hint: Option<Vec<String>>,
    self_side: VersusSideSummary,
    opponent_username_display: String,
    opp_rating: Option<i64>,
    bench_opp: PentagonDto,
    opp_open_white: Vec<VersusOpeningCard>,
    opp_open_black: Vec<VersusOpeningCard>,
    to_analyze: Vec<Game>,
    game_plan: Option<VersusGamePlan>,
}

fn prepare_versus_speed_slice(
    conn: &rusqlite::Connection,
    user: &crate::db::users::model::User,
    opponent_username_display: String,
    speed_lc: &str,
    opp_games_speed: &[Game],
    distinct_speeds_hint: &[String],
) -> Result<VersusSpeedDraft, String> {
    let opponent_games_matching_speed = opp_games_speed.len() as u32;

    let speed_hint =
        if opponent_games_matching_speed == 0 && !distinct_speeds_hint.is_empty() {
            Some(distinct_speeds_hint.to_vec())
        } else {
            None
        };

    let opp_rating = opp_games_speed.first().and_then(|g| g.player_rating);
    let self_rating = rating_for_speed_u(user, speed_lc);

    let self_rows = games_repo::versus_self_analyzed_metrics(
        conn,
        &user.username,
        &user.id,
        speed_lc,
        SELF_METRICS_LIMIT,
    )
    .map_err(|e| e.to_string())?;

    let self_white_agg = games_repo::versus_opening_stats_recent_for_color(
        conn,
        &user.username,
        speed_lc,
        SELF_OPENINGS_RECENT_LIMIT,
        "white",
    )
    .map_err(|e| e.to_string())?;
    let self_black_agg = games_repo::versus_opening_stats_recent_for_color(
        conn,
        &user.username,
        speed_lc,
        SELF_OPENINGS_RECENT_LIMIT,
        "black",
    )
    .map_err(|e| e.to_string())?;
    let self_open_white =
        rows_to_cards(&merge_opening_rows_by_family(self_white_agg), VERSUS_OPENINGS_PER_COLOR);
    let self_open_black =
        rows_to_cards(&merge_opening_rows_by_family(self_black_agg), VERSUS_OPENINGS_PER_COLOR);

    let bucket_you = benchmarks::bucket_key_for_rating(self_rating.unwrap_or(1500));
    let (bench_you_pent, _) = benchmarks::pentagon_and_label(&bucket_you)
        .ok_or_else(|| format!("Unknown benchmark bucket: {}", bucket_you))?;
    let bench_you = PentagonDto::from(bench_you_pent);

    let bucket_opp = benchmarks::bucket_key_for_rating(opp_rating.unwrap_or(1500));
    let (bench_opp_pent, _) = benchmarks::pentagon_and_label(&bucket_opp)
        .ok_or_else(|| format!("Unknown benchmark bucket: {}", bucket_opp))?;
    let bench_opp = PentagonDto::from(bench_opp_pent);

    let opp_open_white = map_agg_hash_to_cards(
        aggregate_openings(opp_games_speed, Some("white")),
        VERSUS_OPENINGS_PER_COLOR,
    );
    let opp_open_black = map_agg_hash_to_cards(
        aggregate_openings(opp_games_speed, Some("black")),
        VERSUS_OPENINGS_PER_COLOR,
    );

    // Newest games first carry the opponent’s current rating context; older tail rarely changes the pentagon mean.
    let to_analyze: Vec<Game> = opp_games_speed
        .iter()
        .take(OPP_ANALYZE_MAX)
        .cloned()
        .collect();

    let gp = if !opp_games_speed.is_empty() {
        Some(build_game_plan(opp_games_speed))
    } else {
        None
    };

    let self_side = summary_side(
        user.username.clone(),
        self_rating,
        &self_rows,
        bench_you,
        self_open_white,
        self_open_black,
    );

    Ok(VersusSpeedDraft {
        opponent_games_matching_speed,
        speed_hint,
        self_side,
        opponent_username_display,
        opp_rating,
        bench_opp,
        opp_open_white,
        opp_open_black,
        to_analyze,
        game_plan: gp,
    })
}

async fn transient_analyze_slice_games(
    app: &AppHandle,
    to_analyze: &[Game],
    prog_current: &mut u32,
    prog_total: u32,
) -> Result<(Vec<MetricGameRow>, u32, u32, Option<String>), String> {
    let mut opp_metric_rows: Vec<MetricGameRow> = Vec::with_capacity(to_analyze.len());
    let mut opponent_analyses_attempted = 0u32;
    let mut opponent_analyses_succeeded = 0u32;
    let mut first_analysis_error: Option<String> = None;

    for g in to_analyze {
        if VERSUS_CANCEL.load(Ordering::SeqCst) {
            return Err("Versus comparison cancelled".into());
        }
        emit_prog(app, "analyze_opponent", *prog_current, prog_total);
        *prog_current = prog_current.saturating_add(1);
        opponent_analyses_attempted += 1;
        match analyze_game_transient(app, g, ANALYSIS_DEPTH) {
            Ok(t) => {
                opp_metric_rows.push(transient_to_metric(g, &t));
                opponent_analyses_succeeded += 1;
            }
            Err(e) => {
                if first_analysis_error.is_none() {
                    first_analysis_error = Some(truncate_diag_err(&e, 280));
                }
            }
        }
    }

    Ok((
        opp_metric_rows,
        opponent_analyses_attempted,
        opponent_analyses_succeeded,
        first_analysis_error,
    ))
}

async fn versus_finish_speed_slice(
    app: &AppHandle,
    draft: VersusSpeedDraft,
    prog_current: &mut u32,
    prog_total: u32,
) -> Result<VersusSpeedSlice, String> {
    let (
        opp_metric_rows,
        opponent_analyses_attempted,
        opponent_analyses_succeeded,
        first_analysis_error,
    ) = transient_analyze_slice_games(app, &draft.to_analyze, prog_current, prog_total).await?;

    let opponent_side = summary_side(
        draft.opponent_username_display,
        draft.opp_rating,
        &opp_metric_rows,
        draft.bench_opp,
        draft.opp_open_white,
        draft.opp_open_black,
    );

    Ok(VersusSpeedSlice {
        diagnostic: VersusDiagnostics {
            opponent_games_matching_speed: draft.opponent_games_matching_speed,
            opponent_analyses_attempted,
            opponent_analyses_succeeded,
            sample_lichess_speeds_when_no_match: draft.speed_hint,
            first_analysis_error,
        },
        self_side: draft.self_side,
        opponent_side,
        game_plan: draft.game_plan,
    })
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
    let user = users_repo::get_active_user(&conn)?
        .ok_or("Active user not found")?;

    if opponent_slug.eq_ignore_ascii_case(user.username.trim()) {
        return Err("Choose an opponent other than yourself".into());
    }

    emit_prog(&app, "fetch_opponent", 0, 1);
    let ndjson = lichess::fetch_games(&app, &opponent_slug, None, Some(LICHESS_FETCH_MAX))
        .await?;
    emit_prog(&app, "fetch_opponent", 1, 1);

    let parsed_games = lichess_games::parse_ndjson(&opponent_slug, &ndjson);
    let opponent_games_in_api_sample = parsed_games.len() as u32;

    let mut distinct_speeds_rated_with_moves: Vec<String> = parsed_games
        .iter()
        .filter(|g| {
            g.rated && g.moves.as_ref().map(|m| !m.trim().is_empty()).unwrap_or(false)
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
    let bullet = versus_finish_speed_slice(&app, bullet_draft, &mut prog_current, analyze_total).await?;

    let blitz_draft = prepare_versus_speed_slice(
        &conn,
        &user,
        opponent_display.clone(),
        "blitz",
        &filtered_by_speed[1],
        &distinct_speeds_rated_with_moves,
    )?;
    let blitz = versus_finish_speed_slice(&app, blitz_draft, &mut prog_current, analyze_total).await?;

    let rapid_draft = prepare_versus_speed_slice(
        &conn,
        &user,
        opponent_display,
        "rapid",
        &filtered_by_speed[2],
        &distinct_speeds_rated_with_moves,
    )?;
    let rapid = versus_finish_speed_slice(&app, rapid_draft, &mut prog_current, analyze_total).await?;

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
        slices: VersusSlices { bullet, blitz, rapid },
    })
}

// Keeps first-analysis error JSON-friendly for SSE payloads without dumping huge engine stderr into the UI.
fn truncate_diag_err(s: &str, max_chars: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= max_chars {
        return t.to_string();
    }
    format!("{}…", t.chars().take(max_chars.saturating_sub(1)).collect::<String>())
}

fn transient_to_metric(game: &Game, t: &TransientAnalysisResult) -> MetricGameRow {
    MetricGameRow {
        accuracy_raw: t.accuracy,
        avg_centipawn_loss: Some(t.avg_centipawn_loss),
        max_adv: Some(i64::from(t.max_advantage_cp)),
        blunders: Some(i64::from(t.blunders)),
        player_result: game.player_result.clone(),
        opening_blunder: t
            .pattern_tags
            .iter()
            .any(|tag| tag == "opening_blunder"),
        endgame_blunder: t.pattern_tags.iter().any(|tag| tag == "endgame_blunder"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_family_label_strips_variation() {
        assert_eq!(
            opening_family_label("  Modern Defense: Some Line  "),
            "Modern Defense"
        );
        assert_eq!(opening_family_label("French Defense"), "French Defense");
    }

    #[test]
    fn opening_score_pct_counts_half_draws() {
        assert!((opening_score_pct(1, 2, 4) - 50.0).abs() < 1e-9);
        assert!((opening_score_pct(2, 0, 4) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn merge_opening_rows_by_family_sums_lines() {
        let rows = vec![
            OpeningAggregateRow {
                opening_name: "Scandinavian Defense: Main Line".into(),
                wins: 2,
                losses: 1,
                draws: 1,
                total: 4,
            },
            OpeningAggregateRow {
                opening_name: "Scandinavian Defense: Mieses".into(),
                wins: 1,
                losses: 0,
                draws: 1,
                total: 2,
            },
        ];
        let merged = merge_opening_rows_by_family(rows);
        assert_eq!(merged.len(), 1);
        let m = &merged[0];
        assert_eq!(m.opening_name, "Scandinavian Defense");
        assert_eq!(m.wins, 3);
        assert_eq!(m.draws, 2);
        assert_eq!(m.total, 6);
        assert!((opening_score_pct(m.wins, m.draws, m.total) - (100.0 * 4.0 / 6.0)).abs() < 1e-9);
    }
}
