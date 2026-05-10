use std::time::{SystemTime, UNIX_EPOCH};

use tauri::AppHandle;

use crate::clients::lichess;
use crate::db::connection::get_conn;
use crate::db::games::model::{Game, GameListItem};
use crate::db::games::repository;
use crate::db::users::repository as users_repository;
use crate::parsers::lichess_games;

#[derive(Debug, serde::Serialize)]
pub struct SyncGamesResult {
    pub inserted: u32,
    pub username: String,
    pub last_sync_completed_at_ms: i64,
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn combine_lichess_cursor(previous: Option<i64>, batch_max_created: Option<i64>) -> Option<i64> {
    match (previous, batch_max_created) {
        (None, None) => None,
        (None, Some(b)) => Some(b),
        (Some(p), None) => Some(p),
        (Some(p), Some(b)) => Some(p.max(b)),
    }
}

pub async fn sync_games(app: AppHandle) -> Result<SyncGamesResult, String> {
    let mut conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?
        .ok_or("Active user not found")?;

    let since = user.lichess_since_cursor_ms;

    let ndjson = lichess::fetch_games(&app, &user.username, since, None).await?;

    let games = lichess_games::parse_ndjson(&user.username, &ndjson);
    let batch_max_created = games.iter().map(|g| g.created_at).max();
    let new_cursor = combine_lichess_cursor(user.lichess_since_cursor_ms, batch_max_created);

    let now_ms = now_millis();

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let inserted = persist_games_in_tx(&tx, games)?;
    users_repository::update_user_games_sync_metadata(&tx, &user.id, new_cursor, now_ms)?;
    tx.commit().map_err(|e| e.to_string())?;

    Ok(SyncGamesResult {
        inserted,
        username: user.username,
        last_sync_completed_at_ms: now_ms,
    })
}

pub fn get_my_games(
    app: AppHandle,
    limit: u32,
) -> Result<Vec<GameListItem>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?
        .ok_or("Active user not found")?;

    repository::get_game_list_items(
        &conn,
        &user.username,
        &user.id,
        limit,
    )
    .map_err(|e| e.to_string())
}

fn persist_games_in_tx(
    tx: &rusqlite::Transaction<'_>,
    games: Vec<Game>,
) -> Result<u32, String> {
    let mut inserted = 0;

    for game in games {
        let rows = repository::upsert_game(tx, &game).map_err(|e| e.to_string())?;

        if rows > 0 {
            inserted += 1;
        }
    }

    Ok(inserted)
}
