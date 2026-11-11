//! Asks before giving up a mission. `Incubation.exe` 0x453900 runs an
//! interface event, `run_event(event)`, cdecl, for a key from 0x450d50 or a
//! click on the button columns from 0x450d70.
//!
//! Event 9 is *Give up!*: the surrender message, then every unit of the player
//! is removed (0x47bc60) and the mission is lost. The original maps Escape to
//! it with no question, and players press Escape expecting a menu, so this asks
//! first, for the key and the button alike. Any other event, or a yes, goes to
//! the original. A no returns 0, as events that change nothing do.
//!
//! launcher: Ask before giving up

use crate::code::*;
use recomp_runtime::{host, Cpu};

const GIVE_UP: u32 = 9;

pub fn f_00453900(cpu: &mut Cpu) {
    let event = cpu.r32(cpu.esp + 4);
    if event != GIVE_UP || !host::mod_enabled(cpu, "give_up") {
        return o_00453900(cpu);
    }
    let german = host::command(cpu).to_ascii_lowercase().contains("-german");
    let (title, text, yes, no) = if german {
        ("Mission aufgeben?", "Willst du die Mission wirklich aufgeben? Alle deine Einheiten gehen verloren, und die Mission ist verloren.", "Aufgeben", "Weiterspielen")
    } else {
        ("Give up the mission?", "Do you really want to give up? All your units will be lost, and so will the mission.", "Give up", "Keep playing")
    };
    if host::ask(cpu, title, text, yes, no) {
        return o_00453900(cpu);
    }
    cpu.eax = 0;
    cpu.pop();
}
