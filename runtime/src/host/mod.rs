//! Replacements for the Windows functions a game imports.

pub mod audio;
pub mod ddraw;
pub mod gamepad;
mod gl;
pub mod glide;
mod glide_gl;
mod glide_raster;
mod kernel32;
pub mod launcher;
mod other;
pub mod pool;
pub mod registry;
pub mod script;
mod user32;
pub mod video;

use crate::cpu::{Cpu, GuestFn, Host, IMPORT_BASE};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

/// Builds a lookup from names to functions. Each entry is a name, the number of
/// stack arguments it pops, and a body that returns the value for EAX.
macro_rules! table {
    ($name:expr; $($key:literal $arguments:literal |$cpu:ident| $body:block)*) => {
        match $name {
            $($key => Some((|$cpu: &mut Cpu| {
                let body = |$cpu: &mut Cpu| -> u32 { $body };
                let result = body($cpu);
                $cpu.finish(result, $arguments);
            }) as GuestFn),)*
            _ => None,
        }
    };
}
pub(crate) use table;

/// The screen the game addresses.
pub const GAME_WIDTH: usize = 640;
pub const GAME_HEIGHT: usize = 480;
/// The game's clock in milliseconds, as `GetTickCount` gives it, for mods.
pub fn tick_count(cpu: &mut Cpu) -> u32 {
    kernel32::tick_count(cpu)
}

/// For a mod that replaces a game's frame limiter: the game wants at most
/// `fps` frames a second. The runtime then waits at the next swap or flip
/// instead, just before the frame is shown, so frames reach the screen evenly
/// however long each took to draw, and in step with the display's refresh.
/// The limiter itself should return at once. A period nobody renews for a
/// quarter of a second lapses, and frames are paced by the refresh alone.
///
/// Some screens show frames without a swap or flip, such as DirectDraw menus
/// that draw straight to the primary surface. When no frame was paced since
/// the last call, this waits itself, a period after that call, as the game's
/// own limiter did; otherwise those screens would run as fast as they can.
pub fn pace_frames(cpu: &mut Cpu, fps: f64) {
    use std::time::Instant;
    if fps <= 0.0 {
        return;
    }
    let period = std::time::Duration::from_secs_f64(1.0 / fps);
    if let Some((previous, called)) = cpu.host.frame_period.filter(|_| !cpu.host.paced_since_limiter) {
        let target = called + previous.min(period);
        let now = Instant::now();
        if target > now {
            sleep(cpu, target - now);
        }
    }
    let now = Instant::now();
    // Keep the beat from the last target unless the game fell behind it, so
    // sleeping late now and then doesn't slow the average.
    let mark = match cpu.host.frame_period {
        Some((previous, called)) if !cpu.host.paced_since_limiter && now < called + previous * 2 => (called + previous).max(now - period / 2),
        _ => now,
    };
    cpu.host.frame_period = Some((period, mark));
    cpu.host.paced_since_limiter = false;
}

/// Whether the player left an optional mod on, by its file name in `mods/`.
/// A mod with a launcher row checks this first, and calls the original when
/// it's off, so turning it off gives the game as it was.
pub fn mod_enabled(cpu: &Cpu, file: &str) -> bool {
    !cpu.host.mods_off.contains(&file)
}

/// Asks the player a yes or no question in a box, for mods. See `video::ask`.
pub fn ask(cpu: &mut Cpu, title: &str, text: &str, yes: &str, no: &str) -> bool {
    video::ask(cpu, title, text, yes, no)
}

/// The game's command line, as `recompCommand` and the launcher made it, such
/// as `Incubation.exe -english -3dfx`. Mods can read a language switch from it.
pub fn command(cpu: &Cpu) -> String {
    std::env::var("RECOMP_COMMAND").unwrap_or_else(|_| cpu.host.command.to_string())
}

/// Where a game path, such as `DATA\\TXT\\German\\GUI\\Campaign.lcl`, is on disk,
/// ignoring case as Windows does. For mods that read game files themselves.
pub fn game_path(cpu: &Cpu, guest: &str) -> PathBuf {
    kernel32::host_path(cpu, guest)
}

