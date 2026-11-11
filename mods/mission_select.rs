//! Starts any mission from the map, for testing the later levels.
//!
//! `Incubation.exe` 0x45c200 is `mission_name(node, out)`, cdecl: it copies
//! the file name of the map node's mission, such as `_C_101`, to `out`, and
//! returns 1. With `--mission=NAME` on the command line, or `-mission=NAME` in
//! `RECOMP_COMMAND` (the fourth argument of `./port recomp`), every node names
//! that mission instead, so clicking the first one on the map starts it.

use crate::code::*;
use recomp_runtime::Cpu;
use std::sync::OnceLock;

fn chosen() -> Option<&'static str> {
    static CHOSEN: OnceLock<Option<String>> = OnceLock::new();
    CHOSEN
        .get_or_init(|| {
            let command = std::env::var("RECOMP_COMMAND").unwrap_or_default();
            let words = std::env::args().skip(1).chain(command.split_whitespace().map(str::to_string).collect::<Vec<_>>());
            let found = words.filter_map(|word| word.trim_start_matches('-').strip_prefix("mission=").map(str::to_string)).last()?;
            let valid = !found.is_empty() && found.len() <= 12 && found.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
            if !valid {
                eprintln!("ignoring --mission={found}: a mission is a name such as _C_105");
            }
            valid.then_some(found)
        })
        .as_deref()
}

pub fn f_0045c200(cpu: &mut Cpu) {
    let out = cpu.r32(cpu.esp + 8);
    o_0045c200(cpu);
    if let Some(name) = chosen() {
        cpu.write_string(out, name);
    }
}
