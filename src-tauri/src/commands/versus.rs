use tauri::AppHandle;

use crate::services::versus;

#[tauri::command]
pub async fn versus_compare(
    app: AppHandle,
    opponent_username: String,
) -> Result<versus::VersusCompareResponse, String> {
    versus::versus_compare(app, opponent_username).await
}

#[tauri::command]
pub fn versus_cancel_compare() {
    versus::cancel_versus_compare();
}
