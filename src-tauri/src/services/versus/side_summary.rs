use crate::services::versus_metrics::{
    effective_game_accuracy, pentagon_from_metrics, MetricGameRow, PentagonDto,
};

use super::types::{VersusOpeningCard, VersusSideSummary};

fn mean_accuracy(rows: &[MetricGameRow]) -> Option<f64> {
    if rows.is_empty() {
        return None;
    }
    let s: f64 = rows
        .iter()
        .map(|r| effective_game_accuracy(r.accuracy_raw, r.avg_centipawn_loss))
        .sum();
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

pub(crate) fn summary_side(
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
