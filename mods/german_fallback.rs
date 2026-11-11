//! Fills gaps in a German translation with the English texts.
//!
//! The GOG version ships German texts in `DATA\TXT\German`, and the fan
//! patch for German speech leaves them as they are. They are older than the
//! add-on: they have no briefings for the Wilderness Missions, multiplayer
//! and skirmish maps, and no names for the add-on's missions, so the map
//! showed "(null)" there.
//!
//! `Incubation.exe` 0x4389e0 opens a file for reading, `open(&handle, path)`,
//! cdecl, and always returns 1, leaving an invalid handle when the file is
//! missing. Every text file goes through it. For a path in a `German` folder:
//!
//! - A missing file is opened from the `English` folder beside it instead.
//!   Briefings fell back to `default.txt` before, which says nothing.
//! - A `.lcl` text list gets the English lines for every `%:NNN` key it lacks,
//!   put before its `[END]`. Lists are stored with every byte inverted.
//!
//! Anything else, and every path outside a `German` folder, goes to the original.

use crate::code::*;
use recomp_runtime::{host, Cpu};

pub fn f_004389e0(cpu: &mut Cpu) {
    let (out, at) = (cpu.r32(cpu.esp + 4), cpu.r32(cpu.esp + 8));
    let guest = cpu.string(at);
    let Some(start) = guest.to_ascii_lowercase().find("\\german\\") else {
        return o_004389e0(cpu);
    };
    let english = format!("{}\\English\\{}", &guest[..start], &guest[start + 8..]);
    let (german_path, english_path) = (host::game_path(cpu, &guest), host::game_path(cpu, &english));
    let Ok(english_data) = std::fs::read(&english_path) else {
        return o_004389e0(cpu);
    };
    let data = match std::fs::read(&german_path) {
        Err(_) => english_data,
        Ok(german_data) if guest.to_ascii_lowercase().ends_with(".lcl") => match merge(&german_data, &english_data) {
            Some(merged) => merged,
            None => return o_004389e0(cpu),
        },
        Ok(_) => return o_004389e0(cpu),
    };
    let handle = host::open_memory(cpu, german_path, data);
    cpu.w32(out, handle);
    cpu.eax = 1;
    cpu.pop();
}

/// The German list with the English lines for its missing keys, or None when
/// nothing is missing.
fn merge(german: &[u8], english: &[u8]) -> Option<Vec<u8>> {
    let (german, inverted) = decode(german);
    let (english, _) = decode(english);
    let have: std::collections::HashSet<u32> = german.split(|&b| b == b'\n').filter_map(key).collect();
    let mut missing = Vec::new();
    for line in english.split_inclusive(|&b| b == b'\n') {
        if key(line).is_some_and(|k| !have.contains(&k)) {
            missing.extend_from_slice(line);
            if !line.ends_with(b"\n") {
                missing.extend_from_slice(b"\r\n");
            }
        }
    }
    if missing.is_empty() {
        return None;
    }
    // Before the last line that starts with [END], or at the end.
    let end = german
        .split_inclusive(|&b| b == b'\n')
        .scan(0, |at, line| { let start = *at; *at += line.len(); Some((start, line)) })
        .filter(|(_, line)| line.trim_ascii_start().starts_with(b"[END]"))
        .last()
        .map_or(german.len(), |(start, _)| start);
    let mut merged = german[..end].to_vec();
    if !merged.ends_with(b"\n") {
        merged.extend_from_slice(b"\r\n");
    }
    merged.extend_from_slice(&missing);
    merged.extend_from_slice(&german[end..]);
    if inverted {
        merged.iter_mut().for_each(|b| *b = !*b);
    }
    Some(merged)
}

/// A list's text, and whether it was stored inverted. Inverted lists start
/// with 0xfe, a 0x01 marker byte before the text.
fn decode(data: &[u8]) -> (Vec<u8>, bool) {
    if data.first() == Some(&0xfe) {
        (data.iter().map(|b| !b).collect(), true)
    } else {
        (data.to_vec(), false)
    }
}

/// The number of a `%:NNN,` or `%:NNN:` line. A line can start with the marker byte.
fn key(line: &[u8]) -> Option<u32> {
    let line = line.strip_prefix(b"\x01").unwrap_or(line).trim_ascii_start();
    let digits = line.strip_prefix(b"%:")?;
    let count = digits.iter().take_while(|b| b.is_ascii_digit()).count();
    std::str::from_utf8(&digits[..count]).ok()?.parse().ok()
}
