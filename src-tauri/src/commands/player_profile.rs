use tauri::AppHandle;

use crate::services::player_profile;

#[tauri::command]
pub fn get_player_profile_chart(app: AppHandle, speed: String) -> Result<player_profile::PlayerProfileChartResponse, String> {
    player_profile::get_player_profile_chart(&app, speed)
}
