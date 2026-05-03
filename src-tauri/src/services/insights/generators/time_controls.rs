use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;

#[derive(Default)]
struct Stats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut map: HashMap<String, Stats> = HashMap::new();

    for game in games {
        let control = normalize(&game.speed);

        let stats = map.entry(control).or_default();

        stats.games += 1;

        match game.player_result.as_str() {
            "win" => stats.wins += 1,
            "loss" => stats.losses += 1,
            _ => stats.draws += 1,
        }
    }

    let mut insights = Vec::new();

    let mut best: Option<(String, f64, i64)> = None;
    let mut worst: Option<(String, f64, i64)> = None;

    for (control, s) in map {
        if s.games < 10 {
            continue;
        }

        let score = (s.wins as f64 + s.draws as f64 * 0.5) / s.games as f64;

        match &best {
            None => best = Some((control.clone(), score, s.games)),
            Some((_, cur, _)) if score > *cur => best = Some((control.clone(), score, s.games)),
            _ => {}
        }

        match &worst {
            None => worst = Some((control.clone(), score, s.games)),
            Some((_, cur, _)) if score < *cur => worst = Some((control.clone(), score, s.games)),
            _ => {}
        }
    }

    if let Some((ctrl, score, games)) = best {
        insights.push(make_best(user_id, &ctrl, score, games));
    }

    if let Some((ctrl, score, games)) = worst {
        insights.push(make_worst(user_id, &ctrl, score, games));
    }

    insights
}

fn normalize(speed: &str) -> String {
    match speed {
        "bullet" => "Bullet".to_string(),
        "blitz" => "Blitz".to_string(),
        "rapid" => "Rapid".to_string(),
        "classical" => "Classical".to_string(),
        other => other.to_string(),
    }
}

fn make_best(user_id: &str, control: &str, score: f64, games: i64) -> Insight {
    Insight {
        id: format!("best-timecontrol-{}", user_id),

        user_id: user_id.to_string(),

        kind: "best_time_control".to_string(),

        title: "Лучший контроль времени".to_string(),

        summary: format!("Ты сильнее всего играешь в {}", control),

        severity: "good".to_string(),

        confidence: confidence(games),

        metric_label: Some("Winrate".to_string()),
        metric_value: Some(format!("{:.0}% ({} игр)", score * 100.0, games)),

        recommendation: Some("Используй этот формат для рейтинговых игр.".to_string()),

        payload_json: None,

        created_at: now(),
        expires_at: None,
    }
}

fn make_worst(user_id: &str, control: &str, score: f64, games: i64) -> Insight {
    Insight {
        id: format!("worst-timecontrol-{}", user_id),

        user_id: user_id.to_string(),

        kind: "worst_time_control".to_string(),

        title: "Слабый контроль времени".to_string(),

        summary: format!("Твои худшие результаты в {}", control),

        severity: "warning".to_string(),

        confidence: confidence(games),

        metric_label: Some("Winrate".to_string()),
        metric_value: Some(format!("{:.0}% ({} игр)", score * 100.0, games)),

        recommendation: Some(
            "Играй больше медленных партий для улучшения качества решений.".to_string(),
        ),

        payload_json: None,

        created_at: now(),
        expires_at: None,
    }
}

fn confidence(games: i64) -> i64 {
    if games >= 50 {
        95
    } else if games >= 25 {
        85
    } else if games >= 10 {
        70
    } else {
        50
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
