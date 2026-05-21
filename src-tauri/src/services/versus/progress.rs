use tauri::{AppHandle, Emitter};

pub(crate) fn emit_prog(app: &AppHandle, phase: &str, current: u32, total: u32) {
    let _ = app.emit(
        "versus://progress",
        serde_json::json!({
            "phase": phase,
            "current": current,
            "total": total,
        }),
    );
}
