//! DirectInput: the system mouse and keyboard, read by polling.
//!
//! The state comes from the same input the window's messages are made from, so
//! the mouse, the keyboard, a controller and scripted steps all reach a game
//! that never looks at its messages. Only `GetDeviceState` is here: buffered
//! data and event notification stop with their names when a game asks.

use super::ddraw::{build_vtable, object, Method};
use crate::cpu::{Cpu, GuestFn};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const DI_OK: u32 = 0;
const DIERR_DEVICE_NOT_REGISTERED: u32 = 0x8004_0154;
const PRESSED: u8 = 0x80;
/// A press lasts at least this long, so a game that polls sees a click or a
/// scripted key whose release came in the same moment.
const SHORTEST_PRESS: Duration = Duration::from_millis(60);
const KEYS: usize = 256;
/// Ids for `set`: keys by their DirectInput code, then the mouse buttons.
const FIRST_BUTTON: usize = KEYS;
const BUTTONS: usize = 4;

const WM_KEYDOWN: u32 = 0x100;
const WM_KEYUP: u32 = 0x101;
const WM_LBUTTONDOWN: u32 = 0x201;
const WM_LBUTTONUP: u32 = 0x202;
const WM_RBUTTONDOWN: u32 = 0x204;
const WM_RBUTTONUP: u32 = 0x205;

#[derive(Clone, Copy, PartialEq)]
enum Device {
    Mouse,
    Keyboard,
}

pub struct Input {
    vtable: u32,
    device_vtable: u32,
    devices: HashMap<u32, Device>,
    /// Keys by DirectInput code, then mouse buttons: 0x80 while held.
    held: [u8; KEYS + BUTTONS],
    pressed_at: HashMap<usize, Instant>,
    /// Releases that wait for their press to have lasted long enough.
    releases: Vec<(usize, Instant)>,
    /// Where the pointer was when the mouse was last read.
    seen: Option<(u32, u32)>,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            vtable: 0,
            device_vtable: 0,
            devices: HashMap::new(),
            held: [0; KEYS + BUTTONS],
            pressed_at: HashMap::new(),
            releases: Vec::new(),
            seen: None,
        }
    }
}

/// The DirectInput code of a Windows virtual key, for the keys messages carry.
fn code_of_virtual_key(key: u32) -> Option<usize> {
    const LETTERS: [u8; 26] = [
        0x1e, 0x30, 0x2e, 0x20, 0x12, 0x21, 0x22, 0x23, 0x17, 0x24, 0x25, 0x26, 0x32, 0x31, 0x18, 0x19, 0x10, 0x13, 0x1f, 0x14, 0x16, 0x2f, 0x11, 0x2d, 0x15, 0x2c,
    ];
    Some(match key {
        0x41..=0x5a => LETTERS[(key - 0x41) as usize] as usize,
        0x31..=0x39 => (key - 0x31 + 2) as usize,
        0x30 => 0x0b,
        0x0d => 0x1c,
        0x1b => 0x01,
        0x08 => 0x0e,
        0x09 => 0x0f,
        0x20 => 0x39,
        0x25 => 0xcb,
        0x26 => 0xc8,
        0x27 => 0xcd,
        0x28 => 0xd0,
        0x24 => 0xc7,
        0x23 => 0xcf,
        0x21 => 0xc9,
        0x22 => 0xd1,
        0x2d => 0xd2,
        0x2e => 0xd3,
        0x11 => 0x1d,
        0x10 => 0x2a,
        0x12 => 0x38,
        0x70..=0x79 => (key - 0x70 + 0x3b) as usize,
        0x7a => 0x57,
        0x7b => 0x58,
        _ => return None,
    })
}

