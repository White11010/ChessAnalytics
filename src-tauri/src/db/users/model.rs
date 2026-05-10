use serde::{Deserialize, Serialize};

use crate::clients::lichess::LichessProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub platform: String,
    pub username: String,

    pub bullet_rating: Option<i64>,
    pub bullet_games: Option<i64>,

    pub blitz_rating: Option<i64>,
    pub blitz_games: Option<i64>,

    pub rapid_rating: Option<i64>,
    pub rapid_games: Option<i64>,

    pub classical_rating: Option<i64>,
    pub classical_games: Option<i64>,

    /// Max `created_at` from last successfully imported Lichess batch; used as `since` for next export.
    pub lichess_since_cursor_ms: Option<i64>,
    /// Wall-clock ms when last games sync finished successfully (for UI "updated N min ago").
    pub last_sync_completed_at_ms: Option<i64>,
}

impl User {
    pub fn from_lichess(data: LichessProfile) -> Self {
        let perfs = data.perfs;

        let bullet = perfs.as_ref().and_then(|p| p.bullet.as_ref());
        let blitz = perfs.as_ref().and_then(|p| p.blitz.as_ref());
        let rapid = perfs.as_ref().and_then(|p| p.rapid.as_ref());
        let classical = perfs.as_ref().and_then(|p| p.classical.as_ref());

        Self {
            id: data.id,
            platform: "lichess".to_string(),
            username: data.username,

            bullet_rating: bullet.and_then(|x| x.rating),
            bullet_games: bullet.and_then(|x| x.games),

            blitz_rating: blitz.and_then(|x| x.rating),
            blitz_games: blitz.and_then(|x| x.games),

            rapid_rating: rapid.and_then(|x| x.rating),
            rapid_games: rapid.and_then(|x| x.games),

            classical_rating: classical.and_then(|x| x.rating),
            classical_games: classical.and_then(|x| x.games),

            lichess_since_cursor_ms: None,
            last_sync_completed_at_ms: None,
        }
    }
}
