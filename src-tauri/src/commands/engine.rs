use tauri::AppHandle;

use crate::services::engine::stockfish::{get_engine, EngineResult, StockfishEngine};

#[tauri::command]
pub fn init_engine(app: AppHandle) -> Result<(), String> {
    let engine = StockfishEngine::new(&app)?;

    let global = get_engine();
    let mut guard = global.lock().map_err(|_| "Failed to lock engine mutex")?;

    *guard = Some(engine);

    Ok(())
}

#[tauri::command]
pub fn analyze_position(fen: String, depth: Option<u8>) -> Result<EngineResult, String> {
    let global = get_engine();

    let mut guard = global.lock().map_err(|_| "Failed to lock engine mutex")?;

    let engine = guard.as_mut().ok_or("Engine not initialized")?;

    engine.analyze(&fen, depth.unwrap_or(12))
}