/// Opens bytes a mod made as a read-only file, and returns its handle, which
/// `ReadFile`, `SetFilePointer`, `GetFileSize` and `CloseHandle` take as usual.
pub fn open_memory(cpu: &mut Cpu, path: PathBuf, data: Vec<u8>) -> u32 {
    let handle = cpu.host.handle();
    cpu.host.files.insert(handle, OpenFile { file: FileData::Memory(std::io::Cursor::new(data)), path });
    handle
}

/// Calls a function of the game that takes its arguments on the stack and pops
/// them itself, such as a callback it handed to Windows, and returns its result.
pub fn call_guest(cpu: &mut Cpu, function: u32, arguments: &[u32]) -> u32 {
    let saved = (cpu.ebx, cpu.esi, cpu.edi, cpu.ebp);
    for argument in arguments.iter().rev() {
        cpu.push(*argument);
    }
    cpu.push(0); // Its `ret` pops this and the arguments.
    cpu.call(function);
    (cpu.ebx, cpu.esi, cpu.edi, cpu.ebp) = saved;
    cpu.eax
}

/// Sleeps, and counts the time for the script's `fps` step. For mods too.
pub fn sleep(cpu: &mut Cpu, time: std::time::Duration) {
    let started = std::time::Instant::now();
    std::thread::sleep(time);
    cpu.host.waited += started.elapsed().as_secs_f64();
}

pub struct LoadedModule {
    pub name: String,
    pub base: u32,
    pub entry: u32,
    pub exports: Vec<crate::pe::Export>,
    pub started: bool,
}

pub struct OpenFile {
    pub file: FileData,
    pub path: PathBuf,
}

/// An open file's bytes: on disk, or made by a mod and kept in memory.
pub enum FileData {
    Disk(std::fs::File),
    Memory(std::io::Cursor<Vec<u8>>),
}

impl FileData {
    pub fn len(&self) -> u64 {
        match self {
            FileData::Disk(file) => file.metadata().map_or(0, |m| m.len()),
            FileData::Memory(data) => data.get_ref().len() as u64,
        }
    }

    pub fn set_len(&mut self, size: u64) {
        match self {
            FileData::Disk(file) => { let _ = file.set_len(size); }
            FileData::Memory(data) => data.get_mut().resize(size as usize, 0),
        }
    }
}

impl std::io::Read for FileData {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self { FileData::Disk(f) => f.read(buf), FileData::Memory(m) => m.read(buf) }
    }
}

impl std::io::Write for FileData {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self { FileData::Disk(f) => f.write(buf), FileData::Memory(m) => m.write(buf) }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self { FileData::Disk(f) => f.flush(), FileData::Memory(m) => m.flush() }
    }
}

impl std::io::Seek for FileData {
    fn seek(&mut self, to: std::io::SeekFrom) -> std::io::Result<u64> {
        match self { FileData::Disk(f) => f.seek(to), FileData::Memory(m) => m.seek(to) }
    }
}

pub struct Search {
    pub entries: Vec<(String, bool, u64)>,
    pub position: usize,
}

const HEAP_START: u32 = 0x2000_0000;
const VIRTUAL_START: u32 = 0x6000_0000;

