// Win-rate splits when opponent rating is weaker / equal / stronger than the player (±50 rating band buckets).
use serde_json::json;

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, CAT_TACTICS};

#[derive(Default, Clone)]
struct Bucket {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

fn win_rate(b: &Bucket) -> f64 {
    if b.games == 0 {
        return 0.0;
    }
    (b.wins as f64 + b.draws as f64 * 0.5) / b.games as f64
}

/// Emits a card when one rating bucket shows a materially better win rate than the others (sample gates in-body).
pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut weaker = Bucket::default();
    let mut equal = Bucket::default();
    let mut stronger = Bucket::default();

    for g in games {
        let (Some(pr), Some(or)) = (g.player_rating, g.opponent_rating) else {
            continue;
        };
        let d = or - pr;
        let b = if d < -50 {
            &mut weaker
        } else if d > 50 {
            &mut stronger
        } else {
            &mut equal
        };
        b.games += 1;
        match g.player_result.as_str() {
            "win" => b.wins += 1,
            "loss" => b.losses += 1,
            _ => b.draws += 1,
        }
    }

    if weaker.games < 15 || equal.games < 15 || stronger.games < 15 {
        return vec![];
    }

    let wr_w = win_rate(&weaker);
    let wr_e = win_rate(&equal);
    let wr_s = win_rate(&stronger);
    let gap = (wr_e - wr_s) * 100.0;
    if gap < 12.0 {
        return vec![];
    }

    let p_w = (wr_w * 100.0).round() as i64;
    let p_e = (wr_e * 100.0).round() as i64;
    let p_s = (wr_s * 100.0).round() as i64;
    let gap_r = gap.round() as i64;

    vec![build_insight(
        format!("opp_rating_{user_id}"),
        user_id,
        "opponent_rating_performance",
        CAT_TACTICS,
        "Игра по силе соперника".to_string(),
        format!(
            "Винрейт: против слабее (−50 и ниже) {p_w}% ({nw} игр), в паритет ±50 — {p_e}% ({ne} игр), против сильнее (+50) {p_s}% ({ns} игр). Просадка к «сильным»: {gap_r} п.п.",
            nw = weaker.games,
            ne = equal.games,
            ns = stronger.games,
        ),
        if gap_r >= 25 { "warning" } else { "info" },
        80,
        None,
        None,
        None,
        Some("Добавь игру против чуть более сильных и разбор ключевых партий.".to_string()),
        "tactics:opponent_rating_buckets",
        73,
        json!({
            "wr_weaker_pct": p_w,
            "wr_equal_pct": p_e,
            "wr_stronger_pct": p_s,
            "n_weaker": weaker.games,
            "n_equal": equal.games,
            "n_stronger": stronger.games,
            "gap_equal_vs_stronger_pp": gap_r,
            "rating_band": 50
        }),
    )]
}
