// src-tauri/src/clients/lichess.rs

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::services::auth;

/// Prefix for `sync_games` errors when Lichess returns HTTP 429 (frontend schedules retry).
pub const LICHESS_RATE_LIMITED: &str = "RATE_LIMITED:";
/// Prefix when token is rejected (do not treat as rate limit).
pub const LICHESS_UNAUTHORIZED: &str = "UNAUTHORIZED:";

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
/// When `since_ms` is set, only games created at or after that timestamp (ms) are returned.
/// When `max_games` is set, limits how many games the API returns (e.g. 500 for Versus).
pub async fn fetch_games(
    app: &AppHandle,
    username: &str,
    since_ms: Option<i64>,
    max_games: Option<u32>,
) -> Result<String, String> {
    let token = auth::load_token(app)?.ok_or("Token not found")?;

    let mut url = format!(
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

    if let Some(since) = since_ms {
        url.push_str(&format!("&since={since}"));
    }
    if let Some(max) = max_games {
        url.push_str(&format!("&max={max}"));
    }

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(format!(
            "{LICHESS_RATE_LIMITED} Lichess rate limit (429)"
        ));
    }
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(format!(
            "{LICHESS_UNAUTHORIZED} Invalid or expired token"
        ));
    }
    if !status.is_success() {
        return Err(format!("Lichess Games API error: {}", status));
    }

    response.text().await.map_err(|e| e.to_string())
}
