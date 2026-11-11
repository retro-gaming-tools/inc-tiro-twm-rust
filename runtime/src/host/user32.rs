//! user32 and gdi32: one window, its messages, and what the game asks of GDI.

use super::table;
use crate::cpu::{Cpu, GuestFn};
use crate::host::{audio, gamepad, script, video};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const MSG_SIZE: u32 = 28;
const WM_CREATE: u32 = 1;
const WM_DESTROY: u32 = 2;
const WM_NCDESTROY: u32 = 0x82;
const WM_ACTIVATEAPP: u32 = 0x1c;
const WM_ACTIVATE: u32 = 6;
const WM_SETFOCUS: u32 = 7;
const WM_SHOWWINDOW: u32 = 0x18;
const WM_PAINT: u32 = 0xf;

/// Ends a function in a `table!` entry early, with a result of FALSE.
macro_rules! return_false {
    () => {
        return 0
    };
}

/// Ends a function in a `table!` entry early, with this result.
macro_rules! return_value {
    ($value:expr) => {
        return $value
    };
}

/// Sends a message to the game's window procedure and returns its result.
pub fn send(cpu: &mut Cpu, message: u32, wparam: u32, lparam: u32) -> u32 {
    let (procedure, window) = (cpu.host.window_procedure, cpu.host.window);
    if procedure == 0 {
        return 0;
    }
    let saved = (cpu.ebx, cpu.esi, cpu.edi, cpu.ebp);
    cpu.push(lparam);
    cpu.push(wparam);
    cpu.push(message);
    cpu.push(window);
    cpu.push(0); // Its `ret 16` pops this and the four arguments.
    cpu.call(procedure);
    (cpu.ebx, cpu.esi, cpu.edi, cpu.ebp) = saved;
    cpu.eax
}

/// The printf that Windows has: no floats, and %ld and %lu mean the same as %d and %u.
fn format(cpu: &Cpu, pattern: &str, mut next: impl FnMut(&Cpu) -> u32) -> String {
    let mut output = String::new();
    let mut characters = pattern.chars().peekable();
    while let Some(character) = characters.next() {
        if character != '%' {
            output.push(character);
            continue;
        }
        let mut left = false;
        let mut zero = false;
        let mut width = 0usize;
        while let Some(&flag) = characters.peek() {
            match flag {
                '-' => left = true,
                '0' => zero = true,
                '+' | ' ' | '#' => {}
                _ => break,
            }
            characters.next();
        }
        while let Some(digit) = characters.peek().and_then(|c| c.to_digit(10)) {
            width = width * 10 + digit as usize;
            characters.next();
        }
        if characters.peek() == Some(&'.') {
            characters.next();
            while characters.peek().is_some_and(|c| c.is_ascii_digit()) {
                characters.next();
            }
        }
        while matches!(characters.peek(), Some('l' | 'h')) {
            characters.next();
        }
        let text = match characters.next() {
            Some('d' | 'i') => (next(cpu) as i32).to_string(),
            Some('u') => next(cpu).to_string(),
            Some('x') => format!("{:x}", next(cpu)),
            Some('X') => format!("{:X}", next(cpu)),
            Some('c') => ((next(cpu) as u8) as char).to_string(),
            Some('s') => cpu.string(next(cpu)),
            Some('%') => "%".to_string(),
            _ => String::new(),
        };
        let padding = width.saturating_sub(text.chars().count());
        if left {
            output.push_str(&text);
            output.push_str(&" ".repeat(padding));
        } else {
            output.push_str(&(if zero { "0" } else { " " }).repeat(padding));
            output.push_str(&text);
        }
    }
    output
}

fn peek_message(cpu: &mut Cpu, at: u32, flags: u32) -> u32 {
    const REMOVE: u32 = 1;
    script::advance(cpu);
    video::pump(cpu);
    gamepad::advance(cpu);
    audio::pump(cpu);
    let Some(&(message, wparam, lparam)) = cpu.host.messages.front() else { return 0 };
    if flags & REMOVE != 0 {
        cpu.host.messages.pop_front();
    }
    cpu.fill(at, MSG_SIZE, 0);
    let window = cpu.host.window;
    cpu.w32(at, window);
    cpu.w32(at + 4, message);
    cpu.w32(at + 8, wparam);
    cpu.w32(at + 12, lparam);
    let ticks = cpu.host.started.elapsed().as_millis() as u32;
    cpu.w32(at + 16, ticks);
    let (x, y) = cpu.host.pointer;
    cpu.w32(at + 20, x);
    cpu.w32(at + 24, y);
    1
}

