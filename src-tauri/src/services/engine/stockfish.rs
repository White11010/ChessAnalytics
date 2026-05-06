use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager};

static ENGINE: Lazy<Arc<Mutex<Option<StockfishEngine>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub struct StockfishEngine {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl StockfishEngine {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let engine_path = get_engine_path(app)?;
        println!("Engine path: {:?}", engine_path);

        let mut child = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start Stockfish: {}", e))?;

        let stdin = child.stdin.take().ok_or("No stdin")?;
        let stdout = child.stdout.take().ok_or("No stdout")?;

        let mut engine = StockfishEngine {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        engine.init()?;

        Ok(engine)
    }

    fn init(&mut self) -> Result<(), String> {
        self.send("uci")?;
        self.wait_for("uciok")?;

        self.send("isready")?;
        self.wait_for("readyok")?;

        Ok(())
    }

    fn send(&mut self, cmd: &str) -> Result<(), String> {
        writeln!(self.stdin, "{}", cmd).map_err(|e| e.to_string())
    }

    fn wait_for(&mut self, keyword: &str) -> Result<(), String> {
        let mut line = String::new();

        loop {
            line.clear();

            let n = self
                .stdout
                .read_line(&mut line)
                .map_err(|e| e.to_string())?;

            if n == 0 {
                return Err("Engine closed".into());
            }

            if line.contains(keyword) {
                return Ok(());
            }
        }
    }

    pub fn analyze(&mut self, fen: &str, depth: u8) -> Result<EngineResult, String> {
        self.send(&format!("position fen {}", fen))?;
        self.send(&format!("go depth {}", depth))?;

        let mut best_move = None;
        let mut eval = None;

        let mut line = String::new();

        loop {
            line.clear();

            let n = self
                .stdout
                .read_line(&mut line)
                .map_err(|e| e.to_string())?;

            if n == 0 {
                break;
            }

            if line.starts_with("info") {
                if let Some(score) = parse_score(&line) {
                    eval = Some(score);
                }
            }

            if line.starts_with("bestmove") {
                best_move = parse_bestmove(&line);
                break;
            }
        }

        Ok(EngineResult { best_move, eval })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct EngineResult {
    pub best_move: Option<String>,
    pub eval: Option<i32>,
}

fn parse_bestmove(line: &str) -> Option<String> {
    line.split_whitespace().nth(1).map(|s| s.to_string())
}

fn parse_score(line: &str) -> Option<i32> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    for i in 0..parts.len() {
        if parts[i] == "score" && i + 2 < parts.len() {
            match parts[i + 1] {
                "cp" => return parts[i + 2].parse::<i32>().ok(),
                "mate" => {
                    let m = parts[i + 2].parse::<i32>().ok()?;
                    return Some(if m > 0 { 10_000 } else { -10_000 });
                }
                _ => {}
            }
        }
    }

    None
}

fn get_engine_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    #[cfg(debug_assertions)]
    {
        let path = std::path::PathBuf::from("resources/stockfish.exe");

        if path.exists() {
            return Ok(path);
        }
    }

    let path = app
        .path()
        .resolve("stockfish.exe", tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    if !path.exists() {
        return Err(format!("Stockfish not found: {:?}", path));
    }

    Ok(path)
}

pub fn get_engine() -> Arc<Mutex<Option<StockfishEngine>>> {
    ENGINE.clone()
}
