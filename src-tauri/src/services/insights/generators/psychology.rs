// Soft psychology-style patterns: streaks, rest gaps, tilt proxies from game order plus optional analysis rows.
use std::collections::HashMap;

use chrono::{Local, TimeZone};
use serde_json::json;

use crate::db::game_analyses::model::GameAnalysisRow;
use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, CAT_PSYCHOLOGY};

fn is_loss(g: &Game) -> bool {
    g.player_result == "loss"
}

fn is_win(g: &Game) -> bool {
    g.player_result == "win"
}

/// Chronological: oldest → newest.
fn sorted_chrono(games: &[Game]) -> Vec<&Game> {
    let mut v: Vec<&Game> = games.iter().collect();
    v.sort_by_key(|g| g.created_at);
    v
}

fn mean(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn rest_effect_insight(
    user_id: &str,
    games: &[Game],
    analyses: &HashMap<String, GameAnalysisRow>,
) -> Option<Insight> {
    let mut by_day: HashMap<chrono::NaiveDate, Vec<&Game>> = HashMap::new();
    for g in games {
        let date = Local
            .timestamp_millis_opt(g.created_at)
            .single()
            .map(|dt| dt.date_naive())
            .unwrap_or_else(|| Local::now().date_naive());
        by_day.entry(date).or_default().push(g);
    }
    for list in by_day.values_mut() {
        list.sort_by_key(|g| g.created_at);
    }

    let mut first_acc: Vec<f64> = Vec::new();
    let mut late_acc: Vec<f64> = Vec::new();

    for list in by_day.values() {
        if list.len() < 5 {
            continue;
        }
        let g0 = list[0];
        if let Some(a) = analyses.get(&g0.id) {
            if a.status == "done" {
                if let Some(acc) = a.accuracy {
                    first_acc.push(acc);
                }
            }
        }
        for g in list.iter().skip(4) {
            if let Some(a) = analyses.get(&g.id) {
                if a.status == "done" {
                    if let Some(acc) = a.accuracy {
                        late_acc.push(acc);
                    }
                }
            }
        }
    }

    if first_acc.len() < 15 || late_acc.len() < 20 {
        return None;
    }

    let m1 = mean(&first_acc);
    let m2 = mean(&late_acc);
    if m2 + 4.0 >= m1 {
        return None;
    }

    let d = m1 - m2;
    let d_r = d.round() as i64;
    let m1_r = m1.round() as i64;
    let m2_r = m2.round() as i64;

    Some(build_insight(
        format!("psych_rest_{user_id}"),
        user_id,
        "psychology_rest_effect",
        CAT_PSYCHOLOGY,
        "Эффект усталости в сессии".to_string(),
        format!(
            "Средняя точность по анализу: первая партия дня {m1_r}% ({n1} партий), с 5-й и далее в тот же день {m2_r}% ({n2} точек). Разница {d_r} п.п.",
            n1 = first_acc.len(),
            n2 = late_acc.len(),
        ),
        "warning",
        70,
        Some("Просадка точности, п.п.".to_string()),
        Some(format!("{d_r}")),
        Some(d_r as f64),
        Some("Остановись после нескольких партий или сделай длинный перерыв.".to_string()),
        "psychology:rest_effect",
        83,
        json!({
            "mean_first_pct": m1_r,
            "mean_fifth_plus_pct": m2_r,
            "n_first_samples": first_acc.len(),
            "n_late_samples": late_acc.len(),
            "drop_pp": d_r
        }),
    ))
}

/// Returns `CAT_PSYCHOLOGY` cards when longitudinal signals in `games` (+ optional `analyses`) clear minimum counts.
pub fn generate(
    user_id: &str,
    games: &[Game],
    analyses: &HashMap<String, GameAnalysisRow>,
) -> Vec<Insight> {
    if games.len() < 20 {
        return vec![];
    }

    let ch = sorted_chrono(games);
    let mut out = Vec::new();

    // Tilt
    let mut tilt_wins = 0i64;
    let mut tilt_n = 0i64;
    for i in 2..ch.len() {
        if i + 3 >= ch.len() {
            continue;
        }
        if is_loss(ch[i - 2]) && is_loss(ch[i - 1]) {
            let j = i + 2;
            if j < ch.len() {
                tilt_n += 1;
                if is_win(ch[j]) {
                    tilt_wins += 1;
                }
            }
        }
    }
    if tilt_n >= 5 {
        let wr = tilt_wins as f64 / tilt_n as f64;
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("psych_tilt_{user_id}"),
            user_id,
            "psychology_tilt",
            CAT_PSYCHOLOGY,
            "Tilt-детектор".to_string(),
            format!(
                "После двух поражений подряд винрейт в 3-й следующей партии: {pct}% ({tilt_wins}/{tilt_n})."
            ),
            if wr < 0.42 { "warning" } else { "info" },
            72,
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Сделай паузу или разминку после серии поражений.".to_string()),
            "tilt:after_ll_positions_3_4",
            98,
            json!({ "tilt_wins": tilt_wins, "tilt_n": tilt_n, "pct": pct }),
        ));
    }

    // Comeback: game right after any loss
    let mut cw = 0i64;
    let mut ct = 0i64;
    for i in 1..ch.len() {
        if is_loss(ch[i - 1]) {
            ct += 1;
            if is_win(ch[i]) {
                cw += 1;
            }
        }
    }
    if ct >= 15 {
        let wr = cw as f64 / ct as f64;
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("psych_comeback_{user_id}"),
            user_id,
            "psychology_comeback",
            CAT_PSYCHOLOGY,
            "Comeback".to_string(),
            format!("Винрейт в партии сразу после поражения: {pct}% ({cw}/{ct})."),
            if wr >= 0.48 { "good" } else { "info" },
            75,
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Сравни с общим темпом — видно, удаётся ли «отыгрываться».".to_string()),
            "comeback:after_loss",
            85,
            json!({ "wins_after_loss": cw, "trials_after_loss": ct, "pct": pct }),
        ));
    }

    if let Some(ins) = rest_effect_insight(user_id, games, analyses) {
        out.push(ins);
    }

    out
}
