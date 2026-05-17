// Middlegame vs endgame error mix and related tactical summaries from `KeyMomentRow` joined to completed analyses.
use std::collections::{HashMap, HashSet};

use serde_json::json;

use crate::db::game_analyses::model::{GameAnalysisRow, KeyMomentRow};
use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::benchmarks;
use crate::services::insights::insight_common::{build_insight, CAT_TACTICS};

fn is_error_kind(k: &str) -> bool {
    matches!(k, "blunder" | "mistake" | "inaccuracy")
}

/// Emits `CAT_TACTICS` insights when error counts across phases cross tuned thresholds (see in-file magic numbers).
pub fn generate(
    user_id: &str,
    games: &[Game],
    analyses: &HashMap<String, GameAnalysisRow>,
    moments: &[KeyMomentRow],
) -> Vec<Insight> {
    let mut out = Vec::new();

    // Three ply bands align with accuracy-by-phase insight copy (opening / middlegame / endgame).
    let mut op_err = 0i64;
    let mut mg_err = 0i64;
    let mut eg_err = 0i64;
    let mut games_with_moments: HashSet<String> = HashSet::new();
    let mut ratings: Vec<i64> = Vec::new();
    for m in moments {
        if !is_error_kind(&m.kind) {
            continue;
        }
        let Some(a) = analyses.get(&m.game_id) else {
            continue;
        };
        if a.status != "done" {
            continue;
        }
        games_with_moments.insert(m.game_id.clone());
        if m.ply <= 20 {
            op_err += 1;
        } else if m.ply <= 50 {
            mg_err += 1;
        } else {
            eg_err += 1;
        }
    }
    for g in games {
        if games_with_moments.contains(&g.id) {
            if let Some(r) = g.player_rating {
                ratings.push(r);
            }
        }
    }
    let total_err = op_err + mg_err + eg_err;
    let n_games = games_with_moments.len() as i64;
    if total_err >= 15 && n_games >= 8 {
        let op_share_r = (op_err as f64 / total_err as f64 * 100.0).round();
        let mg_share_r = (mg_err as f64 / total_err as f64 * 100.0).round();
        let eg_share_r = (eg_err as f64 / total_err as f64 * 100.0).round();
        let median_rating = {
            ratings.sort_unstable();
            if ratings.is_empty() {
                1500
            } else {
                ratings[ratings.len() / 2]
            }
        };
        let bench = benchmarks::phase_error_shares_for_rating(median_rating);
        out.push(build_insight(
            format!("tactics_phase_{user_id}"),
            user_id,
            "tactics_middlegame_vs_endgame",
            CAT_TACTICS,
            "Миттельшпиль vs эндшпиль".to_string(),
            format!(
                "Доля ошибок: дебют {op_share_r}%, миттельшпиль {mg_share_r}%, эндшпиль {eg_share_r}% ({n_games} партий с анализом)."
            ),
            if eg_share_r > 52.0 { "warning" } else { "info" },
            78,
            None,
            None,
            None,
            Some("Добавь эндшпильные задачи, если хвост партии проседает.".to_string()),
            "tactics:phase_error_split",
            76,
            json!({
                "op_err": op_err,
                "mg_err": mg_err,
                "eg_err": eg_err,
                "total_err": total_err,
                "n_games": n_games,
                "opening_share": op_share_r,
                "middlegame_share": mg_share_r,
                "endgame_share": eg_share_r,
                "bench_opening_pct": bench.opening,
                "bench_middlegame_pct": bench.middlegame,
                "bench_endgame_pct": bench.endgame
            }),
        ));
    }

    // Failed conversion: had >= +2.0 advantage, did not win — with time-control breakdown
    let mut by_speed: HashMap<String, (i64, i64)> = HashMap::new();
    for g in games {
        let Some(a) = analyses.get(&g.id) else {
            continue;
        };
        if a.status != "done" {
            continue;
        }
        if a.max_advantage_cp.unwrap_or(0) < 200 {
            continue;
        }
        let label = match g.speed.as_str() {
            "bullet" => "Bullet",
            "blitz" => "Blitz",
            "rapid" => "Rapid",
            "classical" => "Classical",
            other => other,
        };
        let e = by_speed.entry(label.to_string()).or_insert((0, 0));
        e.0 += 1;
        if g.player_result != "win" {
            e.1 += 1;
        }
    }

    let mut with_adv = 0i64;
    let mut failed = 0i64;
    for (w, f) in by_speed.values() {
        with_adv += w;
        failed += f;
    }

    if with_adv >= 8 {
        let rate = (failed as f64 / with_adv as f64) * 100.0;
        let rate_r = rate.round();

        let mut parts: Vec<String> = Vec::new();
        let mut breakdown = serde_json::Map::new();
        for (label, (w, f)) in &by_speed {
            if *w < 3 {
                continue;
            }
            let r = (*f as f64 / *w as f64 * 100.0).round() as i64;
            parts.push(format!("{label}: {r}% ({f}/{w})"));
            breakdown.insert(
                label.clone(),
                json!({ "with_adv": w, "failed": f, "rate": r }),
            );
        }
        let by_line = if parts.is_empty() {
            String::new()
        } else {
            format!(" {}", parts.join(" · "))
        };
        let speed_split = parts.join(" · ");

        out.push(build_insight(
            format!("tactics_conversion_{user_id}"),
            user_id,
            "tactics_conversion_advantage",
            CAT_TACTICS,
            "Конверсия выигранных позиций".to_string(),
            format!(
                "В {with_adv} партиях был перевес ≥+2.0, но {failed} закончились не победой ({rate_r}%).{by_line}"
            ),
            if rate > 35.0 { "warning" } else { "info" },
            82,
            Some("Упущенные победы".to_string()),
            Some(format!("{rate_r}%")),
            Some(rate_r),
            Some("Потренируй техническую реализацию и контроль времени.".to_string()),
            "tactics:failed_conversion_rate",
            86,
            json!({
                "with_adv": with_adv,
                "failed": failed,
                "rate": rate_r,
                "speed_split": speed_split,
                "by_speed": serde_json::Value::Object(breakdown)
            }),
        ));
    }

    out.extend(
        crate::services::insights::generators::tactics_phase_accuracy::generate(
            user_id, games, analyses,
        ),
    );

    out
}