pub fn find_user(name: &str) -> Option<GuestFn> {
    table! { name;
        "wsprintfA" 0 |cpu| {
            let (target, pattern) = (cpu.arg(0), cpu.string(cpu.arg(1)));
            let mut index = 2;
            let text = format(cpu, &pattern, |cpu| { index += 1; cpu.arg(index - 1) });
            cpu.write_string(target, &text);
            text.len() as u32
        }
        "wvsprintfA" 3 |cpu| {
            let (target, pattern, mut list) = (cpu.arg(0), cpu.string(cpu.arg(1)), cpu.arg(2));
            let text = format(cpu, &pattern, |cpu| { list += 4; cpu.r32(list - 4) });
            cpu.write_string(target, &text);
            text.len() as u32
        }
        "RegisterClassA" 1 |cpu| { let class = cpu.arg(0); cpu.host.window_procedure = cpu.r32(class + 4); 1 }
        "CreateWindowExA" 12 |cpu| {
            // Later windows are the game's child controls, such as list boxes.
            // Only the first is the real one: the others just need a handle.
            let window = cpu.host.handle();
            if cpu.host.window != 0 {
                return_value!(window);
            }
            cpu.host.window = window;
            cpu.host.video = video::Video::open(cpu.host.title, cpu.host.icon);
            cpu.fill(0x7ffc_0000, 64, 0);
            send(cpu, WM_CREATE, 0, 0x7ffc_0000);
            window
        }
        // At exit the game passes the handle of a launcher that isn't there.
        // Answering that would wake a game that has already freed everything.
        "ShowWindow" 2 |cpu| {
            if cpu.arg(0) != cpu.host.window || std::mem::replace(&mut cpu.host.window_shown, true) {
                return_false!();
            }
            send(cpu, WM_SHOWWINDOW, 1, 0);
            send(cpu, WM_ACTIVATEAPP, 1, 0);
            send(cpu, WM_ACTIVATE, 1, 0);
            send(cpu, WM_SETFOCUS, 0, 0);
            0
        }
        "UpdateWindow" 1 |cpu| { cpu.host.messages.push_back((WM_PAINT, 0, 0)); 1 }
        // The game ends its message loop from the WM_DESTROY this sends.
        "DestroyWindow" 1 |cpu| {
            if cpu.arg(0) != cpu.host.window {
                return_false!();
            }
            send(cpu, WM_DESTROY, 0, 0);
            send(cpu, WM_NCDESTROY, 0, 0);
            1
        }
        "SetWindowPos" 7 |_cpu| { 1 }
        "SetWindowTextA" 2 |_cpu| { 1 }
        "FindWindowA" 2 |_cpu| { 0 }
        "SetFocus" 1 |cpu| { cpu.host.window }
        "GetActiveWindow" 0 |cpu| { cpu.host.window }
        "EnableWindow" 2 |_cpu| { 0 }
        "SetCursor" 1 |_cpu| { 0 }
        "LoadCursorA" 2 |_cpu| { 0x10 }
        "LoadIconA" 2 |_cpu| { 0x14 }
        "GetSystemMetrics" 1 |cpu| {
            match cpu.arg(0) {
                0 => SCREEN_WIDTH,
                1 => SCREEN_HEIGHT,
                _ => 0,
            }
        }
        "SetCursorPos" 2 |cpu| { let (x, y) = (cpu.arg(0), cpu.arg(1)); video::set_cursor(cpu, x, y); 1 }
        "GetCursorPos" 1 |cpu| {
            let at = cpu.arg(0);
            let (x, y) = cpu.host.pointer;
            cpu.w32(at, x);
            cpu.w32(at + 4, y);
            1
        }
        "ClientToScreen" 2 |_cpu| { 1 }
        "ClipCursor" 1 |_cpu| { 1 }
        "GetClipCursor" 1 |cpu| {
            let at = cpu.arg(0);
            cpu.w32(at, 0);
            cpu.w32(at + 4, 0);
            cpu.w32(at + 8, SCREEN_WIDTH);
            cpu.w32(at + 12, SCREEN_HEIGHT);
            1
        }
        "GetWindowRect" 2 |cpu| {
            let at = cpu.arg(1);
            cpu.w32(at, 0);
            cpu.w32(at + 4, 0);
            cpu.w32(at + 8, SCREEN_WIDTH);
            cpu.w32(at + 12, SCREEN_HEIGHT);
            1
        }
        "PeekMessageA" 5 |cpu| { let (at, flags) = (cpu.arg(0), cpu.arg(4)); peek_message(cpu, at, flags) }
        // Waits for a message, and returns 0 for WM_QUIT, which ends the game's loop.
        "GetMessageA" 4 |cpu| {
            const WM_QUIT: u32 = 0x12;
            let at = cpu.arg(0);
            while peek_message(cpu, at, 1) == 0 {
                super::sleep(cpu, std::time::Duration::from_millis(1));
            }
            u32::from(cpu.r32(at + 4) != WM_QUIT)
        }
        // The scan code of the key that types a character, on a US keyboard,
        // with 2 in the high half when it needs Shift.
        "OemKeyScan" 1 |cpu| {
            const ROWS: [(&str, u32); 4] = [("1234567890-=", 2), ("qwertyuiop[]", 16), ("asdfghjkl;'`", 30), ("\\zxcvbnm,./", 43)];
            let character = (cpu.arg(0) as u8 as char).to_ascii_lowercase();
            let shift = if (cpu.arg(0) as u8).is_ascii_uppercase() { 2 << 16 } else { 0 };
            if character == ' ' {
                return 57;
            }
            ROWS.iter()
                .find_map(|(keys, first)| keys.find(character).map(|index| first + index as u32 | shift))
                .unwrap_or(0xffff_ffff)
        }
        "SetRect" 5 |cpu| {
            let at = cpu.arg(0);
            for index in 0..4 {
                let value = cpu.arg(1 + index);
                cpu.w32(at + index * 4, value);
            }
            1
        }
        "GetForegroundWindow" 0 |cpu| { cpu.host.window }
        "IsIconic" 1 |_cpu| { 0 }
        "ShowCursor" 1 |_cpu| { 0 }
        "InvalidateRect" 3 |_cpu| { 1 }
        "GetWindowLongA" 2 |_cpu| { 0 }
        "SetClassLongA" 3 |_cpu| { 0 }
        "RegisterWindowMessageA" 1 |_cpu| { 0xc000 }
        // The window has no menu bar: the game gets handles, and nothing is shown.
        "LoadMenuA" 2 |_cpu| { 0x40 }
        "GetMenu" 1 |_cpu| { 0x40 }
        "SetMenu" 2 |_cpu| { 1 }
        "DestroyMenu" 1 |_cpu| { 1 }
        "CheckMenuRadioItem" 5 |_cpu| { 1 }
        "EnableMenuItem" 3 |_cpu| { 0 }
        "SetMenuItemInfoA" 4 |_cpu| { 1 }
        "LoadAcceleratorsA" 2 |_cpu| { 0x44 }
        "TranslateAcceleratorA" 3 |_cpu| { 0 }
        "WaitMessage" 0 |_cpu| { std::thread::sleep(std::time::Duration::from_millis(1)); 1 }
        "TranslateMessage" 1 |_cpu| { 0 }
        "DispatchMessageA" 1 |cpu| {
            let at = cpu.arg(0);
            let (message, wparam, lparam) = (cpu.r32(at + 4), cpu.r32(at + 8), cpu.r32(at + 12));
            send(cpu, message, wparam, lparam)
        }
        "SendMessageA" 4 |cpu| {
            if cpu.arg(0) != cpu.host.window {
                return_false!(); // Child controls aren't simulated.
            }
            let (message, wparam, lparam) = (cpu.arg(1), cpu.arg(2), cpu.arg(3));
            send(cpu, message, wparam, lparam)
        }
        "DefWindowProcA" 4 |_cpu| { 0 }
        "PostQuitMessage" 1 |cpu| { cpu.host.messages.push_back((0x12, cpu.arg(0), 0)); 0 }
        "BeginPaint" 2 |cpu| { let at = cpu.arg(1); cpu.fill(at, 64, 0); 0x20 }
        "EndPaint" 2 |_cpu| { 1 }
        "GetDC" 1 |_cpu| { 0x20 }
        "GetWindowDC" 1 |_cpu| { 0x20 }
        "ReleaseDC" 2 |_cpu| { 1 }
        "FillRect" 3 |_cpu| { 1 }
        "MessageBoxA" 4 |cpu| {
            println!("message box: {}: {}", cpu.string(cpu.arg(2)), cpu.string(cpu.arg(1)));
            1
        }
    }
}

