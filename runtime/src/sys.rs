//! The parts of the runtime that differ between Linux and Windows.

use std::ffi::{c_void, CString};

pub type Pointer = *mut c_void;

/// Guest addresses are 32 bits, so all of them are set aside.
const MEMORY_SIZE: usize = 1 << 32;

#[cfg(unix)]
mod os {
    use super::*;

    const LIBRARIES: [&str; 3] = ["./libSDL3.so.0", "libSDL3.so.0", "libSDL3.so"];

    pub fn open_sdl() -> Pointer {
        for name in LIBRARIES {
            let text = CString::new(name).unwrap();
            let library = unsafe { libc::dlopen(text.as_ptr(), libc::RTLD_NOW) };
            if !library.is_null() {
                return library;
            }
        }
        std::ptr::null_mut()
    }

    pub fn symbol(library: Pointer, name: &str) -> Pointer {
        let text = CString::new(name).unwrap();
        unsafe { libc::dlsym(library, text.as_ptr()) }
    }

    pub fn reserve_memory() -> *mut u8 {
        let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE;
        let memory = unsafe { libc::mmap(std::ptr::null_mut(), MEMORY_SIZE, libc::PROT_READ | libc::PROT_WRITE, flags, -1, 0) };
        assert!(memory != libc::MAP_FAILED, "couldn't reserve the game's memory");
        memory as *mut u8
    }
}

#[cfg(windows)]
mod os {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const MEM_COMMIT: u32 = 0x1000;
    const MEM_RESERVE: u32 = 0x2000;
    const PAGE_NOACCESS: u32 = 1;
    const PAGE_READWRITE: u32 = 4;
    const ACCESS_VIOLATION: u32 = 0xc000_0005;
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    const FIRST_HANDLER: u32 = 1;
    const COMMIT_GRANULARITY: usize = 0x1_0000;
    /// Where EXCEPTION_RECORD keeps the address that faulted.
    const FAULT_ADDRESS_OFFSET: usize = 40;

    static BASE: AtomicUsize = AtomicUsize::new(0);

    extern "system" {
        fn LoadLibraryA(name: *const std::ffi::c_char) -> Pointer;
        fn GetProcAddress(module: Pointer, name: *const std::ffi::c_char) -> Pointer;
        fn VirtualAlloc(at: Pointer, size: usize, kind: u32, protect: u32) -> Pointer;
        fn AddVectoredExceptionHandler(first: u32, handler: unsafe extern "system" fn(*mut *const u8) -> i32) -> Pointer;
    }

    /// Commits guest memory the first time it's touched, which is what Linux
    /// does for `MAP_NORESERVE`. Committing 4 GB at once would often fail.
    unsafe extern "system" fn commit_on_fault(pointers: *mut *const u8) -> i32 {
        let record = *pointers;
        if *(record as *const u32) != ACCESS_VIOLATION {
            return CONTINUE_SEARCH;
        }
        let address = *(record.add(FAULT_ADDRESS_OFFSET) as *const usize);
        let base = BASE.load(Ordering::Relaxed);
        if address < base || address - base >= MEMORY_SIZE {
            return CONTINUE_SEARCH;
        }
        let page = base + ((address - base) & !(COMMIT_GRANULARITY - 1));
        if VirtualAlloc(page as Pointer, COMMIT_GRANULARITY, MEM_COMMIT, PAGE_READWRITE).is_null() {
            return CONTINUE_SEARCH;
        }
        CONTINUE_EXECUTION
    }

    pub fn open_sdl() -> Pointer {
        let text = CString::new("SDL3.dll").unwrap();
        unsafe { LoadLibraryA(text.as_ptr()) }
    }

    pub fn symbol(library: Pointer, name: &str) -> Pointer {
        let text = CString::new(name).unwrap();
        unsafe { GetProcAddress(library, text.as_ptr()) }
    }

    pub fn reserve_memory() -> *mut u8 {
        let memory = unsafe { VirtualAlloc(std::ptr::null_mut(), MEMORY_SIZE, MEM_RESERVE, PAGE_NOACCESS) };
        assert!(!memory.is_null(), "couldn't reserve the game's memory");
        BASE.store(memory as usize, Ordering::Relaxed);
        unsafe { AddVectoredExceptionHandler(FIRST_HANDLER, commit_on_fault) };
        memory as *mut u8
    }
}

pub use os::{open_sdl, reserve_memory, symbol};
