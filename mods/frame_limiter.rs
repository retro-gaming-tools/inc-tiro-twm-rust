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
//! Turned off in the launcher, the original limiter runs, bug and all.
//!
//! launcher-advanced: Smooth frame pacing

use crate::code::*;
use recomp_runtime::{host, Cpu};

const MIN_FPS: u32 = 0x522edc;
const MAX_FPS: u32 = 0x522ee0;
const NOW: u32 = 0x522ee4;
const FRAME_START: u32 = 0x522ee8;
const LATE: u32 = 0x522eec;
const DROPPED: u32 = 0x522ef0;

pub fn f_0047f2e0(cpu: &mut Cpu) {
    let min_fps = cpu.r32(cpu.esp + 4) as i32;
    let max_fps = cpu.r32(cpu.esp + 8) as i32;
    if min_fps <= 0 || max_fps <= 0 || !host::mod_enabled(cpu, "frame_limiter") {
        // Turned off, or limits the original asserts on or divides by zero with.
        return o_0047f2e0(cpu);
    }
    if cpu.trace_filter.as_deref() == Some("frames") && (cpu.r32(MIN_FPS) != min_fps as u32 || cpu.r32(MAX_FPS) != max_fps as u32) {
        println!("frame limits {min_fps} to {max_fps} a second");
    }
    cpu.w32(MIN_FPS, min_fps as u32);
    cpu.w32(MAX_FPS, max_fps as u32);
    host::pace_frames(cpu, f64::from(max_fps));
    let now = host::tick_count(cpu);
    cpu.w32(NOW, now);
    cpu.w32(FRAME_START, now);
    cpu.w32(LATE, 0);
    cpu.w32(DROPPED, 0);
    cpu.eax = now;
    cpu.pop();
}