pub fn find_gdi(name: &str) -> Option<GuestFn> {
    table! { name;
        "CreateCompatibleDC" 1 |_cpu| { 0x24 }
        "DeleteDC" 1 |_cpu| { 1 }
        "GetDeviceCaps" 2 |cpu| {
            const BITS_PER_PIXEL: u32 = 12;
            const PLANES: u32 = 14;
            match cpu.arg(1) {
                BITS_PER_PIXEL => 16,
                PLANES => 1,
                _ => 0,
            }
        }
        "GetStockObject" 1 |_cpu| { 0x28 }
        "CreateSolidBrush" 1 |_cpu| { 0x2c }
        "CreateFontA" 14 |_cpu| { 0x30 }
        "CreatePalette" 1 |_cpu| { 0x34 }
        "SelectObject" 2 |_cpu| { 0x28 }
        "SelectPalette" 3 |_cpu| { 0x34 }
        "RealizePalette" 1 |_cpu| { 0 }
        "DeleteObject" 1 |_cpu| { 1 }
        "SetBkMode" 2 |_cpu| { 1 }
        "SetBkColor" 2 |_cpu| { 0 }
        "SetTextColor" 2 |_cpu| { 0 }
        "SetTextAlign" 2 |_cpu| { 0 }
        "TextOutA" 5 |_cpu| { 1 }
        "BitBlt" 9 |_cpu| { 1 }
        "StretchBlt" 11 |_cpu| { 1 }
        "GetSystemPaletteEntries" 4 |_cpu| { 0 }
        "SetDIBColorTable" 4 |_cpu| { 0 }
    }
}
