//! kernel32: memory, files, time and DLLs. There is one thread, so locks do nothing.

use super::table;
use crate::cpu::{Cpu, GuestFn};
use crate::host::{FileData, OpenFile, Search};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const INVALID: u32 = 0xffff_ffff;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_NO_MORE_FILES: u32 = 18;
const GAME_DIR: &str = "C:\\GAME";
const HEAP_ZERO_MEMORY: u32 = 8;
const FILETIME_1970: u64 = 116_444_736_000_000_000;

/// Finds a game path on disk, ignoring case, as Windows does.
pub(crate) fn host_path(cpu: &Cpu, guest: &str) -> PathBuf {
    let text = guest.replace('\\', "/");
    let lower = text.to_lowercase();
    let relative = lower.strip_prefix("c:/game/").or(lower.strip_prefix("c:/game")).map(|r| &text[text.len() - r.len()..]);
    let relative = relative.unwrap_or_else(|| text.strip_prefix("./").unwrap_or(&text)).trim_start_matches('/');
    let path = resolve(&cpu.host.folder, relative);
    // A path on the CD drive may be beside the game folder instead of in it: a
    // copy of a disc often has the program in one folder and the rest next to it.
    if lower.starts_with("d:") && !cpu.host.cd.is_empty() && !path.exists() {
        if let Some(beside) = cpu.host.folder.parent().map(|parent| resolve(parent, relative)).filter(|path| path.exists()) {
            return beside;
        }
    }
    path
}

fn resolve(folder: &Path, relative: &str) -> PathBuf {
    let mut path = folder.to_path_buf();
    // A drive name stays inside the game folder, instead of reaching the real drive.
    for part in relative.split('/').filter(|part| !part.is_empty() && *part != "." && !part.ends_with(':')) {
        let mut found: Vec<_> = std::fs::read_dir(&path)
            .map(|entries| entries.flatten().map(|e| e.file_name()).filter(|n| n.to_string_lossy().eq_ignore_ascii_case(part)).collect())
            .unwrap_or_default();
        found.sort();
        if found.len() > 1 {
            warn_duplicate(&path, &found);
        }
        // An exact match wins, and otherwise the first in sorted order, so the choice doesn't depend on the disk.
        let pick = found.iter().find(|n| n.to_string_lossy() == part).or(found.first()).cloned();
        path.push(pick.unwrap_or_else(|| part.into()));
    }
    path
}

/// Names that differ only in case are one file on Windows. On Linux, copying a patch over a game
/// can leave both the old file and the new one, and which the game gets is then a guess.
fn warn_duplicate(folder: &Path, names: &[std::ffi::OsString]) {
    static WARNED: std::sync::Mutex<Vec<PathBuf>> = std::sync::Mutex::new(Vec::new());
    let key = folder.join(names[0].to_ascii_lowercase());
    let mut warned = WARNED.lock().unwrap();
    if !warned.contains(&key) {
        let list: Vec<_> = names.iter().map(|n| n.to_string_lossy().into_owned()).collect();
        eprintln!("warning: {} has files that differ only in case: {}; keep one of them", folder.display(), list.join(", "));
        warned.push(key);
    }
}

fn create_file(cpu: &mut Cpu, name: String, access: u32, disposition: u32) -> u32 {
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const CREATE_NEW: u32 = 1;
    const CREATE_ALWAYS: u32 = 2;
    const OPEN_ALWAYS: u32 = 4;
    const TRUNCATE_EXISTING: u32 = 5;
    let path = host_path(cpu, &name);
    if cpu.trace {
        println!("  opening {name} as {} {}", path.display(), path.exists());
    }
    let writing = access & GENERIC_WRITE != 0;
    let mut options = std::fs::OpenOptions::new();
    options.read(true).write(writing);
    match disposition {
        CREATE_NEW | CREATE_ALWAYS | OPEN_ALWAYS => { options.write(true).create(true); }
        _ => {}
    }
    if disposition == CREATE_ALWAYS || disposition == TRUNCATE_EXISTING {
        options.truncate(true);
    }
    match options.open(&path) {
        Ok(file) => {
            let handle = cpu.host.handle();
            cpu.host.files.insert(handle, OpenFile { file: FileData::Disk(file), path });
            handle
        }
        Err(_) => {
            cpu.host.last_error = ERROR_FILE_NOT_FOUND;
            INVALID
        }
    }
}