/// Everything the runtime keeps for the game between calls.
pub struct State {
    pub folder: PathBuf,
    pub modules: Vec<LoadedModule>,
    pub last_error: u32,
    pub heap_next: u32,
    pub free: HashMap<u32, Vec<u32>>,
    pub virtual_next: u32,
    pub tls: Vec<u32>,
    pub files: HashMap<u32, OpenFile>,
    pub searches: HashMap<u32, Search>,
    pub next_handle: u32,
    pub command_line: u32,
    pub command_line_wide: u32,
    pub environment: u32,
    pub environment_wide: u32,
    pub started: std::time::Instant,
    pub window: u32,
    pub window_shown: bool,
    pub window_procedure: u32,
    pub messages: VecDeque<(u32, u32, u32)>,
    pub pointer: (u32, u32),
    pub pad: gamepad::Pad,
    /// The clock's last value, and how many reads in a row have returned it.
    pub clock_reads: (u32, u32),
    pub display: (u32, u32),
    pub draw: ddraw::Draw,
    pub script: script::Script,
    pub video: Option<video::Video>,
    pub audio: audio::Audio,
    pub glide: Option<glide::Glide>,
    pub title: &'static str,
    pub command: &'static str,
    pub icon: &'static [u8],
    /// CD music: a file prefix such as `music/Track`, to which `NN.ogg` is
    /// added, a guest address and the value it must hold for that prefix to be
    /// used. An address of 0 is the prefix used otherwise.
    pub music: &'static [(&'static str, u32, u32)],
    /// For the script's `fps` step: seconds asleep, and seconds putting frames on screen.
    pub waited: f64,
    pub presented: f64,
    /// How long the game wants each frame to take, from its frame limiter
    /// through `pace_frames`, and when it last said so.
    pub frame_period: Option<(std::time::Duration, std::time::Instant)>,
    /// Whether a swap or flip waited for that period since `pace_frames` was
    /// last called. If not, the next call waits instead.
    pub paced_since_limiter: bool,
    /// Time between frames reaching the screen, in milliseconds, since the
    /// script's last `fps` step, which reports how even they were.
    pub shown_frames: Vec<f32>,
    /// The optional mods the player turned off, by file name.
    pub mods_off: Vec<&'static str>,
    pub registry: registry::Registry,
    /// The volume label of the game's CD, which is drive D:, or empty for none.
    pub cd: &'static str,
}

impl State {
    pub fn new(folder: &str) -> State {
        State {
            folder: PathBuf::from(folder),
            modules: Vec::new(),
            last_error: 0,
            heap_next: HEAP_START,
            free: HashMap::new(),
            virtual_next: VIRTUAL_START,
            tls: Vec::new(),
            files: HashMap::new(),
            searches: HashMap::new(),
            next_handle: 0x100,
            command_line: 0,
            command_line_wide: 0,
            environment: 0,
            environment_wide: 0,
            started: std::time::Instant::now(),
            window: 0,
            window_shown: false,
            window_procedure: 0,
            messages: VecDeque::new(),
            pointer: (320, 240),
            pad: gamepad::Pad::default(),
            clock_reads: (0, 0),
            display: (640, 480),
            draw: ddraw::Draw::default(),
            script: script::Script::from_environment(),
            video: None,
            audio: audio::Audio::default(),
            glide: None,
            title: "",
            command: "",
            icon: &[],
            music: &[],
            waited: 0.0,
            presented: 0.0,
            frame_period: None,
            paced_since_limiter: false,
            shown_frames: Vec::new(),
            mods_off: Vec::new(),
            registry: registry::Registry::default(),
            cd: "",
        }
    }

    pub fn handle(&mut self) -> u32 {
        self.next_handle += 4;
        self.next_handle
    }
}

impl Cpu {
    /// The nth stack argument of an import, counted from 0.
    #[inline(always)]
    pub fn arg(&self, index: u32) -> u32 {
        self.r32(self.esp.wrapping_add(4 + index * 4))
    }

    /// Returns from an import, popping the return address and `arguments` values.
    #[inline(always)]
    pub fn finish(&mut self, result: u32, arguments: u32) {
        self.eax = result;
        self.esp = self.esp.wrapping_add(4 + arguments * 4);
    }

    pub fn string(&self, mut at: u32) -> String {
        let mut bytes = Vec::new();
        if at != 0 {
            loop {
                let byte = self.r8(at);
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
                at += 1;
            }
        }
        bytes.iter().map(|&b| b as char).collect()
    }

    pub fn wide_string(&self, mut at: u32) -> String {
        let mut text = String::new();
        if at != 0 {
            loop {
                let unit = self.r16(at);
                if unit == 0 {
                    break;
                }
                text.push(char::from_u32(unit as u32).unwrap_or('?'));
                at += 2;
            }
        }
        text
    }