/// The DirectInput code of an SDL scancode, which is a USB keyboard usage.
fn code_of_scancode(scancode: u32) -> Option<usize> {
    const FROM_A: [u8; 96] = [
        // a to z
        0x1e, 0x30, 0x2e, 0x20, 0x12, 0x21, 0x22, 0x23, 0x17, 0x24, 0x25, 0x26, 0x32, 0x31, 0x18, 0x19, 0x10, 0x13, 0x1f, 0x14, 0x16, 0x2f, 0x11, 0x2d, 0x15, 0x2c,
        // 1 to 9 and 0
        0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
        // Enter, Escape, Backspace, Tab, Space, - = [ ] \ # ; ' ` , . /
        0x1c, 0x01, 0x0e, 0x0f, 0x39, 0x0c, 0x0d, 0x1a, 0x1b, 0x2b, 0x2b, 0x27, 0x28, 0x29, 0x33, 0x34, 0x35,
        // Caps Lock, F1 to F12
        0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x57, 0x58,
        // Print Screen, Scroll Lock, Pause, Insert, Home, Page Up, Delete, End, Page Down
        0xb7, 0x46, 0xc5, 0xd2, 0xc7, 0xc9, 0xd3, 0xcf, 0xd1,
        // Right, Left, Down, Up
        0xcd, 0xcb, 0xd0, 0xc8,
        // Num Lock, keypad / * - + Enter, 1 to 9, 0 and .
        0x45, 0xb5, 0x37, 0x4a, 0x4e, 0x9c, 0x4f, 0x50, 0x51, 0x4b, 0x4c, 0x4d, 0x47, 0x48, 0x49, 0x52, 0x53,
    ];
    const MODIFIERS: [u8; 8] = [0x1d, 0x2a, 0x38, 0xdb, 0x9d, 0x36, 0xb8, 0xdc];
    match scancode {
        4..=99 => Some(FROM_A[(scancode - 4) as usize] as usize),
        100 => Some(0x56),
        224..=231 => Some(MODIFIERS[(scancode - 224) as usize] as usize),
        _ => None,
    }
}

impl Input {
    fn set(&mut self, id: usize, down: bool) {
        let now = Instant::now();
        if down {
            self.releases.retain(|(waiting, _)| *waiting != id);
            self.held[id] = PRESSED;
            self.pressed_at.insert(id, now);
            return;
        }
        let earliest = self.pressed_at.get(&id).map_or(now, |pressed| *pressed + SHORTEST_PRESS);
        if earliest <= now {
            self.held[id] = 0;
        } else {
            self.releases.push((id, earliest));
        }
    }

    fn settle(&mut self) {
        let now = Instant::now();
        let held = &mut self.held;
        self.releases.retain(|(id, at)| {
            if *at <= now {
                held[*id] = 0;
            }
            *at > now
        });
    }

    /// Takes the keys and buttons from a message on its way to the window.
    pub fn note(&mut self, message: u32, wparam: u32) {
        match message {
            WM_KEYDOWN | WM_KEYUP => {
                if let Some(code) = code_of_virtual_key(wparam) {
                    self.set(code, message == WM_KEYDOWN);
                }
            }
            WM_LBUTTONDOWN | WM_LBUTTONUP => self.set(FIRST_BUTTON, message == WM_LBUTTONDOWN),
            WM_RBUTTONDOWN | WM_RBUTTONUP => self.set(FIRST_BUTTON + 1, message == WM_RBUTTONDOWN),
            _ => {}
        }
    }

    /// A key of the real keyboard, which has more keys than messages carry.
    pub fn key(&mut self, scancode: u32, down: bool) {
        if let Some(code) = code_of_scancode(scancode) {
            self.set(code, down);
        }
    }
}

macro_rules! method {
    ($offset:expr, $arguments:expr, |$cpu:ident| $body:block) => {
        ($offset, $arguments, (|$cpu: &mut Cpu| { let result: u32 = $body; $cpu.finish(result, $arguments); }) as GuestFn)
    };
}

fn common() -> Vec<Method> {
    vec![
        method!(0x00, 3, |cpu| { let (this, out) = (cpu.arg(0), cpu.arg(2)); cpu.w32(out, this); DI_OK }),
        method!(0x04, 1, |_cpu| { 1 }),
        method!(0x08, 1, |_cpu| { 0 }),
    ]
}

