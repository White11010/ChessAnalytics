use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut late_game_blunders = 0;
    let mut total_games = 0;

    println!("{}", games.len());
    for game in games {
        total_games += 1;

        if is_late_game_throw(game) {
            late_game_blunders += 1;
        }
    }

    if total_games < 15 || late_game_blunders == 0 {
        return vec![];
    }

    let rate = (late_game_blunders as f64 / total_games as f64) * 100.0;

    vec![Insight {
        id: format!("late-blunders-{}", user_id),

        user_id: user_id.to_string(),

        kind: "blunder_moments".to_string(),

        title: "Ошибки в поздней стадии партии".to_string(),

        summary: format!(
            "В {} из {} партий ты терял позиции после 20 хода.",
            late_game_blunders, total_games
        ),

        severity: "warning".to_string(),

        confidence: confidence(total_games),

        metric_label: Some("Frequency".to_string()),

        metric_value: Some(format!("{:.0}%", rate)),

        recommendation: Some("Тренируй эндшпили и conversion выигранных позиций.".to_string()),

        payload_json: None,

        created_at: now(),

        expires_at: None,
    }]
}

fn is_late_game_throw(game: &Game) -> bool {
    let moves_count = game
        .moves
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .count();

    let lost = game.player_result == "loss";

    moves_count >= 40 && lost
}

fn confidence(games: i64) -> i64 {
    if games >= 100 {
        95
    } else if games >= 50 {
        85
    } else {
        70
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
