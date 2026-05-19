// Orchestrates analyze vs persist vs batch worker; bridges Stockfish, classifier, pattern detector, and SQLite repos.
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Utc;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Emitter};

use crate::db::connection::get_conn;
use crate::db::game_analyses::model::{GameAnalysisRow, KeyMomentRow, PatternTagRow};
use crate::db::game_analyses::repository as ga_repo;
use crate::db::games::repository as games_repo;
use crate::db::users::repository as users_repo;
use crate::services::engine::stockfish::{ensure_engine_started, get_engine};

use super::classifier::{
    accuracy_from_acpl, avg_centipawn_loss, classify_player_moves, classify_player_moves_from_eval,
    count_move_kinds, max_min_eval,
};
use super::engine_runner::analyze_eval_history;
use super::key_insight::build_key_insight;
use super::key_moments::pick_key_moments;
use super::model::{GameAnalysisFull, KeyInsight, SimilarGames, SystemConnection};
use super::pattern_detector::detect_patterns;
use super::system_connection::build_system_connection;

/// Minimal metrics for Versus transient runs (no DB write): accuracy, ACPL, adv swing, blunders, pattern tag names.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TransientAnalysisResult {
    pub accuracy: f64,
    pub avg_centipawn_loss: f64,
    pub max_advantage_cp: i32,
    pub min_advantage_cp: i32,
    pub blunders: i32,
    pub pattern_tags: Vec<String>,
}

/// Runs the same eval pass as full analysis but skips persistence; used for opponent games in Versus only.
pub fn analyze_game_transient(
    app: &AppHandle,
    game: &crate::db::games::model::Game,
    depth: u8,
) -> Result<TransientAnalysisResult, String> {
    let moves_str = game.moves.as_deref().unwrap_or("").trim();
    if moves_str.is_empty() {
        return Err("No moves stored for this game".into());
    }

    let uci_moves: Vec<String> = moves_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let player_is_white = game.player_color == "white";

    ensure_engine_started(app)?;

    let global = get_engine();
    let mut guard = global.lock().map_err(|_| "Failed to lock engine mutex")?;
    let engine = guard
        .as_mut()
        .expect("engine present after ensure_engine_started");

    let eval_history = analyze_eval_history(engine, &uci_moves, player_is_white, depth)?;
    let classified = classify_player_moves_from_eval(&uci_moves, &eval_history, player_is_white)?;

    let acpl = avg_centipawn_loss(&classified);
    let accuracy = accuracy_from_acpl(acpl);
    let (b, _m, _i) = count_move_kinds(&classified);
    let (max_adv, min_adv) = max_min_eval(&eval_history);

    let tags = detect_patterns(game, &eval_history, &classified, accuracy, acpl);

    drop(guard);

    Ok(TransientAnalysisResult {
        accuracy,
        avg_centipawn_loss: acpl,
        max_advantage_cp: max_adv,
        min_advantage_cp: min_adv,
        blunders: b,
        pattern_tags: tags.into_iter().map(|(t, _)| t).collect(),
    })
}

