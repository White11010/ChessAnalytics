mod clients;
mod commands;
mod db;
mod parsers;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
            commands::player_profile::get_player_profile_chart,
            commands::insights::get_insights,
            commands::insights::get_daily_insight,
            commands::insights::regenerate_insights,
            commands::engine::init_engine,
            commands::engine::analyze_position,
            commands::game_analysis::analyze_game,
            commands::game_analysis::get_game_analysis,
            commands::game_analysis::analyze_pending_games,
            commands::game_analysis::cancel_pending_analysis,
            commands::game_analysis::get_similar_games,
            commands::versus::versus_compare,
            commands::versus::versus_cancel_compare,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
