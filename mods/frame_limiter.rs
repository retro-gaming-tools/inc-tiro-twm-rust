//! The frame limiter, `Incubation.exe` 0x47f2e0, called once a frame as
//! `limit_frames(min_fps, max_fps)`, cdecl.
//!
//! The original forgets to restart its timer after waiting, so frames
//! alternated between 33 and 16 ms. That was fixed by a `recompPatches` entry
//! at 0x47f381, which the first version of this mod replaced, sleeping instead
//! of reading the clock in a loop.
//!
//! Waiting here, at the start of a frame, still let frames reach the screen
//! unevenly, 27 to 41 ms apart in a mission, since the drawing after it takes
//! longer on some frames than others. Now this only tells the runtime the
//! game's frame rate and returns, and the runtime waits at the swap, just
//! before the frame is shown, in step with the display. The game loop keeps
//! the same period, because the wait moved within it rather than away.
//! Nothing but this function reads the globals it keeps.
//!
//! The launcher's Frame rate row: *Original* runs the original limiter, bug
//! and all; *30* (the default) paces at the rate the game asks for; *60*
//! doubles the missions' 30, leaving the map's 25 and briefings' 20. F9 in
//! game steps on from the row's choice: 30, 60, Original, 30 again, and says
//! which for three seconds (`host::show_note`). Animations step per frame, so at 60
//! they run twice as fast (see `docs/original-engine.md` in the port repo).
//! The values are in this order so that a `settings.ini` from when this was
//! an On/Off row ("Smooth frame pacing", `mod.frame_limiter=1`) reads as 30.
//!
//! launcher-choice: Frame rate = Original | *30 | 60

use crate::code::*;
use recomp_runtime::{host, Cpu};

const MIN_FPS: u32 = 0x522edc;
const MAX_FPS: u32 = 0x522ee0;
const NOW: u32 = 0x522ee4;
const FRAME_START: u32 = 0x522ee8;
const LATE: u32 = 0x522eec;
const DROPPED: u32 = 0x522ef0;

/// The row's values, in its order.
const ORIGINAL: usize = 0;
const THIRTY: usize = 1;
const SIXTY: usize = 2;
const MODES: usize = 3;
const NAMES: [&str; MODES] = ["Original", "30 FPS", "60 FPS"];

/// F9 presses already shown as a note.
static NOTED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The mode now: the row's choice, stepped on once for each F9 press. From
/// 30 that goes 60, Original, 30, as the values' order is.
fn mode(cpu: &mut Cpu) -> usize {
    let presses = host::f9_presses(cpu);
    let chosen = host::mod_choice(cpu, "frame_limiter").unwrap_or(THIRTY);
    let mode = (chosen + presses as usize) % MODES;
    if NOTED.swap(presses, std::sync::atomic::Ordering::Relaxed) != presses {
        host::show_note(cpu, &format!("Frame rate: {}", NAMES[mode]));
    }
    mode
}

pub fn f_0047f2e0(cpu: &mut Cpu) {
    let min_fps = cpu.r32(cpu.esp + 4) as i32;
    let max_fps = cpu.r32(cpu.esp + 8) as i32;
    let mode = mode(cpu);
    if min_fps <= 0 || max_fps <= 0 || mode == ORIGINAL {
        // Turned off, or limits the original asserts on or divides by zero with.
        return o_0047f2e0(cpu);
    }
    if cpu.trace_filter.as_deref() == Some("frames") && (cpu.r32(MIN_FPS) != min_fps as u32 || cpu.r32(MAX_FPS) != max_fps as u32) {
        println!("frame limits {min_fps} to {max_fps} a second");
    }
    cpu.w32(MIN_FPS, min_fps as u32);
    cpu.w32(MAX_FPS, max_fps as u32);
    // Missions ask for 30, which 60 doubles.
    const MISSION_FPS: i32 = 30;
    let paced = if max_fps == MISSION_FPS && mode == SIXTY { max_fps * 2 } else { max_fps };
    host::pace_frames(cpu, f64::from(paced));
    let now = host::tick_count(cpu);
    cpu.w32(NOW, now);
    cpu.w32(FRAME_START, now);
    cpu.w32(LATE, 0);
    cpu.w32(DROPPED, 0);
    cpu.eax = now;
    cpu.pop();
}
