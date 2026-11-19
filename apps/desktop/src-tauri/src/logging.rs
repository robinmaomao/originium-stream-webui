use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use chrono::Local;

use crate::models::LogEntry;

const MAX_BUFFER: usize = 2000;
const MAX_FILE_BYTES: u64 = 10 * 1024 * 1024;

struct LogState {
    buffer: Mutex<VecDeque<LogEntry>>,
    file: Mutex<Option<(PathBuf, fs::File)>>,
}

fn state() -> &'static LogState {
    static STATE: OnceLock<LogState> = OnceLock::new();
    STATE.get_or_init(|| LogState {
        buffer: Mutex::new(VecDeque::new()),
        file: Mutex::new(None),
    })
}

pub fn init(log_dir: &Path) {
    let _ = fs::create_dir_all(log_dir);
    let path = log_dir.join("originium.log");
    if let Ok(file) = OpenOptions::new().create(true).append(true).open(&path) {
        *state().file.lock().unwrap() = Some((path, file));
    }
}

pub fn push(level: &str, target: &str, message: &str) {
    let entry = LogEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
        level: level.to_string(),
        target: target.to_string(),
        message: message.to_string(),
    };

    {
        let mut buffer = state().buffer.lock().unwrap();
        if buffer.len() >= MAX_BUFFER {
            buffer.pop_front();
        }
        buffer.push_back(entry.clone());
    }

    if let Some((_, file)) = state().file.lock().unwrap().as_mut() {
        let _ = writeln!(
            file,
            "{} [{}] {}: {}",
            entry.timestamp, entry.level, entry.target, entry.message
        );
        let _ = file.flush();
        if let Ok(metadata) = file.metadata() {
            if metadata.len() > MAX_FILE_BYTES {
                rotate();
            }
        }
    }
}

fn rotate() {
    let mut guard = state().file.lock().unwrap();
    if let Some((path, _)) = guard.as_ref() {
        let rotated = path.with_extension("log.1");
        let _ = fs::rename(path, rotated);
        if let Ok(file) = OpenOptions::new().create(true).append(true).open(path) {
            *guard = Some((path.clone(), file));
        }
    }
}

pub fn snapshot(limit: usize) -> Vec<LogEntry> {
    let buffer = state().buffer.lock().unwrap();
    let skip = buffer.len().saturating_sub(limit);
    buffer.iter().skip(skip).cloned().collect()
}

pub fn clear() {
    state().buffer.lock().unwrap().clear();
}

pub fn log_file_path() -> Option<PathBuf> {
    state().file.lock().unwrap().as_ref().map(|(path, _)| path.clone())
}
