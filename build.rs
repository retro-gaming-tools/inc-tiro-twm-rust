use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=icon.rc");
    println!("cargo:rerun-if-changed=icon.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }
    let object = format!("{}/icon.o", std::env::var("OUT_DIR").unwrap());
    let status = Command::new("x86_64-w64-mingw32-windres")
        .args(["icon.rc", "-O", "coff", "-o", &object])
        .status()
        .expect("windres isn't available");
    assert!(status.success(), "windres failed");
    println!("cargo:rustc-link-arg={object}");
}
