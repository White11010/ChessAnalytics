// Opening repertoire stats, hot lines, and time-decayed performance views derived from `Game.opening_*` fields.
use std::collections::HashMap;

use serde_json::json;

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, now_ms, CAT_OPENINGS};

const MS_DAY: i64 = 24 * 60 * 60 * 1000; // Exact ms in a day for sliding windows without chrono in every helper.

#[derive(Default, Clone)]
struct OpeningStats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

fn win_rate(s: &OpeningStats) -> f64 {
    if s.games == 0 {
        return 0.0;
    }
    (s.wins as f64 + s.draws as f64 * 0.5) / s.games as f64
}

fn opening_subject(name: &str) -> String {
    format!("opening:{name}")
}

fn confidence_from_n(n: i64) -> i64 {
    if n >= 30 {
        95
    } else if n >= 15 {
        85
    } else if n >= 10 {
        75
    } else if n >= 5 {
        65
    } else {
        55
    }
}

fn rare_gem_confidence(n: i64) -> i64 {
    match n {
        1 => 32,
        2 => 36,
        3 => 40,
        4 => 42,
        5 => 44,
        6 => 45,
        7 => 46,
        8 => 47,
        _ => 48,
    }
}

/// `opening_trend` using an explicit clock (for tests).
pub(crate) fn opening_trend_insight(user_id: &str, games: &[Game], now_ms_val: i64) -> Option<Insight> {
    let recent_start = now_ms_val - 30 * MS_DAY;
    let prev_start = now_ms_val - 60 * MS_DAY;
    let prev_end = recent_start;

    let mut recent_map: HashMap<String, OpeningStats> = HashMap::new();
    let mut prev_map: HashMap<String, OpeningStats> = HashMap::new();

    for g in games {
        let Some(name) = g.opening_name.clone() else {
            continue;
        };
        if name.trim().is_empty() {
            continue;
        }
        let t = g.created_at;
        let st_ref = if t >= recent_start {
            recent_map.entry(name.clone()).or_default()
        } else if t >= prev_start && t < prev_end {
            prev_map.entry(name).or_default()
        } else {
            continue;
        };
        st_ref.games += 1;
        match g.player_result.as_str() {
            "win" => st_ref.wins += 1,
            "loss" => st_ref.losses += 1,
            _ => st_ref.draws += 1,
        }
    }

    let mut best: Option<(String, f64, i64, i64, f64, f64)> = None;
    // name, delta_wr, n_prev, n_recent, wr_prev_pct, wr_recent_pct
    for (name, r_st) in &recent_map {
        if r_st.games < 5 {
            continue;
        }
        let Some(p_st) = prev_map.get(name) else {
            continue;
        };
        if p_st.games < 5 {
            continue;
        }
        let wr_r = win_rate(r_st);
        let wr_p = win_rate(p_st);
        let delta = wr_r - wr_p;
        if delta.abs() < 0.12 {
            continue;
        }
        match &best {
            None => best = Some((
                name.clone(),
                delta,
                p_st.games,
                r_st.games,
                wr_p,
                wr_r,
            )),
            Some((_, d, _, _, _, _)) if delta.abs() > d.abs() => {
                best = Some((
                    name.clone(),
                    delta,
                    p_st.games,
                    r_st.games,
                    wr_p,
                    wr_r,
                ));
            }
            _ => {}
        }
    }

    let (name, delta, n_prev, n_recent, wr_p, wr_r) = best?;
    let wr_prev_pct = (wr_p * 100.0).round() as i64;
    let wr_recent_pct = (wr_r * 100.0).round() as i64;
    let improved = delta > 0.0;
    let sev = if improved { "good" } else { "warning" };
    let conf = if n_prev.min(n_recent) >= 12 {
        78
    } else {
        62
    };

    let short_recent_start = now_ms_val - 14 * MS_DAY;
    let short_prev_start = now_ms_val - 28 * MS_DAY;
    let short_prev_end = short_recent_start;
    let mut short_recent = OpeningStats::default();
    let mut short_prev = OpeningStats::default();
    for g in games {
        let Some(on) = g.opening_name.as_ref() else {
            continue;
        };
        if on != &name {
            continue;
        }
        let t = g.created_at;
        let st_ref = if t >= short_recent_start {
            &mut short_recent
        } else if t >= short_prev_start && t < short_prev_end {
            &mut short_prev
        } else {
            continue;
        };
        st_ref.games += 1;
        match g.player_result.as_str() {
            "win" => st_ref.wins += 1,
            "loss" => st_ref.losses += 1,
            _ => st_ref.draws += 1,
        }
    }

    let mut params = serde_json::Map::new();
    params.insert("opening".into(), json!(name));
    params.insert("wr_prev_pct".into(), json!(wr_prev_pct));
    params.insert("wr_recent_pct".into(), json!(wr_recent_pct));
    params.insert("n_prev".into(), json!(n_prev));
    params.insert("n_recent".into(), json!(n_recent));
    params.insert("delta_pp".into(), json!((delta * 100.0).round()));
    params.insert("improved".into(), json!(improved));

    if short_prev.games >= 3 && short_recent.games >= 3 {
        let wr_sp = win_rate(&short_prev);
        let wr_sr = win_rate(&short_recent);
        let delta_short = wr_sr - wr_sp;
        params.insert(
            "wr_short_prev_pct".into(),
            json!((wr_sp * 100.0).round() as i64),
        );
        params.insert(
            "wr_short_recent_pct".into(),
            json!((wr_sr * 100.0).round() as i64),
        );
        params.insert("delta_short_pp".into(), json!((delta_short * 100.0).round()));
        params.insert("improved_short".into(), json!(delta_short > 0.0));
        params.insert("n_short_prev".into(), json!(short_prev.games));
        params.insert("n_short_recent".into(), json!(short_recent.games));
    }

    Some(build_insight(
        format!("opening_trend_{user_id}"),
        user_id,
        "opening_trend",
        CAT_OPENINGS,
        "Динамика дебюта".to_string(),
        format!(
            "{name}: винрейт {wr_prev_pct}% → {wr_recent_pct}% (пред. 30 дн.: {n_prev} игр, последние 30 дн.: {n_recent})."
        ),
        sev,
        conf,
        None,
        None,
        None,
        Some(if improved {
            "Закрепляй то, что изменилось в подготовке.".to_string()
        } else {
            "Сверь с недавними партиями — возможно, нужен разбор варианта.".to_string()
        }),
        &format!("opening_trend:{name}"),
        84,
        serde_json::Value::Object(params),
    ))
}

