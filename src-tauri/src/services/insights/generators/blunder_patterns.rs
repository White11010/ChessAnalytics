use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;

#[derive(Default)]
struct SideStats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut white = SideStats::default();
    let mut black = SideStats::default();

    for game in games {
        match game.player_color.as_str() {
            "white" => accumulate(&mut white, &game.player_result),
            "black" => accumulate(&mut black, &game.player_result),
            _ => {}
        }
    }

    let mut insights = Vec::new();

    if white.games > 10 && black.games > 10 {
        let white_wr = winrate(&white);
        let black_wr = winrate(&black);

        if (white_wr - black_wr).abs() >= 0.08 {
            if white_wr > black_wr {
                insights.push(make_good_white(
                    user_id,
                    white_wr,
                    black_wr,
                    white.games,
                    black.games,
                ));
            } else {
                insights.push(make_bad_black(
                    user_id,
                    white_wr,
                    black_wr,
                    white.games,
                    black.games,
                ));
            }
        }
    }

    insights
}

fn accumulate(stats: &mut SideStats, result: &str) {
    stats.games += 1;

    match result {
        "win" => stats.wins += 1,
        "loss" => stats.losses += 1,
        _ => stats.draws += 1,
    }
}

fn winrate(s: &SideStats) -> f64 {
    if s.games == 0 {
        return 0.0;
    }

    (s.wins as f64 + s.draws as f64 * 0.5) / s.games as f64
}

fn make_good_white(
    user_id: &str,
    white_wr: f64,
    black_wr: f64,
    w_games: i64,
    b_games: i64,
) -> Insight {
    Insight {
        id: format!("blunder-white-{}", user_id),

        user_id: user_id.to_string(),

        kind: "side_performance".to_string(),

        title: "Ты сильнее играешь белыми".to_string(),

        summary: format!(
            "White: {:.0}% ({} игр), Black: {:.0}% ({} игр)",
            white_wr * 100.0,
            w_games,
            black_wr * 100.0,
            b_games
        ),

        severity: "info".to_string(),

        confidence: confidence(w_games + b_games),

        metric_label: Some("Winrate gap".to_string()),
        metric_value: Some(format!("+{:.0}%", (white_wr - black_wr) * 100.0)),

        recommendation: Some("Работай над дебютами за чёрных и защитой.".to_string()),

        payload_json: None,

        created_at: now(),
        expires_at: None,
    }
}

fn make_bad_black(
    user_id: &str,
    white_wr: f64,
    black_wr: f64,
    w_games: i64,
    b_games: i64,
) -> Insight {
    Insight {
        id: format!("blunder-black-{}", user_id),

        user_id: user_id.to_string(),

        kind: "side_performance".to_string(),

        title: "Проблема с игрой чёрными".to_string(),

        summary: format!(
            "White: {:.0}%, Black: {:.0}%",
            white_wr * 100.0,
            black_wr * 100.0
        ),

        severity: "warning".to_string(),

        confidence: confidence(w_games + b_games),

        metric_label: Some("Winrate gap".to_string()),
        metric_value: Some(format!("-{:.0}%", (white_wr - black_wr).abs() * 100.0)),

        recommendation: Some("Чёрные требуют отдельной тренировочной фокусировки.".to_string()),

        payload_json: None,

        created_at: now(),
        expires_at: None,
    }
}

fn confidence(games: i64) -> i64 {
    if games >= 80 {
        95
    } else if games >= 40 {
        85
    } else if games >= 20 {
        75
    } else {
        60
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