fn create_device(cpu: &mut Cpu) -> u32 {
    const SYSTEM_MOUSE: u32 = 0x6f1d_2b60;
    const SYSTEM_KEYBOARD: u32 = 0x6f1d_2b61;
    let (id, out) = (cpu.arg(1), cpu.arg(2));
    let kind = match cpu.r32(id) {
        SYSTEM_MOUSE => Device::Mouse,
        SYSTEM_KEYBOARD => Device::Keyboard,
        _ => return DIERR_DEVICE_NOT_REGISTERED,
    };
    let vtable = cpu.host.input.device_vtable;
    let device = object(cpu, vtable);
    cpu.host.input.devices.insert(device, kind);
    cpu.w32(out, device);
    DI_OK
}

fn capabilities(cpu: &mut Cpu) -> u32 {
    const ATTACHED: u32 = 1;
    const TYPE_MOUSE: u32 = 2;
    const TYPE_KEYBOARD: u32 = 3;
    let (this, at) = (cpu.arg(0), cpu.arg(1));
    let mouse = cpu.host.input.devices.get(&this) == Some(&Device::Mouse);
    let size = cpu.r32(at);
    cpu.fill(at + 4, size.saturating_sub(4), 0);
    cpu.w32(at + 4, ATTACHED);
    cpu.w32(at + 8, if mouse { TYPE_MOUSE } else { TYPE_KEYBOARD });
    cpu.w32(at + 12, if mouse { 2 } else { 0 });
    cpu.w32(at + 16, if mouse { 2 } else { KEYS as u32 });
    DI_OK
}

/// The mouse gives how far it moved since it was last read, and the keyboard
/// a byte for each key.
fn device_state(cpu: &mut Cpu) -> u32 {
    const MOUSE_STATE: u32 = 16;
    let (this, size, at) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
    cpu.host.input.settle();
    if cpu.host.input.devices.get(&this) == Some(&Device::Keyboard) {
        let keys = cpu.host.input.held;
        let count = (size as usize).min(KEYS);
        cpu.bytes_mut(at, count as u32).copy_from_slice(&keys[..count]);
        return DI_OK;
    }
    let pointer = cpu.host.pointer;
    let seen = cpu.host.input.seen.replace(pointer).unwrap_or(pointer);
    let buttons = cpu.host.input.held;
    cpu.fill(at, size, 0);
    cpu.w32(at, pointer.0.wrapping_sub(seen.0));
    cpu.w32(at + 4, pointer.1.wrapping_sub(seen.1));
    if size >= MOUSE_STATE {
        cpu.bytes_mut(at + 12, BUTTONS as u32).copy_from_slice(&buttons[FIRST_BUTTON..]);
    }
    DI_OK
}

fn build_tables(cpu: &mut Cpu) {
    if cpu.host.input.vtable != 0 {
        return;
    }
    let mut input = common();
    input.push(method!(0x0c, 4, |cpu| { create_device(cpu) }));
    let mut device = common();
    device.extend([
        method!(0x0c, 2, |cpu| { capabilities(cpu) }),
        method!(0x18, 3, |_cpu| { DI_OK }),
        method!(0x1c, 1, |_cpu| { DI_OK }),
        method!(0x20, 1, |_cpu| { DI_OK }),
        method!(0x24, 3, |cpu| { device_state(cpu) }),
        method!(0x2c, 2, |_cpu| { DI_OK }),
        method!(0x34, 3, |_cpu| { DI_OK }),
    ]);
    cpu.host.input.vtable = build_vtable(cpu, "IDirectInput", 8, &input);
    cpu.host.input.device_vtable = build_vtable(cpu, "IDirectInputDevice", 18, &device);
}

pub fn find(name: &str) -> Option<GuestFn> {
    super::table! { name;
        "DirectInputCreateA" 4 |cpu| {
            build_tables(cpu);
            let vtable = cpu.host.input.vtable;
            let at = object(cpu, vtable);
            let out = cpu.arg(2);
            cpu.w32(out, at);
            DI_OK
        }
    }
}
