use tauri::AppHandle;

use crate::db::connection::get_conn;
use crate::db::games::repository as games_repository;
use crate::db::insights::model::Insight;
use crate::db::insights::repository as insights_repository;
use crate::db::users::repository as users_repository;

use super::generators::{blunder_moments, blunder_patterns, openings, time_controls};

pub fn regenerate_for_active_user(app: AppHandle) -> Result<Vec<Insight>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?.ok_or("Active user not found")?;

    let games = games_repository::get_games_by_username(&conn, &user.username, 1000)
        .map_err(|e| e.to_string())?;
    println!("{}", games.len());

    let mut insights = Vec::new();

    insights.extend(openings::generate(&user.id, &games));
    insights.extend(time_controls::generate(&user.id, &games));
    insights.extend(blunder_patterns::generate(&user.id, &games));
    insights.extend(blunder_moments::generate(&user.id, &games));

    insights_repository::replace_user_insights(&conn, &user.id, &insights)
        .map_err(|e| e.to_string())?;

    Ok(insights)
}

pub fn get_for_active_user(app: AppHandle) -> Result<Vec<Insight>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?.ok_or("Active user not found")?;

    insights_repository::get_user_insights(&conn, &user.id).map_err(|e| e.to_string())
}