static ANALYSIS_CANCEL: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
// Prevents overlapping background threads that would contend on the single global Stockfish mutex and duplicate work.
static BATCH_RUNNING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// Signals the background batch loop to stop after the current game; paired with `analyze_pending_games` worker.
pub fn cancel_pending_analysis() {
    ANALYSIS_CANCEL.store(true, Ordering::SeqCst);
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

/// Tag overlap (broad) and first key-moment kind overlap (narrow) for the game detail similar-games panel.
pub fn get_similar_games(app: &AppHandle, game_id: &str) -> Result<SimilarGames, String> {
    let conn = get_conn(app)?;
    let stored = ga_repo::get_analysis_stored(&conn, game_id)
        .map_err(|e| e.to_string())?
        .ok_or("No analysis for this game")?;

    if stored.analysis.status != "done" {
        return Ok(SimilarGames {
            broad: vec![],
            narrow: vec![],
        });
    }

    let user_id = stored.analysis.user_id.as_str();
    let broad =
        ga_repo::find_similar_by_tags(&conn, user_id, game_id, 20).map_err(|e| e.to_string())?;

    let key_moments: Vec<super::model::KeyMoment> =
        serde_json::from_str(stored.analysis.key_moments_json.as_deref().unwrap_or("[]"))
            .unwrap_or_default();

    // First picked moment is the primary narrative hook; narrow matches reuse that moment type across the library.
    let narrow_kind = key_moments
        .first()
        .map(|m| m.kind.as_str())
        .unwrap_or("blunder");

    let narrow = ga_repo::find_similar_by_moment_kind(&conn, user_id, game_id, narrow_kind, 20)
        .map_err(|e| e.to_string())?;

    Ok(SimilarGames { broad, narrow })
}

/// Loads stored analysis JSON into `GameAnalysisFull`; attaches similar games only when status is `done`.
pub fn get_analysis(app: &AppHandle, game_id: &str) -> Result<Option<GameAnalysisFull>, String> {
    let conn = get_conn(app)?;
    let stored = ga_repo::get_analysis_stored(&conn, game_id).map_err(|e| e.to_string())?;
    let Some(stored) = stored else {
        return Ok(None);
    };

    let similar = if stored.analysis.status == "done" {
        get_similar_games(app, game_id).unwrap_or_else(|_| SimilarGames {
            broad: vec![],
            narrow: vec![],
        })
    } else {
        SimilarGames {
            broad: vec![],
            narrow: vec![],
        }
    };

    let mut full = stored_to_full(stored)?;
    full.similar_games = similar;
    Ok(Some(full))
}

fn stored_to_full(stored: ga_repo::GameAnalysisStored) -> Result<GameAnalysisFull, String> {
    let err = stored.analysis.error.clone();
    Ok(GameAnalysisFull {
        game_id: stored.analysis.game_id.clone(),
        status: stored.analysis.status.clone(),
        depth: stored.analysis.depth as u8,
        key_moments: serde_json::from_str(
            stored.analysis.key_moments_json.as_deref().unwrap_or("[]"),
        )
        .unwrap_or_default(),
        key_insight: serde_json::from_str(
            stored.analysis.key_insight_json.as_deref().unwrap_or("{}"),
        )
        .unwrap_or(KeyInsight {
            title: "…".into(),
            description: "Analysis pending or failed.".into(),
            severity: "info".into(),
            kind: "pending".into(),
        }),
        system_connection: serde_json::from_str(
            stored
                .analysis
                .system_connection_json
                .as_deref()
                .unwrap_or("{}"),
        )
        .unwrap_or(SystemConnection {
            text: String::new(),
            tag: String::new(),
            count: 0,
            window: 10,
            secondary_text: None,
            primary_variant: String::new(),
            secondary_variant: "none".into(),
            secondary_total: 0,
            secondary_wr_pct: 0.0,
        }),
        eval_history: serde_json::from_str(
            stored.analysis.eval_history_json.as_deref().unwrap_or("[]"),
        )
        .unwrap_or_default(),
        accuracy: stored.analysis.accuracy.unwrap_or(0.0),
        avg_centipawn_loss: stored.analysis.avg_centipawn_loss.unwrap_or(0.0),
        max_advantage_cp: stored.analysis.max_advantage_cp.unwrap_or(0) as i32,
        min_advantage_cp: stored.analysis.min_advantage_cp.unwrap_or(0) as i32,
        blunders: stored.analysis.blunders.unwrap_or(0) as i32,
        mistakes: stored.analysis.mistakes.unwrap_or(0) as i32,
        inaccuracies: stored.analysis.inaccuracies.unwrap_or(0) as i32,
        pattern_tags: stored.tags.iter().map(|t| t.tag.clone()).collect(),
        similar_games: SimilarGames {
            broad: vec![],
            narrow: vec![],
        },
        error: err,
    })
}

/// Full persist path for one game: engine run, tags, key moments, upsert row, then second-pass system_connection patch.
pub fn analyze_game(
    app: &AppHandle,
    game_id: &str,
    depth: Option<u8>,
) -> Result<GameAnalysisFull, String> {
    let depth = depth.unwrap_or(14); // Default depth balances quality vs batch time for typical user libraries.
    let conn = get_conn(app)?;
    let user = users_repo::get_active_user(&conn)?.ok_or("Active user not found")?;

    let game = games_repo::get_game_by_id(&conn, game_id)
        .map_err(|e| e.to_string())?
        .ok_or("Game not found")?;

    if game.username != user.username {
        return Err("Game does not belong to active user".into());
    }

    let moves_str = game.moves.as_deref().unwrap_or("").trim();
    if moves_str.is_empty() {
        return persist_failed(
            &conn,
            game_id,
            &user.id,
            depth,
            "No moves stored for this game",
        );
    }

    let uci_moves: Vec<String> = moves_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let player_is_white = game.player_color == "white";

    ensure_engine_started(app)?;

    let global = get_engine();
    let mut guard = global.lock().map_err(|_| "Failed to lock engine mutex")?;
    let engine = guard
        .as_mut()
        .expect("engine present after ensure_engine_started");

    let eval_history = analyze_eval_history(engine, &uci_moves, player_is_white, depth)?;
    let classified =
        classify_player_moves(engine, &uci_moves, &eval_history, player_is_white, depth)?;

    let acpl = avg_centipawn_loss(&classified);
    let accuracy = accuracy_from_acpl(acpl);
    let (b, m, i) = count_move_kinds(&classified);
    let (max_adv, min_adv) = max_min_eval(&eval_history);

    let tags = detect_patterns(&game, &eval_history, &classified, accuracy, acpl);
    let key_moments = pick_key_moments(&uci_moves, &classified, 3)?;
    let key_insight = build_key_insight(&game, &tags, &classified, max_adv);

    let eval_json = serde_json::to_string(&eval_history).map_err(|e| e.to_string())?;
    let moments_json = serde_json::to_string(&key_moments).map_err(|e| e.to_string())?;
    let insight_json = serde_json::to_string(&key_insight).map_err(|e| e.to_string())?;

    let tag_rows: Vec<PatternTagRow> = tags
        .iter()
        .map(|(tag, weight)| PatternTagRow {
            game_id: game_id.to_string(),
            user_id: user.id.clone(),
            tag: tag.clone(),
            weight: *weight,
        })
        .collect();

    let moment_rows: Vec<KeyMomentRow> = key_moments
        .iter()
        .map(|km| KeyMomentRow {
            game_id: game_id.to_string(),
            user_id: user.id.clone(),
            ply: km.ply,
            kind: km.kind.clone(),
            swing_cp: i64::from(km.swing_cp),
        })
        .collect();

    let ts = now_ms();
    let row = GameAnalysisRow {
        game_id: game_id.to_string(),
        user_id: user.id.clone(),
        status: "done".into(),
        depth: depth as i64,
        accuracy: Some(accuracy),
        avg_centipawn_loss: Some(acpl),
        max_advantage_cp: Some(max_adv as i64),
        min_advantage_cp: Some(min_adv as i64),
        blunders: Some(b as i64),
        mistakes: Some(m as i64),
        inaccuracies: Some(i as i64),
        eval_history_json: Some(eval_json),
        key_moments_json: Some(moments_json),
        key_insight_json: Some(insight_json),
        system_connection_json: None,
        created_at: ts,
        updated_at: ts,
        error: None,
    };

    ga_repo::upsert_analysis(&conn, &row, &tag_rows, &moment_rows).map_err(|e| e.to_string())?;

    // System connection queries similar games in DB after base upsert so tags exist for correlation counts.
    let system = build_system_connection(&conn, &user.username, &user.id, &tags)?;
    let system_json = serde_json::to_string(&system).map_err(|e| e.to_string())?;
    ga_repo::update_system_connection_json(&conn, game_id, &system_json, now_ms())
        .map_err(|e| e.to_string())?;

    drop(guard);

    let similar = get_similar_games(app, game_id).unwrap_or_else(|_| SimilarGames {
        broad: vec![],
        narrow: vec![],
    });

    Ok(GameAnalysisFull {
        game_id: game_id.to_string(),
        status: "done".into(),
        depth,
        key_moments,
        key_insight,
        system_connection: system,
        eval_history,
        accuracy,
        avg_centipawn_loss: acpl,
        max_advantage_cp: max_adv,
        min_advantage_cp: min_adv,
        blunders: b,
        mistakes: m,
        inaccuracies: i,
        pattern_tags: tags.into_iter().map(|(t, _)| t).collect(),
        similar_games: similar,
        error: None,
    })
}

fn persist_failed(
    conn: &rusqlite::Connection,
    game_id: &str,
    user_id: &str,
    depth: u8,
    msg: &str,
) -> Result<GameAnalysisFull, String> {
    let ts = now_ms();
    let row = GameAnalysisRow {
        game_id: game_id.to_string(),
        user_id: user_id.to_string(),
        status: "failed".into(),
        depth: depth as i64,
        accuracy: None,
        avg_centipawn_loss: None,
        max_advantage_cp: None,
        min_advantage_cp: None,
        blunders: None,
        mistakes: None,
        inaccuracies: None,
        eval_history_json: None,
        key_moments_json: None,
        key_insight_json: None,
        system_connection_json: None,
        created_at: ts,
        updated_at: ts,
        error: Some(msg.to_string()),
    };
    ga_repo::upsert_analysis(conn, &row, &[], &[]).map_err(|e| e.to_string())?;
    Err(msg.to_string())
}

/// Spawns a background thread to analyze up to 10k pending ids with progress events; no-op if queue empty or busy.
pub fn analyze_pending_games(app: AppHandle, depth: Option<u8>) -> Result<(), String> {
    let conn = get_conn(&app)?;
    let user = users_repo::get_active_user(&conn)?.ok_or("Active user not found")?;
    let total = ga_repo::count_pending_games(&conn, &user.username).map_err(|e| e.to_string())?;

    if total == 0 {
        let _ = app.emit(
            "game-analysis://done",
            serde_json::json!({ "processed": 0, "total": 0 }),
        );
        return Ok(());
    }

    // High cap drains backlog in one run; UI still cancels mid-loop; oldest-first ordering avoids starving old games.
    let pending =
        ga_repo::get_pending_game_ids(&conn, &user.username, 10_000).map_err(|e| e.to_string())?;

    if BATCH_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        let _ = app.emit("game-analysis://batch-busy", serde_json::json!({}));
        return Err(
            "Game analysis batch is already running. Wait for it to finish or cancel it.".into(),
        );
    }

    ANALYSIS_CANCEL.store(false, Ordering::SeqCst);

    std::thread::spawn(move || {
        struct ClearBatchRunning;
        impl Drop for ClearBatchRunning {
            fn drop(&mut self) {
                BATCH_RUNNING.store(false, Ordering::SeqCst);
            }
        }
        let _clear_running = ClearBatchRunning;

        let depth = depth.unwrap_or(14);
        let mut done = 0u32;
        for game_id in pending {
            if ANALYSIS_CANCEL.load(Ordering::SeqCst) {
                break;
            }
            let gid = game_id.clone();
            let _ = app.emit(
                "game-analysis://analyzing",
                serde_json::json!({ "game_id": gid.clone() }),
            );
            let res = analyze_game(&app, &gid, Some(depth)).ok();

            done += 1;
            let _ = app.emit(
                "game-analysis://progress",
                serde_json::json!({
                    "current": done,
                    "total": total,
                    "game_id": gid,
                    "ok": res.is_some(),
                }),
            );
        }

        let _ = app.emit(
            "game-analysis://done",
            serde_json::json!({ "processed": done, "total": total }),
        );
    });

    Ok(())
}
