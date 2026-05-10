//! Shared pentagon-style aggregates (home profile + Versus).
use serde::Serialize;

use crate::services::benchmarks::Pentagon as BenchPentagon;

pub const MIN_PENTAGON_SAMPLE: usize = 5;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PentagonDto {
    pub accuracy: f64,
    pub stability: f64,
    pub conversion: Option<f64>,
    pub openings: f64,
    pub endgame: f64,
}

impl From<BenchPentagon> for PentagonDto {
    fn from(p: BenchPentagon) -> Self {
        Self {
            accuracy: p.accuracy,
            stability: p.stability,
            conversion: Some(p.conversion),
            openings: p.openings,
            endgame: p.endgame,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricGameRow {
    pub accuracy_raw: f64,
    pub avg_centipawn_loss: Option<f64>,
    pub max_adv: Option<i64>,
    /// Average blunders per game from analysis (Versus-only / optional).
    pub blunders: Option<i64>,
    pub player_result: String,
    pub opening_blunder: bool,
    pub endgame_blunder: bool,
}

fn population_std_dev(samples: &[f64]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let n = samples.len() as f64;
    let mean = samples.iter().copied().sum::<f64>() / n;
    let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    var.sqrt()
}

fn accuracy_from_centipawns(acpl: f64) -> f64 {
    if acpl <= 0.0 {
        return 100.0;
    }
    let a = 103.1668_f64 * (-0.00368208_f64 * acpl).exp();
    a.clamp(0.0, 100.0)
}

/// Some rows persist `accuracy` as 0 even when ACPL implies a nonzero value.
pub fn effective_game_accuracy(acc: f64, acpl: Option<f64>) -> f64 {
    if !acc.is_finite() || acc <= f64::EPSILON {
        if let Some(acpl_v) = acpl.filter(|x| x.is_finite() && *x > 1e-6) {
            return accuracy_from_centipawns(acpl_v);
        }
    }
    acc.clamp(0.0, 100.0)
}

/// Cohort-relative pentagon built from per-game metric rows (`benchmark` = population bucket polygon).
pub fn pentagon_from_metrics(games: &[MetricGameRow], benchmark: &PentagonDto) -> Option<PentagonDto> {
    if games.len() < MIN_PENTAGON_SAMPLE {
        return None;
    }

    let accuracies: Vec<f64> = games
        .iter()
        .map(|g| effective_game_accuracy(g.accuracy_raw, g.avg_centipawn_loss))
        .collect();
    let n = games.len() as f64;
    let accuracy = accuracies.iter().sum::<f64>() / n;

    let sigma_player = population_std_dev(&accuracies);
    let sigma_eps = 1e-6_f64;
    let sigma_pop = ((100.0 - benchmark.stability).max(0.0) / 4.0).max(1e-3);
    let stability = (benchmark.stability * sigma_pop / sigma_player.max(sigma_eps)).clamp(0.0, 100.0);

    let eligible: Vec<&MetricGameRow> = games
        .iter()
        .filter(|g| g.max_adv.map(|m| m >= 200).unwrap_or(false))
        .collect();
    let conversion = if eligible.is_empty() {
        None
    } else {
        let wins = eligible
            .iter()
            .filter(|g| g.player_result == "win")
            .count() as f64;
        Some(100.0 * wins / eligible.len() as f64)
    };

    let ob = games.iter().filter(|g| g.opening_blunder).count() as f64;
    let eb = games.iter().filter(|g| g.endgame_blunder).count() as f64;
    let openings = (100.0 * (1.0 - ob / n)).max(0.0);
    let endgame = (100.0 * (1.0 - eb / n)).max(0.0);

    Some(PentagonDto {
        accuracy,
        stability,
        conversion,
        openings,
        endgame,
    })
}
