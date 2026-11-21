//! Warns in the launcher when English texts are chosen but the game's
//! English text folder holds German ones. The fan-made German patch (German
//! speech) writes German texts into `DATA/TXT/English`: the menus, the main
//! campaign's briefings and the unit data. A patched copy then shows German
//! with either setting, which looks like a broken language switch. The menu
//! file `GUI/Txtmod_a.lcl` says which language it is in its header comment:
//! "English" in GOG's, "Deutsch" in the patch's. See `docs/original-engine.md`
//! in the port repo. Nothing here replaces a game function.
//!
//! launcher-check: english_texts

use recomp_runtime::loader::Choices;
use std::path::{Path, PathBuf};

const MENU_TEXTS: [&str; 5] = ["DATA", "TXT", "English", "GUI", "Txtmod_a.lcl"];
/// The header comment is in the first few hundred bytes.
const HEADER: usize = 400;

pub fn english_texts(choices: &Choices) -> Option<[String; 2]> {
    if !choices.language.eq_ignore_ascii_case("English") {
        return None;
    }
    let path = find(choices.folder, &MENU_TEXTS)?;
    let bytes = std::fs::read(path).ok()?;
    is_german(&bytes).then(|| ["English texts are German here".to_string(), "Copy GOG's DATA/TXT/English".to_string()])
}

/// Whether an `.lcl` file's header names German, unscrambling it first if it
/// starts with 0xFE (the rest XOR 0xFF).
fn is_german(bytes: &[u8]) -> bool {
    let text: Vec<u8> = match bytes.split_first() {
        Some((0xfe, rest)) => rest.iter().take(HEADER).map(|byte| byte ^ 0xff).collect(),
        _ => bytes.iter().take(HEADER).copied().collect(),
    };
    String::from_utf8_lossy(&text).contains("Deutsch")
}

/// A path under the folder, ignoring the case of each part, as Windows does.
fn find(folder: &Path, parts: &[&str]) -> Option<PathBuf> {
    let mut path = folder.to_path_buf();
    for part in parts {
        let entry = std::fs::read_dir(&path).ok()?.flatten().find(|entry| entry.file_name().to_string_lossy().eq_ignore_ascii_case(part))?;
        path = entry.path();
    }
    Some(path)
}