/// Reads from an open file into guest memory, and returns how many bytes came.
fn read_file(cpu: &mut Cpu, handle: u32, at: u32, size: u32) -> u32 {
    cpu.touch(at, size);
    let target = cpu.bytes_mut(at, size) as *mut [u8];
    cpu.host.files.get_mut(&handle).map_or(0, |open| {
        let target = unsafe { &mut *target };
        let mut total = 0;
        while total < target.len() {
            match open.file.read(&mut target[total..]) {
                Ok(0) | Err(_) => break,
                Ok(count) => total += count,
            }
        }
        total as u32
    })
}

/// Fills a WIN32_FIND_DATAA.
fn write_find_data(cpu: &mut Cpu, at: u32, name: &str, directory: bool, size: u64) {
    const FIND_DATA_SIZE: u32 = 320;
    cpu.fill(at, FIND_DATA_SIZE, 0);
    cpu.w32(at, if directory { 0x10 } else { 0x20 });
    cpu.w32(at + 32, size as u32);
    cpu.write_string(at + 44, name);
}

fn wildcard(pattern: &[u8], text: &[u8]) -> bool {
    match (pattern.first(), text.first()) {
        (None, None) => true,
        (Some(b'*'), _) => wildcard(&pattern[1..], text) || (!text.is_empty() && wildcard(pattern, &text[1..])),
        (Some(b'?'), Some(_)) => wildcard(&pattern[1..], &text[1..]),
        (Some(p), Some(t)) if p.eq_ignore_ascii_case(t) => wildcard(&pattern[1..], &text[1..]),
        _ => false,
    }
}

fn find_first(cpu: &mut Cpu, pattern: String, data: u32) -> u32 {
    let text = pattern.replace('\\', "/");
    let (folder, mask) = text.rsplit_once('/').unwrap_or((".", &text));
    let directory = host_path(cpu, folder);
    let mask = if mask == "*.*" { "*" } else { mask };
    let mut entries: Vec<(String, bool, u64)> = std::fs::read_dir(directory)
        .map(|list| {
            list.flatten()
                .filter(|e| wildcard(mask.as_bytes(), e.file_name().to_string_lossy().as_bytes()))
                .map(|e| {
                    let meta = e.metadata().ok();
                    (e.file_name().to_string_lossy().to_string(), meta.as_ref().is_some_and(|m| m.is_dir()), meta.map_or(0, |m| m.len()))
                })
                .collect()
        })
        .unwrap_or_default();
    entries.sort();
    let Some((name, is_directory, size)) = entries.first().cloned() else {
        cpu.host.last_error = ERROR_FILE_NOT_FOUND;
        return INVALID;
    };
    write_find_data(cpu, data, &name, is_directory, size);
    let handle = cpu.host.handle();
    cpu.host.searches.insert(handle, Search { entries, position: 1 });
    handle
}

fn find_next(cpu: &mut Cpu, handle: u32, data: u32) -> u32 {
    let next = cpu.host.searches.get_mut(&handle).and_then(|search| {
        search.position += 1;
        search.entries.get(search.position - 1).cloned()
    });
    match next {
        Some((name, directory, size)) => {
            write_find_data(cpu, data, &name, directory, size);
            1
        }
        None => {
            cpu.host.last_error = ERROR_NO_MORE_FILES;
            0
        }
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as u64 / 100)
}

