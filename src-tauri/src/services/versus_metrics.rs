//! Pentagon DTO and math shared by profile chart and Versus.
//! Stability: absolute spread score with noisy low-accuracy games excluded and shrinkage toward cohort benchmark on small samples.
use serde::Serialize;

use crate::services::benchmarks::Pentagon as BenchPentagon;

pub const MIN_PENTAGON_SAMPLE: usize = 5; // Below this, variance collapses the polygon; UI shows null instead of noise.
/// Per-game accuracy at or below this is excluded from stability variance (broken / placeholder analysis rows).
pub const STABILITY_ACCURACY_FLOOR: f64 = 1.0;
const STABILITY_SHRINK_FULL_SAMPLE: f64 = 20.0;
const STABILITY_SIGMA_CAP: f64 = 20.0;
const STABILITY_BENCH_FLOOR_RATIO: f64 = 0.35;
const STABILITY_ABSOLUTE_FLOOR: f64 = 15.0;

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

/// Absolute stability from per-game accuracies (after floor filter), blended toward `benchmark.stability` when n < 20.
pub fn stability_score(accuracies_for_stability: &[f64], benchmark_stability: f64) -> f64 {
    if accuracies_for_stability.is_empty() {
        return benchmark_stability.clamp(0.0, 100.0);
    }
    let sigma = population_std_dev(accuracies_for_stability).min(STABILITY_SIGMA_CAP);
    let raw = (100.0 - 4.0 * sigma).clamp(0.0, 100.0);
    let n = accuracies_for_stability.len() as f64;
    let w = (n / STABILITY_SHRINK_FULL_SAMPLE).min(1.0);
    let bench = benchmark_stability.clamp(0.0, 100.0);
    let blended = w * raw + (1.0 - w) * bench;
    let floor = (bench * STABILITY_BENCH_FLOOR_RATIO).max(STABILITY_ABSOLUTE_FLOOR);
    blended.max(floor).clamp(0.0, 100.0)
}

/// Pentagon built from per-game metric rows; `benchmark` anchors stability on small or noisy samples.
pub fn pentagon_from_metrics(games: &[MetricGameRow], benchmark: &PentagonDto) -> Option<PentagonDto> {
    if games.len() < MIN_PENTAGON_SAMPLE {
        return None;
    }

    let accuracies: Vec<f64> = games
        .iter()
        .map(|g| effective_game_accuracy(g.accuracy_raw, g.avg_centipawn_loss))
        .collect();

    let accuracies_for_stability: Vec<f64> = accuracies
        .iter()
        .copied()
        .filter(|a| *a > STABILITY_ACCURACY_FLOOR)
        .collect();

    if accuracies_for_stability.len() < MIN_PENTAGON_SAMPLE {
        return None;
    }

    let n = games.len() as f64;
    let accuracy = accuracies.iter().sum::<f64>() / n;
    let stability = stability_score(&accuracies_for_stability, benchmark.stability);

    // Conversion only counts positions where the player reached +200cp; otherwise won-won games look inflated.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_benchmark() -> PentagonDto {
        PentagonDto {
            accuracy: 80.0,
            stability: 80.0,
            conversion: Some(70.0),
            openings: 80.0,
            endgame: 90.0,
        }
    }

    fn row(acc: f64) -> MetricGameRow {
        MetricGameRow {
            accuracy_raw: acc,
            avg_centipawn_loss: None,
            max_adv: Some(250),
            blunders: Some(0),
            player_result: "win".into(),
            opening_blunder: false,
            endgame_blunder: false,
        }
    }

    #[test]
    fn stability_absolute_raw_at_full_sample() {
        assert!((stability_score(&[80.0; 20], 80.0) - 100.0).abs() < 1e-6);
        let mid: Vec<f64> = (0..20).map(|i| 70.0 + (i as f64) * (20.0 / 19.0)).collect();
        let sigma = population_std_dev(&mid);
        let expected = (100.0 - 4.0 * sigma).clamp(0.0, 100.0);
        assert!((stability_score(&mid, 80.0) - expected).abs() < 1e-6);
    }

    #[test]
    fn stability_shrinkage_toward_benchmark_on_small_n() {
        let score_n5 = stability_score(&[85.0; 5], 80.0);
        assert!((score_n5 - 85.0).abs() < 1e-6); // w=0.25, raw=100 → 0.25*100+0.75*80
        let score_n20 = stability_score(&[85.0; 20], 80.0);
        assert!((score_n20 - 100.0).abs() < 1e-6);
    }

    #[test]
    fn stability_excludes_near_zero_accuracy_from_variance() {
        let noisy: Vec<MetricGameRow> = vec![
            row(0.0),
            row(0.0),
            row(90.0),
            row(92.0),
            row(91.0),
            row(93.0),
            row(89.0),
        ];
        let bench = dummy_benchmark();
        let with_zeros = pentagon_from_metrics(&noisy, &bench).expect("pentagon");
        let clean: Vec<MetricGameRow> = noisy.iter().skip(2).cloned().collect();
        let without_zeros = pentagon_from_metrics(&clean, &bench).expect("pentagon");
        assert!(with_zeros.stability > 0.0);
        assert!(with_zeros.stability >= without_zeros.stability - 1.0);
    }

    #[test]
    fn pentagon_requires_min_sample() {
        let bench = dummy_benchmark();
        let few: Vec<MetricGameRow> = (0..4).map(|_| row(85.0)).collect();
        assert!(pentagon_from_metrics(&few, &bench).is_none());
    }

    #[test]
    fn pentagon_none_when_too_few_valid_accuracies_for_stability() {
        let bench = dummy_benchmark();
        let games: Vec<MetricGameRow> = vec![
            row(0.0),
            row(0.0),
            row(0.5),
            row(0.0),
            row(0.0),
            row(90.0),
        ];
        assert!(pentagon_from_metrics(&games, &bench).is_none());
    }

    #[test]
    fn pentagon_returns_shrunk_stability_on_small_equal_sample() {
        let bench = dummy_benchmark();
        let games: Vec<MetricGameRow> = (0..5).map(|_| row(85.0)).collect();
        let p = pentagon_from_metrics(&games, &bench).expect("pentagon");
        assert!((p.stability - 85.0).abs() < 1e-6);
    }

    #[test]
    fn stability_floor_prevents_zero_on_extreme_spread() {
        let bench = 80.0;
        let wild: Vec<f64> = (0..50)
            .map(|i| if i % 2 == 0 { 40.0 } else { 95.0 })
            .collect();
        let score = stability_score(&wild, bench);
        let floor = (bench * STABILITY_BENCH_FLOOR_RATIO).max(STABILITY_ABSOLUTE_FLOOR);
        assert!(score >= floor - 1e-6);
        assert!(score > 0.0);
    }

    #[test]
    fn stability_sigma_cap_keeps_raw_at_least_twenty() {
        let wild: Vec<f64> = (0..30).map(|_| 0.0).chain((0..30).map(|_| 100.0)).collect();
        let sigma = population_std_dev(&wild).min(STABILITY_SIGMA_CAP);
        let raw = (100.0 - 4.0 * sigma).clamp(0.0, 100.0);
        assert!(raw >= 20.0 - 1e-6);
    }
}