    pub fn write_string(&mut self, at: u32, text: &str) {
        for (index, character) in text.chars().enumerate() {
            self.w8(at + index as u32, character as u32 as u8);
        }
        self.w8(at + text.chars().count() as u32, 0);
    }

    pub fn write_wide(&mut self, at: u32, text: &str) {
        for (index, character) in text.chars().enumerate() {
            self.w16(at + index as u32 * 2, character as u32 as u16);
        }
        self.w16(at + text.chars().count() as u32 * 2, 0);
    }

    pub fn fill(&mut self, at: u32, size: u32, value: u8) {
        unsafe { std::ptr::write_bytes(self.memory.add(at as usize), value, size as usize) }
    }

    /// Makes sure memory exists before the system writes to it. On Windows a
    /// page is committed by a fault, which system calls don't raise. Volatile
    /// keeps the compiler from removing a write of the value just read.
    pub fn touch(&mut self, at: u32, size: u32) {
        const PAGE: u32 = 4096;
        for page in (at & !(PAGE - 1)..at.saturating_add(size)).step_by(PAGE as usize) {
            let address = unsafe { self.memory.add(page.max(at) as usize) };
            unsafe { address.write_volatile(address.read_volatile()) };
        }
    }

    pub fn bytes(&self, at: u32, size: u32) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.memory.add(at as usize), size as usize) }
    }

    pub fn bytes_mut(&mut self, at: u32, size: u32) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.memory.add(at as usize), size as usize) }
    }

    /// A small heap for the game: blocks of a size go back on that size's list.
    pub fn allocate(&mut self, size: u32) -> u32 {
        let rounded = (size.max(1) + 15) & !15;
        if let Some(block) = self.host.free.get_mut(&rounded).and_then(|list| list.pop()) {
            return block;
        }
        let block = self.host.heap_next + 16;
        self.host.heap_next = block + rounded;
        assert!(self.host.heap_next < VIRTUAL_START, "the game's heap is full");
        self.w32(block - 16, rounded);
        self.w32(block - 12, size);
        block
    }

    pub fn release(&mut self, block: u32) {
        if block != 0 {
            let rounded = self.r32(block - 16);
            self.host.free.entry(rounded).or_default().push(block);
        }
    }

    pub fn block_size(&self, block: u32) -> u32 {
        self.r32(block - 12)
    }

    /// A pointer to a host function, for GetProcAddress and for COM tables.
    pub fn thunk(&mut self, name: &str, function: Option<GuestFn>) -> u32 {
        self.imports.push(Host { name: name.to_string(), function });
        IMPORT_BASE + self.imports.len() as u32 - 1
    }

    /// Runs a DLL's start-up routine once, as Windows does when it loads one.
    pub fn start_module(&mut self, index: usize) {
        if std::mem::replace(&mut self.host.modules[index].started, true) {
            return;
        }
        let (base, entry) = (self.host.modules[index].base, self.host.modules[index].entry);
        if entry == 0 {
            return;
        }
        const PROCESS_ATTACH: u32 = 1;
        self.push(0);
        self.push(PROCESS_ATTACH);
        self.push(base);
        self.push(0);
        self.call(entry);
    }
}

/// The Rust version of an import, if there is one yet.
pub fn find(dll: &str, name: &str) -> Option<GuestFn> {
    match dll {
        "kernel32.dll" => kernel32::find(name),
        "user32.dll" => user32::find_user(name),
        "gdi32.dll" => user32::find_gdi(name),
        "ddraw.dll" => ddraw::find(name),
        "glide2x.dll" => glide::find(name),
        "advapi32.dll" => registry::find(name),
        "dsound.dll" => audio::find_dsound(name),
        "dplayx.dll" => other::find_dplay(name),
        "audio.dll" => other::find_audio(name),
        "winmm.dll" => other::find_winmm(name),
        "comctl32.dll" => other::find_comctl(name),
        _ => None,
    }
}