/// Writes a SYSTEMTIME for a count of 100 ns units since 1970.
fn write_system_time(cpu: &mut Cpu, at: u32, ticks: u64) {
    let seconds = ticks / 10_000_000;
    let days = (seconds / 86_400) as i64;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 { shifted + 3 } else { shifted - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    let values = [year, month, (days + 4) % 7, day, (seconds / 3600 % 24) as i64, (seconds / 60 % 60) as i64, (seconds % 60) as i64, (ticks / 10_000 % 1000) as i64];
    for (index, value) in values.iter().enumerate() {
        cpu.w16(at + index as u32 * 2, *value as u16);
    }
}

fn module_named(cpu: &Cpu, name: &str) -> Option<usize> {
    let wanted = name.rsplit(['\\', '/']).next().unwrap_or(name).to_lowercase();
    let wanted = if wanted.contains('.') { wanted } else { format!("{wanted}.dll") };
    cpu.host.modules.iter().position(|module| module.name == wanted)
}

fn load_library(cpu: &mut Cpu, name: String) -> u32 {
    if let Some(index) = module_named(cpu, &name) {
        cpu.start_module(index);
        return cpu.host.modules[index].base;
    }
    0
}

fn get_proc_address(cpu: &mut Cpu, module: u32, wanted: u32) -> u32 {
    let name = if wanted < 0x1_0000 { None } else { Some(cpu.string(wanted)) };
    let found = cpu.host.modules.iter().find(|m| m.base == module).and_then(|m| {
        m.exports.iter().find(|e| match &name {
            Some(name) => e.name.as_deref() == Some(name),
            None => e.ordinal == wanted,
        })
    });
    found.map_or(0, |export| export.address)
}

fn wide_to_narrow(cpu: &mut Cpu, source: u32, count: u32, target: u32, capacity: u32) -> u32 {
    let text: Vec<u16> = if count == INVALID {
        let mut units = Vec::new();
        let mut at = source;
        loop {
            let unit = cpu.r16(at);
            units.push(unit);
            at += 2;
            if unit == 0 {
                break;
            }
        }
        units
    } else {
        (0..count).map(|i| cpu.r16(source + i * 2)).collect()
    };
    if capacity != 0 {
        for (index, unit) in text.iter().enumerate().take(capacity as usize) {
            cpu.w8(target + index as u32, *unit as u8);
        }
    }
    text.len() as u32
}

fn narrow_to_wide(cpu: &mut Cpu, source: u32, count: u32, target: u32, capacity: u32) -> u32 {
    let text: Vec<u8> = if count == INVALID {
        let mut bytes = cpu.string(source).chars().map(|c| c as u32 as u8).collect::<Vec<u8>>();
        bytes.push(0);
        bytes
    } else {
        cpu.bytes(source, count).to_vec()
    };
    if capacity != 0 {
        for (index, byte) in text.iter().enumerate().take(capacity as usize) {
            cpu.w16(target + index as u32 * 2, *byte as u16);
        }
    }
    text.len() as u32
}

fn character_type(byte: u32) -> u16 {
    let c = byte as u8;
    let mut kind = 0x200; // Everything counts as defined.
    if c.is_ascii_uppercase() { kind |= 0x1 | 0x100 }
    if c.is_ascii_lowercase() { kind |= 0x2 | 0x100 }
    if c.is_ascii_digit() { kind |= 0x4 }
    if c == b' ' || (9..=13).contains(&c) { kind |= 0x8 }
    if c.is_ascii_punctuation() { kind |= 0x10 }
    if c < 32 || c == 127 { kind |= 0x20 }
    if c == b' ' { kind |= 0x40 }
    if c.is_ascii_hexdigit() { kind |= 0x80 }
    kind
}

/// LCMapString: only case conversion, in the Windows-1252 subset ASCII covers.
fn map_case(flags: u32, value: u32) -> u32 {
    const LOWER: u32 = 0x100;
    const UPPER: u32 = 0x200;
    let byte = value as u8;
    if flags & UPPER != 0 {
        byte.to_ascii_uppercase() as u32
    } else if flags & LOWER != 0 {
        byte.to_ascii_lowercase() as u32
    } else {
        value
    }
}

/// The clock in milliseconds. Games of this era wait by reading it in a loop, so
/// repeated reads of the same value rest for a moment instead of using a core.
pub(crate) fn tick_count(cpu: &mut Cpu) -> u32 {
    const READS_BEFORE_RESTING: u32 = 8;
    const REST: std::time::Duration = std::time::Duration::from_micros(200);
    let now = cpu.host.started.elapsed().as_millis() as u32;
    let (last, reads) = cpu.host.clock_reads;
    let reads = if now == last { reads + 1 } else { 0 };
    cpu.host.clock_reads = (now, reads);
    if reads >= READS_BEFORE_RESTING {
        super::sleep(cpu, REST);
    }
    now
}

fn set_up_strings(cpu: &mut Cpu) {
    if cpu.host.command_line == 0 {
        let line = std::env::var("RECOMP_COMMAND").unwrap_or_else(|_| cpu.host.command.to_string());
        let narrow = cpu.allocate(line.len() as u32 + 1);
        cpu.write_string(narrow, &line);
        let wide = cpu.allocate(line.len() as u32 * 2 + 2);
        cpu.write_wide(wide, &line);
        cpu.host.command_line = narrow;
        cpu.host.command_line_wide = wide;
        cpu.host.environment = cpu.allocate(4);
        cpu.fill(cpu.host.environment, 4, 0);
        cpu.host.environment_wide = cpu.host.environment;
    }
}

pub fn find(name: &str) -> Option<GuestFn> {
    table! { name;
        "GetLastError" 0 |cpu| { cpu.host.last_error }
        "SetLastError" 1 |cpu| { cpu.host.last_error = cpu.arg(0); 0 }
        // Windows 98, so nothing takes the NT paths, and the game's check for
        // DirectX 5 passes on the build number alone.
        "GetVersion" 0 |_cpu| { 0xc000_0a04 }
        "GetVersionExA" 1 |cpu| {
            let at = cpu.arg(0);
            cpu.w32(at + 4, 4);
            cpu.w32(at + 8, 10);
            cpu.w32(at + 12, 1998);
            cpu.w32(at + 16, 1);
            cpu.w8(at + 20, 0);
            1
        }
        "GetSystemInfo" 1 |cpu| {
            let at = cpu.arg(0);
            cpu.fill(at, 36, 0);
            cpu.w32(at + 4, 4096);
            cpu.w32(at + 8, 0x10000);
            cpu.w32(at + 12, 0x7fff_0000);
            cpu.w32(at + 16, 1);
            cpu.w32(at + 20, 1);
            cpu.w32(at + 24, 586);
            cpu.w32(at + 28, 0x10000);
            0
        }
        "GlobalMemoryStatus" 1 |cpu| {
            let at = cpu.arg(0);
            const MEGABYTES_64: u32 = 64 << 20;
            cpu.w32(at, 32);
            cpu.w32(at + 4, 50);
            for offset in [8, 12, 16, 20, 24, 28] {
                cpu.w32(at + offset, MEGABYTES_64);
            }
            0
        }
        "GetCommandLineA" 0 |cpu| { set_up_strings(cpu); cpu.host.command_line }
        "GetCommandLineW" 0 |cpu| { set_up_strings(cpu); cpu.host.command_line_wide }
        "GetEnvironmentStrings" 0 |cpu| { set_up_strings(cpu); cpu.host.environment }
        "GetEnvironmentStringsW" 0 |cpu| { set_up_strings(cpu); cpu.host.environment_wide }
        "FreeEnvironmentStringsA" 1 |_cpu| { 1 }
        "FreeEnvironmentStringsW" 1 |_cpu| { 1 }
        "SetEnvironmentVariableA" 2 |_cpu| { 1 }
        "GetStartupInfoA" 1 |cpu| { let at = cpu.arg(0); cpu.fill(at, 68, 0); cpu.w32(at, 68); 0 }
        "GetModuleHandleA" 1 |cpu| {
            let name = cpu.arg(0);
            if name == 0 {
                cpu.host.modules[0].base
            } else {
                let text = cpu.string(name);
                module_named(cpu, &text).map_or(0, |index| cpu.host.modules[index].base)
            }
        }
        "GetModuleFileNameA" 3 |cpu| {
            let (module, at) = (cpu.arg(0), cpu.arg(1));
            let name = cpu.host.modules.iter().find(|m| m.base == module).map_or("incubation.exe".to_string(), |m| m.name.clone());
            let text = format!("{GAME_DIR}\\{name}");
            cpu.write_string(at, &text);
            text.len() as u32
        }
        "LoadLibraryA" 1 |cpu| { let name = cpu.string(cpu.arg(0)); load_library(cpu, name) }
        "FreeLibrary" 1 |_cpu| { 1 }
        "GetProcAddress" 2 |cpu| { let (module, wanted) = (cpu.arg(0), cpu.arg(1)); get_proc_address(cpu, module, wanted) }
        "DisableThreadLibraryCalls" 1 |_cpu| { 1 }

        "HeapCreate" 3 |_cpu| { 1 }
        "HeapDestroy" 1 |_cpu| { 1 }
        "HeapValidate" 3 |_cpu| { 1 }
        "HeapCompact" 2 |_cpu| { 0 }
        "HeapAlloc" 3 |cpu| {
            let (flags, size) = (cpu.arg(1), cpu.arg(2));
            let block = cpu.allocate(size);
            if flags & HEAP_ZERO_MEMORY != 0 { cpu.fill(block, size, 0); }
            block
        }
        "HeapFree" 3 |cpu| { let block = cpu.arg(2); cpu.release(block); 1 }
        "HeapSize" 3 |cpu| { cpu.block_size(cpu.arg(2)) }
        "HeapReAlloc" 4 |cpu| {
            let (flags, old, size) = (cpu.arg(1), cpu.arg(2), cpu.arg(3));
            let block = cpu.allocate(size);
            let kept = cpu.block_size(old).min(size);
            if flags & HEAP_ZERO_MEMORY != 0 { cpu.fill(block, size, 0); }
            unsafe { std::ptr::copy(cpu.memory.add(old as usize), cpu.memory.add(block as usize), kept as usize) };
            cpu.release(old);
            block
        }
        "VirtualAlloc" 4 |cpu| {
            let (wanted, size) = (cpu.arg(0), cpu.arg(1));
            if wanted != 0 {
                wanted
            } else {
                let at = cpu.host.virtual_next;
                cpu.host.virtual_next = (at + size + 0xffff) & !0xffff;
                at
            }
        }
        "VirtualFree" 3 |_cpu| { 1 }
        "GlobalAlloc" 2 |cpu| { let size = cpu.arg(1); let block = cpu.allocate(size); cpu.fill(block, size, 0); block }
        "GlobalFree" 1 |cpu| { let block = cpu.arg(0); cpu.release(block); 0 }

        "TlsAlloc" 0 |cpu| { cpu.host.tls.push(0); cpu.host.tls.len() as u32 - 1 }
        "TlsFree" 1 |_cpu| { 1 }
        "TlsGetValue" 1 |cpu| { cpu.host.last_error = 0; cpu.host.tls[cpu.arg(0) as usize] }
        "TlsSetValue" 2 |cpu| { let (index, value) = (cpu.arg(0) as usize, cpu.arg(1)); cpu.host.tls[index] = value; 1 }

        "InitializeCriticalSection" 1 |_cpu| { 0 }
        "DeleteCriticalSection" 1 |_cpu| { 0 }
        "EnterCriticalSection" 1 |_cpu| { 0 }
        "LeaveCriticalSection" 1 |_cpu| { 0 }
        "CreateMutexA" 3 |cpu| { cpu.host.last_error = 0; cpu.host.handle() }
        "ReleaseMutex" 1 |_cpu| { 1 }
        "CreateEventA" 4 |cpu| { cpu.host.handle() }
        "SetEvent" 1 |_cpu| { 1 }
        "WaitForSingleObject" 2 |_cpu| { 0 }
        "GetCurrentProcess" 0 |_cpu| { 0xffff_ffff }
        "GetCurrentThread" 0 |_cpu| { 0xffff_fffe }
        "GetCurrentProcessId" 0 |_cpu| { 1 }
        "GetCurrentThreadId" 0 |_cpu| { 1 }
        "SetPriorityClass" 2 |_cpu| { 1 }
        "SetThreadPriority" 2 |_cpu| { 1 }
        "SetErrorMode" 1 |_cpu| { 0 }
        "SetConsoleCtrlHandler" 2 |_cpu| { 1 }
        "SetUnhandledExceptionFilter" 1 |_cpu| { 0 }
        "SetHandleCount" 1 |cpu| { cpu.arg(0) }
        "SetStdHandle" 2 |_cpu| { 1 }
        "GetStdHandle" 1 |cpu| { cpu.arg(0) }
        "GetFileType" 1 |cpu| { if cpu.host.files.contains_key(&cpu.arg(0)) { 1 } else { 2 } }
        "IsBadReadPtr" 2 |_cpu| { 0 }
        "IsBadWritePtr" 2 |_cpu| { 0 }
        "IsBadCodePtr" 1 |_cpu| { 0 }
        "Sleep" 1 |cpu| { super::sleep(cpu, std::time::Duration::from_millis(cpu.arg(0) as u64)); 0 }
        "Beep" 2 |_cpu| { 1 }
        "ExitProcess" 1 |cpu| { println!("the game exited with code {}", cpu.arg(0)); std::process::exit(cpu.arg(0) as i32) }
        "TerminateProcess" 2 |cpu| { println!("the game ended itself with code {}", cpu.arg(1)); std::process::exit(cpu.arg(1) as i32) }

        "GetTickCount" 0 |cpu| { tick_count(cpu) }
        "GetLocalTime" 1 |cpu| { let at = cpu.arg(0); write_system_time(cpu, at, now()); 0 }
        "GetSystemTime" 1 |cpu| { let at = cpu.arg(0); write_system_time(cpu, at, now()); 0 }
        "GetTimeZoneInformation" 1 |cpu| { let at = cpu.arg(0); cpu.fill(at, 172, 0); 0 }
        "FileTimeToLocalFileTime" 2 |cpu| { let value = cpu.r64(cpu.arg(0)); let at = cpu.arg(1); cpu.w64(at, value); 1 }
        "FileTimeToSystemTime" 2 |cpu| {
            let (from, at) = (cpu.arg(0), cpu.arg(1));
            let ticks = cpu.r64(from).saturating_sub(FILETIME_1970);
            write_system_time(cpu, at, ticks);
            1
        }

        "GetACP" 0 |_cpu| { 1252 }
        "GetOEMCP" 0 |_cpu| { 437 }
        "IsValidCodePage" 1 |_cpu| { 1 }
        "IsValidLocale" 2 |_cpu| { 1 }
        "GetUserDefaultLCID" 0 |_cpu| { 0x409 }
        "GetLocaleInfoA" 4 |cpu| { cpu.host.last_error = 1; 0 }
        "GetLocaleInfoW" 4 |cpu| { cpu.host.last_error = 1; 0 }
        "GetCPInfo" 2 |cpu| {
            let at = cpu.arg(1);
            cpu.fill(at, 18, 0);
            cpu.w32(at, 1);
            cpu.w8(at + 4, b'?');
            1
        }
        "MultiByteToWideChar" 6 |cpu| {
            let (source, count, target, capacity) = (cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            narrow_to_wide(cpu, source, count, target, capacity)
        }
        "WideCharToMultiByte" 8 |cpu| {
            let (source, count, target, capacity) = (cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            wide_to_narrow(cpu, source, count, target, capacity)
        }
        "GetStringTypeA" 5 |cpu| {
            let (source, count, target) = (cpu.arg(2), cpu.arg(3), cpu.arg(4));
            for index in 0..count {
                let kind = character_type(cpu.r8(source + index) as u32);
                cpu.w16(target + index * 2, kind);
            }
            1
        }
        "GetStringTypeW" 4 |cpu| {
            let (source, count, target) = (cpu.arg(1), cpu.arg(2), cpu.arg(3));
            for index in 0..count {
                let kind = character_type(cpu.r16(source + index * 2) as u32);
                cpu.w16(target + index * 2, kind);
            }
            1
        }
        "LCMapStringA" 6 |cpu| {
            let (flags, source, count, target, capacity) = (cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            let count = if count == INVALID { cpu.string(source).len() as u32 + 1 } else { count };
            if capacity != 0 {
                for index in 0..count.min(capacity) {
                    let value = map_case(flags, cpu.r8(source + index) as u32);
                    cpu.w8(target + index, value as u8);
                }
            }
            count
        }
        "LCMapStringW" 6 |cpu| {
            let (flags, source, count, target, capacity) = (cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            let count = if count == INVALID { cpu.wide_string(source).len() as u32 + 1 } else { count };
            if capacity != 0 {
                for index in 0..count.min(capacity) {
                    let value = map_case(flags, cpu.r16(source + index * 2) as u32);
                    cpu.w16(target + index * 2, value as u16);
                }
            }
            count
        }

        "lstrlenA" 1 |cpu| { cpu.string(cpu.arg(0)).len() as u32 }
        "lstrcpyA" 2 |cpu| { let (to, from) = (cpu.arg(0), cpu.arg(1)); let text = cpu.string(from); cpu.write_string(to, &text); to }
        "lstrcatA" 2 |cpu| {
            let (to, from) = (cpu.arg(0), cpu.arg(1));
            let text = cpu.string(from);
            let end = to + cpu.string(to).len() as u32;
            cpu.write_string(end, &text);
            to
        }

        "CreateFileA" 7 |cpu| { let name = cpu.string(cpu.arg(0)); let (access, disposition) = (cpu.arg(1), cpu.arg(4)); create_file(cpu, name, access, disposition) }
        "CreateFileW" 7 |cpu| { let name = cpu.wide_string(cpu.arg(0)); let (access, disposition) = (cpu.arg(1), cpu.arg(4)); create_file(cpu, name, access, disposition) }
        "CloseHandle" 1 |cpu| { let handle = cpu.arg(0); cpu.host.files.remove(&handle); 1 }
        "ReadFile" 5 |cpu| {
            let (handle, at, size, done) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            let count = read_file(cpu, handle, at, size);
            if done != 0 { cpu.w32(done, count); }
            1
        }
        "WriteFile" 5 |cpu| {
            let (handle, at, size, done) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            let data = cpu.bytes(at, size).to_vec();
            if let Some(open) = cpu.host.files.get_mut(&handle) {
                let _ = open.file.write_all(&data);
            } else {
                print!("{}", String::from_utf8_lossy(&data));
            }
            if done != 0 { cpu.w32(done, size); }
            1
        }
        "SetFilePointer" 4 |cpu| {
            let (handle, distance, method) = (cpu.arg(0), cpu.arg(1) as i32 as i64, cpu.arg(3));
            let target = match method {
                0 => SeekFrom::Start(distance as u64),
                1 => SeekFrom::Current(distance),
                _ => SeekFrom::End(distance),
            };
            cpu.host.files.get_mut(&handle).and_then(|open| open.file.seek(target).ok()).map_or(INVALID, |at| at as u32)
        }
        "GetFileSize" 2 |cpu| {
            let (handle, high) = (cpu.arg(0), cpu.arg(1));
            if high != 0 { cpu.w32(high, 0); }
            cpu.host.files.get(&handle).map_or(INVALID, |open| open.file.len() as u32)
        }
        "GetFileInformationByHandle" 2 |cpu| {
            let (handle, at) = (cpu.arg(0), cpu.arg(1));
            let size = cpu.host.files.get(&handle).map_or(0, |open| open.file.len());
            cpu.fill(at, 52, 0);
            cpu.w32(at, 0x20);
            cpu.w32(at + 32, (size >> 32) as u32);
            cpu.w32(at + 36, size as u32);
            1
        }
        "FlushFileBuffers" 1 |_cpu| { 1 }
        "SetEndOfFile" 1 |cpu| {
            let handle = cpu.arg(0);
            if let Some(open) = cpu.host.files.get_mut(&handle) {
                if let Ok(at) = open.file.stream_position() {
                    open.file.set_len(at);
                }
            }
            1
        }
        "GetFileAttributesA" 1 |cpu| {
            let name = cpu.string(cpu.arg(0));
            match std::fs::metadata(host_path(cpu, &name)) {
                Ok(meta) => if meta.is_dir() { 0x10 } else { 0x20 },
                Err(_) => { cpu.host.last_error = ERROR_FILE_NOT_FOUND; INVALID }
            }
        }
        "SetFileAttributesA" 2 |_cpu| { 1 }
        "DeleteFileA" 1 |cpu| { let name = cpu.string(cpu.arg(0)); u32::from(std::fs::remove_file(host_path(cpu, &name)).is_ok()) }
        "CreateDirectoryA" 2 |cpu| { let name = cpu.string(cpu.arg(0)); u32::from(std::fs::create_dir(host_path(cpu, &name)).is_ok()) }
        "RemoveDirectoryA" 1 |cpu| { let name = cpu.string(cpu.arg(0)); u32::from(std::fs::remove_dir(host_path(cpu, &name)).is_ok()) }
        "MoveFileA" 2 |cpu| {
            let (from, to) = (cpu.string(cpu.arg(0)), cpu.string(cpu.arg(1)));
            u32::from(std::fs::rename(host_path(cpu, &from), host_path(cpu, &to)).is_ok())
        }
        "CopyFileA" 3 |cpu| {
            let (from, to) = (cpu.string(cpu.arg(0)), cpu.string(cpu.arg(1)));
            u32::from(std::fs::copy(host_path(cpu, &from), host_path(cpu, &to)).is_ok())
        }
        "FindFirstFileA" 2 |cpu| { let pattern = cpu.string(cpu.arg(0)); let data = cpu.arg(1); find_first(cpu, pattern, data) }
        "FindNextFileA" 2 |cpu| { let (handle, data) = (cpu.arg(0), cpu.arg(1)); find_next(cpu, handle, data) }
        "FindClose" 1 |cpu| { let handle = cpu.arg(0); cpu.host.searches.remove(&handle); 1 }
        "OpenFile" 3 |cpu| {
            const OF_EXIST: u32 = 0x4000;
            let (name, style) = (cpu.string(cpu.arg(0)), cpu.arg(2));
            let handle = create_file(cpu, name, 0, 3);
            if style & OF_EXIST != 0 && handle != INVALID {
                cpu.host.files.remove(&handle);
                1
            } else {
                handle
            }
        }
        "GetCurrentDirectoryA" 2 |cpu| { let at = cpu.arg(1); cpu.write_string(at, GAME_DIR); GAME_DIR.len() as u32 }
        "SetCurrentDirectoryA" 1 |_cpu| { 1 }
        "GetFullPathNameA" 4 |cpu| {
            let (name, size, at, last) = (cpu.string(cpu.arg(0)), cpu.arg(1), cpu.arg(2), cpu.arg(3));
            let full = if name.contains(':') { name } else { format!("{GAME_DIR}\\{name}") };
            if (full.len() as u32) < size { cpu.write_string(at, &full); }
            if last != 0 { cpu.w32(last, at + full.rfind('\\').map_or(0, |p| p as u32 + 1)); }
            full.len() as u32
        }
        // The 16-bit style file calls, which old code kept using on handles from OpenFile.
        "_lclose" 1 |cpu| { let handle = cpu.arg(0); cpu.host.files.remove(&handle); 0 }
        "_llseek" 3 |cpu| {
            let (handle, distance, method) = (cpu.arg(0), cpu.arg(1) as i32 as i64, cpu.arg(2));
            let target = match method {
                0 => SeekFrom::Start(distance as u64),
                1 => SeekFrom::Current(distance),
                _ => SeekFrom::End(distance),
            };
            cpu.host.files.get_mut(&handle).and_then(|open| open.file.seek(target).ok()).map_or(INVALID, |at| at as u32)
        }
        "_lread" 3 |cpu| { let (handle, at, size) = (cpu.arg(0), cpu.arg(1), cpu.arg(2)); read_file(cpu, handle, at, size) }
        "_hread" 3 |cpu| { let (handle, at, size) = (cpu.arg(0), cpu.arg(1), cpu.arg(2)); read_file(cpu, handle, at, size) }

        // Global memory is never moved, so a handle is its address.
        "GlobalLock" 1 |cpu| { cpu.arg(0) }
        "GlobalUnlock" 1 |_cpu| { 1 }
        "GlobalHandle" 1 |cpu| { cpu.arg(0) }
        "GlobalReAlloc" 3 |cpu| {
            let (old, size) = (cpu.arg(0), cpu.arg(1));
            let block = cpu.allocate(size);
            cpu.fill(block, size, 0);
            if old != 0 {
                let kept = cpu.block_size(old).min(size);
                let data = cpu.bytes(old, kept).to_vec();
                cpu.bytes_mut(block, kept).copy_from_slice(&data);
                cpu.release(old);
            }
            block
        }

        "InterlockedIncrement" 1 |cpu| { let at = cpu.arg(0); let value = cpu.r32(at).wrapping_add(1); cpu.w32(at, value); value }
        "InterlockedDecrement" 1 |cpu| { let at = cpu.arg(0); let value = cpu.r32(at).wrapping_sub(1); cpu.w32(at, value); value }
        "OutputDebugStringA" 1 |cpu| { if cpu.trace { println!("  debug: {}", cpu.string(cpu.arg(0)).trim_end()); } 0 }
        "GetSystemDefaultLangID" 0 |_cpu| { 0x409 }
        "CompareStringA" 6 |cpu| {
            const IGNORE_CASE: u32 = 1;
            let (flags, first, first_count, second, second_count) = (cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
            let text = |cpu: &Cpu, at: u32, count: u32| {
                let text = if count == INVALID { cpu.string(at) } else { cpu.bytes(at, count).iter().map(|&b| b as char).collect() };
                if flags & IGNORE_CASE != 0 { text.to_lowercase() } else { text }
            };
            // 1 is less, 2 is equal and 3 is greater.
            text(cpu, first, first_count).cmp(&text(cpu, second, second_count)) as i32 as u32 + 2
        }

        // Drive C: is the hard disk, and D: is the game's CD when it has one.
        "GetLogicalDrives" 0 |cpu| { if cpu.host.cd.is_empty() { 4 } else { 12 } }
        "GetDriveTypeA" 1 |cpu| {
            const NO_ROOT: u32 = 1;
            const FIXED: u32 = 3;
            const CDROM: u32 = 5;
            match cpu.string(cpu.arg(0)).to_lowercase().chars().next() {
                Some('c') => FIXED,
                Some('d') if !cpu.host.cd.is_empty() => CDROM,
                _ => NO_ROOT,
            }
        }
        "GetVolumeInformationA" 8 |cpu| {
            let (root, name, name_size, serial, length, flags, system, system_size) =
                (cpu.string(cpu.arg(0)).to_lowercase(), cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5), cpu.arg(6), cpu.arg(7));
            let (label, kind) = match root.chars().next() {
                Some('c') => ("", "FAT"),
                Some('d') if !cpu.host.cd.is_empty() => (cpu.host.cd, "CDFS"),
                _ => return 0,
            };
            if name != 0 && label.len() < name_size as usize { cpu.write_string(name, label); }
            if system != 0 && kind.len() < system_size as usize { cpu.write_string(system, kind); }
            if serial != 0 { cpu.w32(serial, 0x1234_5678); }
            if length != 0 { cpu.w32(length, 255); }
            if flags != 0 { cpu.w32(flags, 0); }
            1
        }
        "GetDiskFreeSpaceA" 5 |cpu| {
            let values = [cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4)];
            for (at, value) in values.into_iter().zip([8, 512, 1 << 18, 1 << 19]) {
                cpu.w32(at, value);
            }
            1
        }
    }
}
