//! Small DLLs.

use super::{ddraw, table};
use crate::cpu::{Cpu, GuestFn};

const DSERR_GENERIC: u32 = 0x8000_4005;

/// The game needs a lobby object to start, and only asks it how it was launched.
fn create_lobby(cpu: &mut Cpu) -> u32 {
    const SLOTS: u32 = 16;
    const GET_CONNECTION_SETTINGS: u32 = 0x20;
    const DPERR_NOTLOBBIED: u32 = 0x8877_042e;
    let methods: [ddraw::Method; 3] = [
        (0x04, 1, (|cpu: &mut Cpu| cpu.finish(1, 1)) as GuestFn),
        (0x08, 1, (|cpu: &mut Cpu| cpu.finish(0, 1)) as GuestFn),
        (GET_CONNECTION_SETTINGS, 4, (|cpu: &mut Cpu| cpu.finish(DPERR_NOTLOBBIED, 4)) as GuestFn),
    ];
    let table = ddraw::build_vtable(cpu, "IDirectPlayLobby", SLOTS, &methods);
    let lobby = ddraw::object(cpu, table);
    let out = cpu.arg(1);
    cpu.w32(out, lobby);
    0
}

/// Multiplayer isn't included, so DirectPlay itself reports that it isn't available.
pub fn find_dplay(name: &str) -> Option<GuestFn> {
    table! { name;
        "#1" 3 |_cpu| { DSERR_GENERIC }
        "#2" 2 |_cpu| { DSERR_GENERIC }
        "#4" 5 |cpu| { create_lobby(cpu) }
    }
}

/// MIDI music isn't played yet: the game can open a stream and queue data, and
/// hears nothing. No "done" callback comes back either.
pub fn find_winmm(name: &str) -> Option<GuestFn> {
    const HEADER_FLAGS: u32 = 16;
    const PREPARED: u32 = 2;
    table! { name;
        "midiOutGetDevCapsA" 3 |cpu| { let (at, size) = (cpu.arg(1), cpu.arg(2)); cpu.fill(at, size, 0); 0 }
        "midiStreamOpen" 6 |cpu| { let at = cpu.arg(0); let handle = cpu.host.handle(); cpu.w32(at, handle); 0 }
        "midiStreamProperty" 3 |_cpu| { 0 }
        "midiStreamOut" 3 |_cpu| { 0 }
        "midiStreamRestart" 1 |_cpu| { 0 }
        "midiStreamPause" 1 |_cpu| { 0 }
        "midiStreamStop" 1 |_cpu| { 0 }
        "midiStreamClose" 1 |_cpu| { 0 }
        "midiOutPrepareHeader" 3 |cpu| { let at = cpu.arg(1) + HEADER_FLAGS; let flags = cpu.r32(at); cpu.w32(at, flags | PREPARED); 0 }
        "midiOutUnprepareHeader" 3 |cpu| { let at = cpu.arg(1) + HEADER_FLAGS; let flags = cpu.r32(at); cpu.w32(at, flags & !PREPARED); 0 }
        "midiOutReset" 1 |_cpu| { 0 }
        "midiOutSetVolume" 2 |_cpu| { 0 }
    }
}

/// Common controls are never drawn: the game gets handles, and nothing is shown.
pub fn find_comctl(name: &str) -> Option<GuestFn> {
    table! { name;
        "#17" 0 |_cpu| { 0 } // InitCommonControls
        "ImageList_Create" 5 |cpu| { cpu.host.handle() }
        "ImageList_ReplaceIcon" 3 |_cpu| { 0 }
    }
}

pub fn find_audio(name: &str) -> Option<GuestFn> {
    if let Some(function) = super::audio::find_mci(name) {
        return Some(function);
    }
    table! { name;
        "mciGetErrorStringA" 3 |cpu| { let at = cpu.arg(1); cpu.w8(at, 0); 1 }
    }
}
