use std::sync::atomic::Ordering;

use tauri::AppHandle;

use crate::db::games::model::Game;
use crate::db::games::repository::{self as games_repo};
use crate::services::benchmarks;
use crate::services::game_analysis::service::{analyze_game_transient, TransientAnalysisResult};
use crate::services::versus_metrics::{MetricGameRow, PentagonDto};

use super::game_plan::build_game_plan;
use super::openings::{
    aggregate_openings, group_rows_into_families, map_agg_hash_to_cards, rows_to_cards_with_lines,
};
use super::progress::emit_prog;
use super::side_summary::summary_side;
use super::speed::rating_for_speed_u;
use super::types::{
    VersusDiagnostics, VersusGamePlan, VersusOpeningCard, VersusSideSummary, VersusSpeedSlice,
};
use super::{
    ANALYSIS_DEPTH, OPP_ANALYZE_MAX, SELF_METRICS_LIMIT, SELF_OPENINGS_RECENT_LIMIT,
    VERSUS_CANCEL, VERSUS_OPENINGS_PER_COLOR,
};

pub(crate) struct VersusSpeedDraft {
    pub opponent_games_matching_speed: u32,
    pub speed_hint: Option<Vec<String>>,
    pub self_side: VersusSideSummary,
    pub opponent_username_display: String,
    pub opp_rating: Option<i64>,
    pub bench_opp: PentagonDto,
    pub opp_open_white: Vec<VersusOpeningCard>,
    pub opp_open_black: Vec<VersusOpeningCard>,
    pub to_analyze: Vec<Game>,
    pub game_plan: Option<VersusGamePlan>,
}

pub(crate) fn prepare_versus_speed_slice(
    conn: &rusqlite::Connection,
    user: &crate::db::users::model::User,
    opponent_username_display: String,
    speed_lc: &str,
    opp_games_speed: &[Game],
    distinct_speeds_hint: &[String],
) -> Result<VersusSpeedDraft, String> {
    let opponent_games_matching_speed = opp_games_speed.len() as u32;

    let speed_hint = if opponent_games_matching_speed == 0 && !distinct_speeds_hint.is_empty() {
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
    let self_open_white = rows_to_cards_with_lines(
        group_rows_into_families(self_white_agg.clone()),
        VERSUS_OPENINGS_PER_COLOR,
    );
    let self_open_black = rows_to_cards_with_lines(
        group_rows_into_families(self_black_agg.clone()),
        VERSUS_OPENINGS_PER_COLOR,
    );

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
        Some(build_game_plan(
            opp_games_speed,
            &user.username,
            self_white_agg,
            self_black_agg,
        ))
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

pub(crate) async fn versus_finish_speed_slice(
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

// Keeps first-analysis error JSON-friendly for SSE payloads without dumping huge engine stderr into the UI.
fn truncate_diag_err(s: &str, max_chars: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= max_chars {
        return t.to_string();
    }
    format!(
        "{}…",
        t.chars()
            .take(max_chars.saturating_sub(1))
            .collect::<String>()
    )
}

fn transient_to_metric(game: &Game, t: &TransientAnalysisResult) -> MetricGameRow {
    MetricGameRow {
        accuracy_raw: t.accuracy,
        avg_centipawn_loss: Some(t.avg_centipawn_loss),
        max_adv: Some(i64::from(t.max_advantage_cp)),
        blunders: Some(i64::from(t.blunders)),
        player_result: game.player_result.clone(),
        opening_blunder: t.pattern_tags.iter().any(|tag| tag == "opening_blunder"),
        endgame_blunder: t.pattern_tags.iter().any(|tag| tag == "endgame_blunder"),
    }
}
