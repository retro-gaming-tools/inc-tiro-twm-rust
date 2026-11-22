//! Crashes a real child process as the launcher could, and reads what it left
//! in recomp.log (user, 2026-11-22).

use std::process::Command;

const KIND: &str = "RECOMP_TEST_CRASH";

/// Not a test of its own: the child that crashes, when a test below asks.
#[test]
fn crashing_child() {
    let Ok(kind) = std::env::var(KIND) else { return };
    super::report_crashes("Test");
    crate::runlog::start("Test", "1.2", || None);
    crate::host::launcher::note("picked /some/disc.cue");
    if kind == "panic" {
        panic!("the launcher broke");
    }
    std::process::abort();
}

fn log_after(kind: &str) -> String {
    let folder = std::env::temp_dir().join(format!("recomp-crash-test-{}-{kind}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "loader::crash_tests::crashing_child", "--test-threads=1"])
        .current_dir(&folder)
        .env(KIND, kind)
        .env("RECOMP_HEADLESS", "1")
        // Where an AppImage is, which is where its log goes.
        .env("APPIMAGE", folder.join("game.AppImage"))
        .output()
        .unwrap()
        .status;
    assert!(!status.success());
    let log = std::fs::read_to_string(folder.join("recomp.log")).unwrap();
    let _ = std::fs::remove_dir_all(&folder);
    log
}

#[test]
fn a_launcher_panic_is_logged_with_its_steps() {
    let log = log_after("panic");
    for wanted in ["Test 1.2, on ", "launcher: picked /some/disc.cue", "the launcher broke", "crash_tests.rs", "backtrace:"] {
        assert!(log.contains(wanted), "{wanted} isn't in:\n{log}");
    }
}

#[test]
fn a_crash_without_a_panic_still_leaves_the_steps() {
    let log = log_after("abort");
    assert!(log.contains("launcher: picked /some/disc.cue"), "{log}");
}
