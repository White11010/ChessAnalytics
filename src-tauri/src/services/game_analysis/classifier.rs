use shakmaty::fen::Fen;
use shakmaty::{Chess, EnPassantMode};

use crate::services::engine::stockfish::StockfishEngine;

use super::move_token::play_token;

#[derive(Debug, Clone)]
pub struct ClassifiedMove {
    pub half_move_index: usize,
    /// Full-move number (1-based) as shown to user (e.g. white's 23rd half-move = move 12 as pair).
    pub display_move_number: i32,
    pub uci: String,
    pub kind: &'static str,
    pub eval_before: i32,
    pub eval_after: i32,
    /// Player perspective: negative usually means loss of advantage.
    pub swing_cp: i32,
    pub best_move_uci: Option<String>,
}

fn is_player_move(half_move_index: usize, player_is_white: bool) -> bool {
    let white_to_move = half_move_index.is_multiple_of(2);
    white_to_move == player_is_white
}

fn classify_swing(swing: i32) -> &'static str {
    if swing <= -200 {
        "blunder"
    } else if swing <= -100 {
        "mistake"
    } else if swing <= -50 {
        "inaccuracy"
    } else {
        "good"
    }
}

fn position_at_prefix(uci_moves: &[String], up_to: usize) -> Result<Chess, String> {
    let mut pos = Chess::new();
    for (i, tok) in uci_moves.iter().take(up_to).enumerate() {
        play_token(&mut pos, tok).map_err(|reason| format!("prefix ply {} `{}`: {reason}", i + 1, tok))?;
    }
    Ok(pos)
}

pub fn classify_player_moves(
    engine: &mut StockfishEngine,
    uci_moves: &[String],
    eval_history: &[i32],
    player_is_white: bool,
    depth: u8,
) -> Result<Vec<ClassifiedMove>, String> {
    if eval_history.len() != uci_moves.len() + 1 {
        return Err("eval_history length mismatch".into());
    }

    let mut out = Vec::new();

    for k in 0..uci_moves.len() {
        if !is_player_move(k, player_is_white) {
            continue;
        }

        let eval_before = eval_history[k];
        let eval_after = eval_history[k + 1];
        let swing = eval_after - eval_before;

        let pos = position_at_prefix(uci_moves, k)?;
        let fen_str = format!("{}", Fen::from_position(pos.clone(), EnPassantMode::Legal));
        let res = engine.analyze(&fen_str, depth)?;
        let best_uci = res.best_move.clone();
        let played = &uci_moves[k];

        let matches_best = best_uci
            .as_ref()
            .map(|b| b.eq_ignore_ascii_case(played))
            .unwrap_or(false);

        let mut kind = classify_swing(swing);
        let base = kind;
        if base == "good"
            && matches_best
            && eval_before < -80
            && swing >= 100
        {
            kind = "brilliant";
        }

        let display_move_number = (k as i32) / 2 + 1;

        out.push(ClassifiedMove {
            half_move_index: k,
            display_move_number,
            uci: played.clone(),
            kind,
            eval_before,
            eval_after,
            swing_cp: swing,
            best_move_uci: best_uci,
        });
    }

    Ok(out)
}

/// Same player-move swings as [`classify_player_moves`], but without per-move Stockfish lookups
/// (no best-move / brilliant detection). Intended for Versus-style batch scans.
pub fn classify_player_moves_from_eval(
    uci_moves: &[String],
    eval_history: &[i32],
    player_is_white: bool,
) -> Result<Vec<ClassifiedMove>, String> {
    if eval_history.len() != uci_moves.len() + 1 {
        return Err("eval_history length mismatch".into());
    }

    let mut out = Vec::new();

    for k in 0..uci_moves.len() {
        if !is_player_move(k, player_is_white) {
            continue;
        }

        let eval_before = eval_history[k];
        let eval_after = eval_history[k + 1];
        let swing = eval_after - eval_before;
        let kind = classify_swing(swing);
        let display_move_number = (k as i32) / 2 + 1;

        out.push(ClassifiedMove {
            half_move_index: k,
            display_move_number,
            uci: uci_moves[k].clone(),
            kind,
            eval_before,
            eval_after,
            swing_cp: swing,
            best_move_uci: None,
        });
    }

    Ok(out)
}

pub fn count_move_kinds(classified: &[ClassifiedMove]) -> (i32, i32, i32) {
    let mut b = 0;
    let mut m = 0;
    let mut i = 0;
    for c in classified {
        match c.kind {
            "blunder" => b += 1,
            "mistake" => m += 1,
            "inaccuracy" => i += 1,
            _ => {}
        }
    }
    (b, m, i)
}

pub fn avg_centipawn_loss(classified: &[ClassifiedMove]) -> f64 {
    if classified.is_empty() {
        return 0.0;
    }
    let sum: f64 = classified
        .iter()
        .map(|c| (-c.swing_cp.min(0)) as f64)
        .sum();
    sum / classified.len() as f64
}

/// Lichess-style accuracy from average centipawn loss (player moves only).
pub fn accuracy_from_acpl(acpl: f64) -> f64 {
    if acpl <= 0.0 {
        return 100.0;
    }
    let a = 103.1668_f64 * (-0.00368208_f64 * acpl).exp();
    a.clamp(0.0, 100.0)
}

pub fn max_min_eval(eval_history: &[i32]) -> (i32, i32) {
    if eval_history.is_empty() {
        return (0, 0);
    }
    let max = *eval_history.iter().max().unwrap_or(&0);
    let min = *eval_history.iter().min().unwrap_or(&0);
    (max, min)
}
