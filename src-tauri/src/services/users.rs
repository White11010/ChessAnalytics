use tauri::AppHandle;

use crate::clients::lichess;
use crate::db::connection::get_conn;
use crate::db::users::model::User;
use crate::db::users::repository;

pub async fn sync_user(app: AppHandle) -> Result<User, String> {
    let profile = lichess::fetch_me(&app).await?;

    let user = User::from_lichess(profile);

    let conn = get_conn(&app)?;

    repository::upsert_user(&conn, &user)?;
    repository::set_active_user(&conn, &user.id)?;

    repository::get_active_user(&conn)?
        .ok_or_else(|| "Active user not found after sync".to_string())
}

pub fn get_me(app: &AppHandle) -> Result<Option<User>, String> {
    let conn = get_conn(app)?;
    repository::get_active_user(&conn)
}

pub async fn sync_me(app: &AppHandle) -> Result<User, String> {
    let profile = lichess::fetch_me(app).await?;

    let user = User::from_lichess(profile);

    let conn = get_conn(app)?;
    repository::upsert_user(&conn, &user)?;
    repository::set_active_user(&conn, &user.id)?;

    repository::get_active_user(&conn)?
        .ok_or_else(|| "Active user not found after sync".to_string())
}
