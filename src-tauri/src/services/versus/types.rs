//! Versus IPC response types (camelCase JSON for the frontend).

use std::collections::HashMap;

use serde::Serialize;

use crate::services::versus_metrics::PentagonDto;

/// One full Lichess `opening_name` line inside an aggregated opening family (share of games in that family).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusOpeningLineShare {
    pub name: String,
    pub total: u32,
    pub frequency_pct: f64,
}

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
    #[serde(default)]
    pub lines: Vec<VersusOpeningLineShare>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusPlanEntry {
    pub title: String,
    pub selection_tier: u8,
    pub reason_kind: String,
    pub reason_params: HashMap<String, serde_json::Value>,
    /// `play` or `avoid`
    pub tier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_win_rate_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opp_win_rate_pct: Option<f64>,
    pub self_games: u32,
    pub opp_games: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersusPlanSide {
    pub play: Vec<VersusPlanEntry>,
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
