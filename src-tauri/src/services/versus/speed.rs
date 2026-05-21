use crate::db::games::model::Game;

/// Normalize Lichess `speed`/`perf.type` buckets to the app's bullet | blitz | rapid filters.
pub(crate) fn game_matches_requested_speed(game_speed: &str, requested: &str) -> bool {
    let gs = game_speed.trim().to_lowercase();
    let rq = requested.trim().to_lowercase();
    if gs.is_empty() {
        return false;
    }
    if gs == rq {
        return true;
    }
    // Lichess splits hyper-bullet vs bullet; players often choose "bullet" in UI for both.
    if rq == "bullet" && gs == "ultrabullet" {
        return true;
    }
    false
}

pub(crate) fn rating_for_speed_u(user: &crate::db::users::model::User, speed: &str) -> Option<i64> {
    match speed {
        "bullet" => user.bullet_rating,
        "blitz" => user.blitz_rating,
        "rapid" => user.rapid_rating,
        _ => None,
    }
}

pub(crate) fn filter_opponent_games_for_speed(parsed: &[Game], speed_lc: &str) -> Vec<Game> {
    let mut opp_games: Vec<Game> = parsed
        .iter()
        .filter(|g| {
            g.rated
                && game_matches_requested_speed(&g.speed, speed_lc)
                && g.moves
                    .as_ref()
                    .map(|m| !m.trim().is_empty())
                    .unwrap_or(false)
        })
        .cloned()
        .collect();
    opp_games.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    opp_games
}
