//! `recomp.log`: the launcher's steps, what was started and any crash, for a
//! player to send. Each line is written at once, so the log still says how far
//! the launcher got when SDL or a driver kills the program without a panic.

use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub const FILE_NAME: &str = "recomp.log";
/// The run before's log, so starting again doesn't lose the one to send.
pub const PREVIOUS_FILE_NAME: &str = "recomp-previous.log";

static LOG: Mutex<Option<std::fs::File>> = Mutex::new(None);
static PATH: OnceLock<PathBuf> = OnceLock::new();

fn open_in(folder: PathBuf) -> Option<(PathBuf, std::fs::File)> {
    let path = folder.join(FILE_NAME);
    if path.is_file() {
        let _ = std::fs::rename(&path, folder.join(PREVIOUS_FILE_NAME));
    }
    std::fs::File::create(&path).ok().map(|file| (path, file))
}

/// Starts the log beside the program, as `settings.ini` is, else in `fallback`.
pub fn start(title: &str, version: &str, fallback: impl FnOnce() -> Option<PathBuf>) {
    let opened = crate::host::launcher::program_folder().and_then(open_in).or_else(|| fallback().and_then(open_in));
    if let Some((path, file)) = opened {
        let _ = PATH.set(path);
        *LOG.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(file);
    }
    line(&format!("{title} {version}, on {} {}", std::env::consts::OS, std::env::consts::ARCH));
}

/// Adds a line. Never waits, so the panic hook can use it.
pub fn line(text: &str) {
    if let Ok(mut log) = LOG.try_lock() {
        if let Some(file) = log.as_mut() {
            let _ = file.write_all(format!("{text}\n").as_bytes());
        }
    }
}

pub fn path() -> Option<PathBuf> {
    PATH.get().cloned()
}

/// A scripted or headless run must never wait on a message box.
pub fn automated() -> bool {
    std::env::var("RECOMP_SCRIPT").is_ok_and(|script| !script.trim().is_empty()) || std::env::var_os("RECOMP_HEADLESS").is_some()
}
