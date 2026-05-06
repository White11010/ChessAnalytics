use tauri::AppHandle;

use crate::db::games::model::Game;
use crate::services::games;

#[tauri::command]
pub async fn sync_games(app: AppHandle) -> Result<games::SyncGamesResult, String> {
    games::sync_games(app).await
}

#[tauri::command]
pub fn get_games(app: AppHandle, limit: Option<u32>) -> Result<Vec<Game>, String> {
    games::get_my_games(app, limit.unwrap_or(100000))
}

#[tauri::command]
pub fn refresh_games_background(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        let _ = games::sync_games(app).await;
    });

    Ok(())
}