fn opening_color_split_insight(user_id: &str, games: &[Game]) -> Option<Insight> {
    let mut by: HashMap<String, HashMap<String, OpeningStats>> = HashMap::new();
    for g in games {
        let Some(name) = g.opening_name.clone() else {
            continue;
        };
        if name.trim().is_empty() {
            continue;
        }
        let color = g.player_color.clone();
        if color != "white" && color != "black" {
            continue;
        }
        let m = by.entry(name).or_default();
        let st = m.entry(color).or_default();
        st.games += 1;
        match g.player_result.as_str() {
            "win" => st.wins += 1,
            "loss" => st.losses += 1,
            _ => st.draws += 1,
        }
    }

    let mut best: Option<(String, f64, String, f64, String, i64, i64)> = None;
    // name, wr_a, color_a, wr_b, color_b, n_a, n_b
    for (name, m) in &by {
        let (Some(w), Some(b)) = (m.get("white"), m.get("black")) else {
            continue;
        };
        if w.games < 5 || b.games < 5 {
            continue;
        }
        let wr_w = win_rate(w);
        let wr_b = win_rate(b);
        let gap = (wr_w - wr_b).abs();
        if gap < 0.12 {
            continue;
        }
        let (stronger_c, stronger_wr, _weaker_wr, n_s, n_w) = if wr_w >= wr_b {
            ("white", wr_w, wr_b, w.games, b.games)
        } else {
            ("black", wr_b, wr_w, b.games, w.games)
        };
        let weaker_c = if stronger_c == "white" { "black" } else { "white" };
        match &best {
            None => {
                best = Some((
                    name.clone(),
                    gap,
                    stronger_c.to_string(),
                    stronger_wr,
                    weaker_c.to_string(),
                    n_s,
                    n_w,
                ))
            }
            Some((_, g, _, _, _, _, _)) if gap > *g => {
                best = Some((
                    name.clone(),
                    gap,
                    stronger_c.to_string(),
                    stronger_wr,
                    weaker_c.to_string(),
                    n_s,
                    n_w,
                ))
            }
            _ => {}
        }
    }

    let (name, gap, stronger_c, stronger_wr, weaker_c, n_s, n_w) = best?;
    let s_pct = (stronger_wr * 100.0).round() as i64;
    let w_pct = ((stronger_wr - gap) * 100.0).round() as i64;
    let gap_pp = (gap * 100.0).round() as i64;

    Some(build_insight(
        format!("opening_color_split_{user_id}"),
        user_id,
        "opening_color_split",
        CAT_OPENINGS,
        "Дебют по цвету".to_string(),
        format!(
            "{name}: за {stronger_c} {s_pct}% ({n_s} игр), за {weaker_c} {w_pct}% ({n_w} игр), разница {gap_pp} п.п."
        ),
        "info",
        72,
        Some("Разница винрейта, п.п.".to_string()),
        Some(format!("{gap_pp}")),
        Some(gap_pp as f64),
        Some("Разведи подготовку по цветам — структуры различаются.".to_string()),
        &format!("opening_color_split:{name}"),
        79,
        json!({
            "opening": name,
            "stronger_color": stronger_c,
            "weaker_color": weaker_c,
            "stronger_wr_pct": s_pct,
            "weaker_wr_pct": w_pct,
            "n_stronger": n_s,
            "n_weaker": n_w,
            "gap_pp": gap_pp
        }),
    ))
}

