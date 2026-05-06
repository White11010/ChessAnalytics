mod clients;
mod commands;
mod db;
mod parsers;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .setup(|app| {
            db::connection::init(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::save_token,
            commands::auth::load_token,
            commands::auth::delete_token,
            commands::users::get_me,
            commands::users::sync_me,
            commands::app::bootstrap,
            commands::games::sync_games,
            commands::games::get_games,
            commands::games::refresh_games_background,
            commands::insights::get_insights,
            commands::insights::regenerate_insights,
            commands::engine::init_engine,
            commands::engine::analyze_position,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
