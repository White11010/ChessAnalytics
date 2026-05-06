// src-tauri/src/clients/lichess.rs

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::services::auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perf {
    pub games: Option<i64>,
    pub rating: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perfs {
    pub bullet: Option<Perf>,
    pub blitz: Option<Perf>,
    pub rapid: Option<Perf>,
    pub classical: Option<Perf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LichessProfile {
    pub id: String,
    pub username: String,
    pub perfs: Option<Perfs>,
}

/// GET /api/account
pub async fn fetch_me(app: &AppHandle) -> Result<LichessProfile, String> {
    let token = auth::load_token(app)?.ok_or("Token not found")?;

    let client = reqwest::Client::new();

    let response = client
        .get("https://lichess.org/api/account")
        .header("Accept", "application/json")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Lichess API error: {}", response.status()));
    }

    response
        .json::<LichessProfile>()
        .await
        .map_err(|e| e.to_string())
}

/// GET /api/games/user/{username}
pub async fn fetch_games(app: &AppHandle, username: &str) -> Result<String, String> {
    let token = auth::load_token(app)?.ok_or("Token not found")?;

    let url = format!(
        concat!(
            "https://lichess.org/api/games/user/{}",
            "?moves=true",
            "&pgnInJson=true",
            "&evals=true",
            "&accuracy=true",
            "&opening=true",
            "&lastFen=true"
        ),
        username
    );

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("Accept", "application/x-ndjson")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Lichess Games API error: {}", response.status()));
    }

    response.text().await.map_err(|e| e.to_string())
}