/// Emits zero or more `CAT_OPENINGS` cards from the last imported games (no engine data required).
pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut map: HashMap<String, OpeningStats> = HashMap::new();
    let mut games_with_opening: i64 = 0;
    let mut overall = OpeningStats::default();

    for game in games {
        let Some(name) = game.opening_name.clone() else {
            continue;
        };
        if name.trim().is_empty() {
            continue;
        }
        games_with_opening += 1;
        overall.games += 1;
        match game.player_result.as_str() {
            "win" => {
                overall.wins += 1;
                let stats = map.entry(name).or_default();
                stats.games += 1;
                stats.wins += 1;
            }
            "loss" => {
                overall.losses += 1;
                let stats = map.entry(name).or_default();
                stats.games += 1;
                stats.losses += 1;
            }
            _ => {
                overall.draws += 1;
                let stats = map.entry(name).or_default();
                stats.games += 1;
                stats.draws += 1;
            }
        }
    }

    if games_with_opening < 10 {
        return vec![];
    }

    let overall_wr = win_rate(&overall);
    let overall_pct = (overall_wr * 100.0).round() as i64;

    let share_denom = games_with_opening.max(1) as f64;
    let mut out = Vec::new();

    // 1. Best opening (n >= 5)
    let mut best: Option<(String, f64, i64)> = None;
    for (name, st) in &map {
        if st.games < 5 {
            continue;
        }
        let wr = win_rate(st);
        match &best {
            None => best = Some((name.clone(), wr, st.games)),
            Some((_, w, _)) if wr > *w => best = Some((name.clone(), wr, st.games)),
            _ => {}
        }
    }
    if let Some((name, wr, n)) = best {
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("opening_best_{user_id}"),
            user_id,
            "opening_best",
            CAT_OPENINGS,
            "Лучший дебют".to_string(),
            format!("{name} — {pct}% побед за {n} партий."),
            "good",
            confidence_from_n(n),
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Используй этот дебют в решающих партиях.".to_string()),
            &opening_subject(&name),
            72,
            json!({ "opening": name, "pct": pct, "n": n }),
        ));
    }

    // 2. Worst among frequent (n >= 10, share >= 8% of opening games)
    let mut worst_freq: Option<(String, f64, i64, f64)> = None;
    for (name, st) in &map {
        if st.games < 10 {
            continue;
        }
        let share = st.games as f64 / share_denom;
        if share < 0.08 {
            continue;
        }
        let wr = win_rate(st);
        match &worst_freq {
            None => worst_freq = Some((name.clone(), wr, st.games, share)),
            Some((_, w, _, _)) if wr < *w => {
                worst_freq = Some((name.clone(), wr, st.games, share))
            }
            _ => {}
        }
    }
    if let Some((name, wr, n, share)) = worst_freq {
        let pct = (wr * 100.0).round();
        let share_pct = (share * 100.0).round();
        out.push(build_insight(
            format!("opening_worst_frequent_{user_id}"),
            user_id,
            "opening_worst_frequent",
            CAT_OPENINGS,
            "Худший из частых дебютов".to_string(),
            format!(
                "{name} — {pct}% при {n} партиях (~{share_pct}% всех партий с дебютом)."
            ),
            "warning",
            confidence_from_n(n),
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Разбери типовые ошибки в этом дебюте.".to_string()),
            &opening_subject(&name),
            88,
            json!({
                "opening": name,
                "pct": pct,
                "n": n,
                "share_pct": share_pct
            }),
        ));
    }

    // 3. Rare gem: n < 10, wr > 70%
    let mut gem: Option<(String, f64, i64)> = None;
    for (name, st) in &map {
        if st.games >= 10 {
            continue;
        }
        let wr = win_rate(st);
        if wr <= 0.7 {
            continue;
        }
        match &gem {
            None => gem = Some((name.clone(), wr, st.games)),
            Some((_, _, gn)) if st.games > *gn => gem = Some((name.clone(), wr, st.games)),
            _ => {}
        }
    }
    if let Some((name, wr, n)) = gem {
        let pct = (wr * 100.0).round();
        let conf = rare_gem_confidence(n);
        out.push(build_insight(
            format!("opening_rare_gem_{user_id}"),
            user_id,
            "opening_rare_gem",
            CAT_OPENINGS,
            "Редкий дебют-находка".to_string(),
            format!(
                "{name} — {pct}% за {n} партий. Малая выборка — трактуй осторожно."
            ),
            "good",
            conf,
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Сыграй больше партий в этой линии, чтобы подтвердить сигнал.".to_string()),
            &opening_subject(&name),
            45,
            json!({
                "opening": name,
                "pct": pct,
                "n": n,
                "low_sample": true
            }),
        ));
    }

    // 4. Dependency: one opening > 40% of all games (with opening tag) + strength assessment
    let mut top: Option<(String, i64)> = None;
    for (name, st) in &map {
        match &top {
            None => top = Some((name.clone(), st.games)),
            Some((_, g)) if st.games > *g => top = Some((name.clone(), st.games)),
            _ => {}
        }
    }
    if let Some((name, n)) = top {
        let share = n as f64 / share_denom;
        if share > 0.4 {
            let share_pct = (share * 100.0).round();
            let st = map.get(&name).unwrap();
            let wr = win_rate(st);
            let wr_pct = (wr * 100.0).round() as i64;
            let strength = if wr >= overall_wr + 0.08 || wr >= 0.55 {
                "strength"
            } else if wr <= overall_wr - 0.08 || wr <= 0.45 {
                "risk"
            } else {
                "neutral"
            };
            let (severity, title, recommendation) = match strength {
                "strength" => (
                    "good",
                    "Дебют — твой козырь".to_string(),
                    "Опирайся на эту систему в важных партиях.".to_string(),
                ),
                "risk" => (
                    "warning",
                    "Опасная дебютная зависимость".to_string(),
                    "Часто играешь эту систему при низком винрейте — расширь репертуар и разбер ошибки.".to_string(),
                ),
                _ => (
                    "info",
                    "Дебютная зависимость".to_string(),
                    "Расширь репертуар, чтобы не быть предсказуемым.".to_string(),
                ),
            };
            out.push(build_insight(
                format!("opening_dependency_{user_id}"),
                user_id,
                "opening_dependency",
                CAT_OPENINGS,
                title,
                format!(
                    "{name} — {share_pct}% всех партий с дебютом ({n} игр), винрейт {wr_pct}% (база по всем дебютам: {overall_pct}%)."
                ),
                severity,
                80,
                Some("Доля партий".to_string()),
                Some(format!("{share_pct}%")),
                Some(share_pct as f64),
                Some(recommendation),
                "opening_dependency",
                100,
                json!({
                    "opening": name,
                    "share_pct": share_pct,
                    "n": n,
                    "wr_pct": wr_pct,
                    "overall_pct": overall_pct,
                    "strength": strength
                }),
            ));
        }
    }

    if let Some(ins) = opening_color_split_insight(user_id, games) {
        out.push(ins);
    }

    if let Some(ins) = opening_trend_insight(user_id, games, now_ms()) {
        out.push(ins);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::games::model::Game;

    fn minimal_game(
        id: &str,
        opening: &str,
        color: &str,
        result: &str,
        created_at: i64,
    ) -> Game {
        Game {
            id: id.to_string(),
            username: "u".to_string(),
            platform: "Lichess".to_string(),
            rated: true,
            speed: "blitz".to_string(),
            time_control: "180+0".to_string(),
            created_at,
            player_name: "p".to_string(),
            player_id: "p".to_string(),
            opponent_name: "o".to_string(),
            opponent_id: "o".to_string(),
            white_name: "p".to_string(),
            white_id: "p".to_string(),
            black_name: "o".to_string(),
            black_id: "o".to_string(),
            white_rating: Some(2000),
            black_rating: Some(2000),
            player_rating: Some(2000),
            opponent_rating: Some(2000),
            winner: if result == "draw" {
                None
            } else if result == "win" {
                Some(
                    if color == "white" {
                        "white"
                    } else {
                        "black"
                    }
                    .to_string(),
                )
            } else {
                Some(
                    if color == "white" {
                        "black"
                    } else {
                        "white"
                    }
                    .to_string(),
                )
            },
            player_color: color.to_string(),
            player_result: result.to_string(),
            opening_eco: None,
            opening_name: Some(opening.to_string()),
            moves: None,
            last_fen: None,
            pgn: String::new(),
        }
    }

    #[test]
    fn opening_color_split_detects_gap() {
        let t0 = 1_700_000_000_000i64;
        let mut games = Vec::new();
        for i in 0..6 {
            games.push(minimal_game(
                &format!("w{i}"),
                "Sicilian Defense",
                "white",
                "win",
                t0 + i,
            ));
        }
        for i in 0..6 {
            games.push(minimal_game(
                &format!("b{i}"),
                "Sicilian Defense",
                "black",
                "loss",
                t0 + 100 + i,
            ));
        }
        for i in 0..10 {
            games.push(minimal_game(
                &format!("f{i}"),
                "French Defense",
                "white",
                "draw",
                t0 + 200 + i,
            ));
        }
        let ins = opening_color_split_insight("user1", &games).expect("insight");
        assert_eq!(ins.kind, "opening_color_split");
        assert!(ins.payload_json.as_deref().unwrap().contains("Sicilian"));
    }

    #[test]
    fn opening_trend_detects_swing() {
        let now = 1_800_000_000_000i64;
        let day = MS_DAY;
        let mut games: Vec<Game> = Vec::new();
        let opening = "Caro-Kann Defense";
        // Prev window: 40–10 days ago — 5 wins / 5 games = 100%
        for i in 0..5 {
            games.push(minimal_game(
                &format!("p{i}"),
                opening,
                "white",
                "win",
                now - 40 * day + i * day,
            ));
        }
        // Recent window: 5 losses / 5 games = 0%
        for i in 0..5 {
            games.push(minimal_game(
                &format!("r{i}"),
                opening,
                "white",
                "loss",
                now - 20 * day + i * day,
            ));
        }
        // Filler with other opening so games_with_opening >= 10
        for i in 0..10 {
            games.push(minimal_game(
                &format!("x{i}"),
                "Other",
                "black",
                "win",
                now - 15 * day + i,
            ));
        }
        let ins = opening_trend_insight("user1", &games, now).expect("trend");
        assert_eq!(ins.kind, "opening_trend");
        assert!(ins.payload_json.as_deref().unwrap().contains("Caro"));
    }
}
