//! A controller, working as a mouse. Strategy games like these have too many
//! buttons on screen to map to a controller's, so the left stick moves the
//! pointer, the right stick moves it slowly for small targets, and buttons or
//! triggers click. The game only ever sees mouse messages and keys.

use crate::cpu::Cpu;
use crate::host::video;
use std::time::Instant;

const WM_KEYDOWN: u32 = 0x100;
const WM_KEYUP: u32 = 0x101;
const WM_MOUSEMOVE: u32 = 0x200;
const WM_LBUTTONDOWN: u32 = 0x201;
const WM_LBUTTONUP: u32 = 0x202;
const WM_RBUTTONDOWN: u32 = 0x204;
const WM_RBUTTONUP: u32 = 0x205;
const MK_LBUTTON: u32 = 1;
const MK_RBUTTON: u32 = 2;

pub const AXIS_LEFT_TRIGGER: u8 = 4;
pub const AXIS_RIGHT_TRIGGER: u8 = 5;
const TRIGGER_PRESS: f32 = 0.5;
const TRIGGER_RELEASE: f32 = 0.3;
pub const BUTTON_SOUTH: u8 = 0;
pub const BUTTON_EAST: u8 = 1;
pub const BUTTON_START: u8 = 6;
const BUTTON_RIGHT_SHOULDER: u8 = 10;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;
const BUTTON_DPAD_LEFT: u8 = 13;
const BUTTON_DPAD_RIGHT: u8 = 14;
const KEY_ESCAPE: u32 = 0x1b;
const KEY_LEFT: u32 = 0x25;
const KEY_UP: u32 = 0x26;
const KEY_RIGHT: u32 = 0x27;
const KEY_DOWN: u32 = 0x28;

const SCREEN_RIGHT: i32 = 639;
const SCREEN_BOTTOM: i32 = 479;
/// Worn sticks don't quite centre, so small readings count as none.
const DEAD_ZONE: f32 = 0.18;
/// Pixels a second at full tilt, for the left stick and the right stick.
const SPEEDS: [f32; 2] = [420.0, 90.0];
/// Longer gaps are a stall, which shouldn't throw the pointer across the screen.
const LONGEST_STEP: f32 = 0.1;

pub struct Pad {
    /// Each stick's x and y, from -1 to 1.
    pub sticks: [[f32; 2]; 2],
    /// Whether the left and right triggers are pressed.
    triggers: [bool; 2],
    last: Instant,
    /// Fractions of a pixel carried over, so slow movement still moves.
    leftover: [f32; 2],
    held: u32,
}

impl Default for Pad {
    fn default() -> Pad {
        Pad { sticks: [[0.0; 2]; 2], triggers: [false; 2], last: Instant::now(), leftover: [0.0; 2], held: 0 }
    }
}

/// Past the dead zone, a squared curve gives fine control near the centre.
fn curve(value: f32) -> f32 {
    let beyond = ((value.abs() - DEAD_ZONE) / (1.0 - DEAD_ZONE)).clamp(0.0, 1.0);
    value.signum() * beyond * beyond
}

/// Records an axis, numbered as SDL does: left x and y, right x and y, then the
/// left and right triggers.
pub fn set_axis(cpu: &mut Cpu, axis: u8, value: f32) {
    match axis {
        0..=3 => cpu.host.pad.sticks[(axis / 2) as usize][(axis % 2) as usize] = value,
        AXIS_LEFT_TRIGGER => set_trigger(cpu, 0, BUTTON_EAST, value),
        AXIS_RIGHT_TRIGGER => set_trigger(cpu, 1, BUTTON_SOUTH, value),
        _ => {}
    }
}

/// A trigger is a button with two thresholds, so a finger resting near one
/// doesn't click over and over.
fn set_trigger(cpu: &mut Cpu, index: usize, button: u8, value: f32) {
    let held = cpu.host.pad.triggers[index];
    let down = value >= if held { TRIGGER_RELEASE } else { TRIGGER_PRESS };
    if down != held {
        cpu.host.pad.triggers[index] = down;
        set_button(cpu, button, down);
    }
}

/// Moves the pointer for the time since the last call.
pub fn advance(cpu: &mut Cpu) {
    let pad = &mut cpu.host.pad;
    let now = Instant::now();
    let elapsed = (now - pad.last).as_secs_f32().min(LONGEST_STEP);
    pad.last = now;
    let mut whole = [0i32; 2];
    for axis in 0..2 {
        let wanted: f32 = (0..2).map(|stick| curve(pad.sticks[stick][axis]) * SPEEDS[stick] * elapsed).sum();
        let total = pad.leftover[axis] + wanted;
        whole[axis] = total.trunc() as i32;
        pad.leftover[axis] = total.fract();
    }
    if whole == [0, 0] {
        return;
    }
    let held = pad.held;
    let (x, y) = cpu.host.pointer;
    let x = (x as i32 + whole[0]).clamp(0, SCREEN_RIGHT) as u32;
    let y = (y as i32 + whole[1]).clamp(0, SCREEN_BOTTOM) as u32;
    cpu.host.pointer = (x, y);
    cpu.host.messages.push_back((WM_MOUSEMOVE, held, x | (y << 16)));
}

/// Handles a controller button going down or up.
pub fn set_button(cpu: &mut Cpu, button: u8, down: bool) {
    if button == BUTTON_RIGHT_SHOULDER {
        if down {
            video::toggle_stats(cpu);
        }
        return;
    }
    let (x, y) = cpu.host.pointer;
    let position = x | (y << 16);
    let mouse = |pressed, released, flag| (if down { pressed } else { released }, flag);
    let click = match button {
        BUTTON_SOUTH => Some(mouse(WM_LBUTTONDOWN, WM_LBUTTONUP, MK_LBUTTON)),
        BUTTON_EAST => Some(mouse(WM_RBUTTONDOWN, WM_RBUTTONUP, MK_RBUTTON)),
        _ => None,
    };
    if let Some((message, flag)) = click {
        let pad = &mut cpu.host.pad;
        pad.held = if down { pad.held | flag } else { pad.held & !flag };
        let held = pad.held;
        cpu.host.messages.push_back((message, held, position));
        return;
    }
    let key = match button {
        BUTTON_START => KEY_ESCAPE,
        BUTTON_DPAD_UP => KEY_UP,
        BUTTON_DPAD_DOWN => KEY_DOWN,
        BUTTON_DPAD_LEFT => KEY_LEFT,
        BUTTON_DPAD_RIGHT => KEY_RIGHT,
        _ => return,
    };
    cpu.host.messages.push_back((if down { WM_KEYDOWN } else { WM_KEYUP }, key, 0));
}
