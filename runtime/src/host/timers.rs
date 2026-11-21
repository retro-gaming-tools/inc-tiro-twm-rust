//! Multimedia timers: `timeSetEvent` and `timeKillEvent`.
//!
//! Windows ran a timer's callback on a thread of its own. The translated game
//! has one thread, so the callback runs on it instead, between two
//! instructions, as a timer interrupt did under DOS: the generated code of a
//! game that imports `timeSetEvent` calls `Cpu::poll` on every jump backwards.
//! A small thread only watches the clock and raises `DUE`, which keeps the
//! poll down to one load.

use crate::cpu::Cpu;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// Raised by the watching thread when a timer's time has come.
pub static DUE: AtomicBool = AtomicBool::new(false);
/// When the next timer is due, in microseconds since `START`.
static NEXT: AtomicU64 = AtomicU64::new(u64::MAX);
static START: OnceLock<Instant> = OnceLock::new();

const PERIODIC: u32 = 1;
/// A timer further behind than this skips what it missed, so that a long
/// stall isn't followed by a burst of callbacks.
const FURTHEST_BEHIND: Duration = Duration::from_secs(1);

struct Timer {
    id: u32,
    period: Duration,
    next: Instant,
    callback: u32,
    user: u32,
    periodic: bool,
}

#[derive(Default)]
pub struct Timers {
    list: Vec<Timer>,
    /// A callback is running, and isn't interrupted by another.
    running: bool,
    last_id: u32,
    watched: bool,
}

impl Timers {
    /// Tells the watching thread when to raise `DUE` next.
    fn publish(&mut self) {
        let start = *START.get_or_init(Instant::now);
        let next = self.list.iter().map(|timer| timer.next).min();
        NEXT.store(next.map_or(u64::MAX, |at| at.saturating_duration_since(start).as_micros() as u64), Ordering::Relaxed);
        if !std::mem::replace(&mut self.watched, true) {
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_millis(1));
                if start.elapsed().as_micros() as u64 >= NEXT.load(Ordering::Relaxed) {
                    DUE.store(true, Ordering::Relaxed);
                }
            });
        }
    }
}

/// Everything a callback may change that the interrupted code still needs.
struct Registers {
    general: [u32; 8],
    flags: (bool, bool, bool, bool, bool, u8),
    st: [f64; 8],
    top: usize,
    conditions: (bool, bool, bool, bool),
    control: u16,
}

impl Registers {
    fn of(cpu: &Cpu) -> Registers {
        Registers {
            general: [cpu.eax, cpu.ecx, cpu.edx, cpu.ebx, cpu.esp, cpu.ebp, cpu.esi, cpu.edi],
            flags: (cpu.cf, cpu.zf, cpu.sf, cpu.of, cpu.df, cpu.parity),
            st: cpu.st,
            top: cpu.top,
            conditions: (cpu.c0, cpu.c1, cpu.c2, cpu.c3),
            control: cpu.fpu_control,
        }
    }

    fn restore(self, cpu: &mut Cpu) {
        [cpu.eax, cpu.ecx, cpu.edx, cpu.ebx, cpu.esp, cpu.ebp, cpu.esi, cpu.edi] = self.general;
        (cpu.cf, cpu.zf, cpu.sf, cpu.of, cpu.df, cpu.parity) = self.flags;
        cpu.st = self.st;
        cpu.top = self.top;
        (cpu.c0, cpu.c1, cpu.c2, cpu.c3) = self.conditions;
        cpu.fpu_control = self.control;
    }
}

/// Runs the callbacks that are due, once for each period that has passed.
#[cold]
#[inline(never)]
pub fn run(cpu: &mut Cpu) {
    if cpu.host.timers.running {
        return;
    }
    DUE.store(false, Ordering::Relaxed);
    cpu.host.timers.running = true;
    let saved = Registers::of(cpu);
    loop {
        let now = Instant::now();
        let timers = &mut cpu.host.timers.list;
        let Some(index) = timers.iter().position(|timer| timer.next <= now) else { break };
        let timer = &mut timers[index];
        let (id, callback, user) = (timer.id, timer.callback, timer.user);
        if timer.periodic {
            timer.next += timer.period;
            if now.saturating_duration_since(timer.next) > FURTHEST_BEHIND {
                timer.next = now;
            }
        } else {
            timers.remove(index);
        }
        cpu.df = false;
        super::call_guest(cpu, callback, &[id, 0, user, 0, 0]);
    }
    saved.restore(cpu);
    cpu.host.timers.running = false;
    cpu.host.timers.publish();
}

/// `timeSetEvent(delay, resolution, callback, user, flags)`: the timer's id.
pub fn set_event(cpu: &mut Cpu) -> u32 {
    let (delay, callback, user, flags) = (cpu.arg(0), cpu.arg(2), cpu.arg(3), cpu.arg(4));
    let period = Duration::from_millis(delay.max(1) as u64);
    let timers = &mut cpu.host.timers;
    timers.last_id += 1;
    let id = timers.last_id;
    timers.list.push(Timer { id, period, next: Instant::now() + period, callback, user, periodic: flags & PERIODIC != 0 });
    timers.publish();
    id
}

pub fn kill_event(cpu: &mut Cpu) -> u32 {
    let id = cpu.arg(0);
    let timers = &mut cpu.host.timers;
    timers.list.retain(|timer| timer.id != id);
    timers.publish();
    0
}
