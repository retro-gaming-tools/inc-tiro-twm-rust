//! Static recompiler for 32-bit Windows games.
//!
//! `recomp <game> <file>=<sha256>...` checks that each file is the exact build
//! this recompilation was made for, finds its code, translates it to Rust, and
//! writes a crate to /generated, with the runtime beside it.

mod discover;
mod lift;
#[allow(dead_code)]
#[path = "../runtime/src/pe.rs"]
mod pe;

use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write;
use std::path::Path;
use std::process::ExitCode;

const FUNCTIONS_PER_FILE: usize = 300;
const FIRST_DLL_BASE: u32 = 0x1000_0000;
const DLL_SPACING: u32 = 0x0100_0000;

/// What's wrong with one input file, in words for the player.
fn check_file(path: &str, expected: &str) -> Result<Vec<u8>, String> {
    let file = std::fs::read(path).map_err(|_| "is missing".to_string())?;
    let found: String = Sha256::digest(&file).iter().map(|byte| format!("{byte:02x}")).collect();
    if found == expected.to_lowercase() {
        Ok(file)
    } else {
        Err(format!("is a different build ({} bytes, SHA-256 {found})", file.len()))
    }
}

fn copy_folder(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_folder(&entry.path(), &target)?;
        } else {
            write_if_changed(&target, &std::fs::read_to_string(entry.path())?)?;
        }
    }
    Ok(())
}

struct Module {
    file: String,
    base: u32,
    sha256: String,
    functions: Vec<u32>,
}

/// Embeds the icon in the Windows executable.
const BUILD_SCRIPT: &str = r#"use std::process::Command;

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
"#;

/// What the player is told about the game.
struct Info {
    title: String,
    needs: String,
    /// The command line the game sees, which is how some games pick a renderer.
    command: String,
}

/// Leaves unchanged files alone, so cargo doesn't rebuild them.
fn write_if_changed(path: &Path, text: &str) -> std::io::Result<()> {
    if std::fs::read_to_string(path).is_ok_and(|old| old == text) {
        return Ok(());
    }
    std::fs::write(path, text)
}

fn write_crate(game: &str, info: &Info, modules: &[Module], chunks: &[String], mods: &[Mod]) -> std::io::Result<()> {
    let root = Path::new("/generated");
    let source = root.join("src");
    std::fs::create_dir_all(source.join("code"))?;
    std::fs::create_dir_all(root.join("mods"))?;
    write_if_changed(&source.join("mods.rs"), &mods_module(mods))?;
    copy_folder(Path::new("/recomp/runtime"), &root.join("runtime"))?;
    for name in ["icon.ico", "icon.rgba", "icon.png"] {
        std::fs::copy(Path::new("/port").join(name), root.join(name))?;
    }
    write_if_changed(&root.join("icon.rc"), "1 ICON \"icon.ico\"\n")?;
    write_if_changed(&root.join("build.rs"), BUILD_SCRIPT)?;

    let version = std::env::var("RECOMP_VERSION").ok().filter(|v| !v.trim().is_empty()).unwrap_or("0.1".into());
    let version = version.trim();
    write_if_changed(&root.join("VERSION"), &format!("{version}\n"))?;
    let semver = cargo_version(version);
    let cargo = format!(
        "[package]\nname = \"{game}-rust\"\nversion = \"{semver}\"\nedition = \"2021\"\n\n[dependencies]\nrecomp-runtime = {{ path = \"runtime\" }}\n\n[profile.release]\nopt-level = 2\ncodegen-units = 256\ndebug = false\npanic = \"unwind\"\n\n[profile.release.package.recomp-runtime]\nopt-level = 3\n"
    );
    write_if_changed(&root.join("Cargo.toml"), &cargo)?;
    write_if_changed(&root.join(".gitignore"), &format!("target/\ndist/\n{game}-rust\ngame\n"))?;
    write_if_changed(&root.join("README.md"), &readme(game, info))?;

    let mut code = String::from("#![allow(warnings)]\n\npub use crate::mods::*;\n");
    for index in 0..chunks.len() {
        let _ = writeln!(code, "mod chunk_{index:03};\npub use chunk_{index:03}::*;");
    }
    write_if_changed(&source.join("code/mod.rs"), &code)?;
    for (index, chunk) in chunks.iter().enumerate() {
        write_if_changed(&source.join(format!("code/chunk_{index:03}.rs")), chunk)?;
    }

    let mut main = String::from("#![windows_subsystem = \"windows\"]\n\nmod code;\nmod mods;\n\nuse recomp_runtime::{loader, GuestFn};\n\nstatic MODULES: &[loader::Module] = &[\n");
    for module in modules {
        let _ = writeln!(main, "    loader::Module {{ file: \"{}\", base: {:#x}, sha256: \"{}\" }},", module.file, module.base, module.sha256);
    }
    main.push_str("];\n\nstatic TABLE: &[(u32, GuestFn)] = &[\n");
    for function in modules.iter().flat_map(|m| &m.functions) {
        let _ = writeln!(main, "    ({function:#x}, code::f_{function:08x}),");
    }
    let _ = write!(
        main,
        "];\n\nstatic GAME: loader::Game = loader::Game {{\n    title: \"{}\",\n    version: include_str!(\"../VERSION\").trim_ascii(),\n    needs: \"{}\",\n    command: \"{}\",\n    switches: &[{}],\n    renderers: &[{}],\n    languages: &[{}],\n    mods: &[{}],\n    modules: MODULES,\n    table: TABLE,\n    icon: include_bytes!(\"../icon.rgba\"),\n    music: &[{}],\n    registry: &[{}],\n    cd: {:?},\n}};\n\nfn main() {{\n    loader::run(&GAME);\n}}\n",
        info.title, info.needs, info.command, switches(), choices("RECOMP_RENDERERS"), choices("RECOMP_LANGUAGES"), launcher_mods(mods), music(),
        registry(), std::env::var("RECOMP_CD").unwrap_or_default()
    );
    write_if_changed(&source.join("main.rs"), &main)
}

/// Cargo wants three numbers, so `0.5-beta` becomes `0.5.0-beta`.
fn cargo_version(version: &str) -> String {
    let (numbers, label) = version.split_once('-').map_or((version, None), |(n, l)| (n, Some(l)));
    let mut parts: Vec<&str> = numbers.split('.').collect();
    parts.resize(3, "0");
    let numbers = parts[..3].join(".");
    label.map_or(numbers.clone(), |label| format!("{numbers}-{label}"))
}

/// The generated repo's README. The repo is public: it holds the translated
/// code, the runtime, the recompiler and the build, so anyone can build the
/// release themselves or regenerate the code from their own copy of the game.
fn readme(game: &str, info: &Info) -> String {
    let title = &info.title;
    let needs = &info.needs;
    // Hand-written sections about this game, such as its launcher's rows.
    let sections = std::fs::read_to_string("/generated/release/readme-sections.md").map(|text| format!("{}\n\n", text.trim_end())).unwrap_or_default();
    // `recompRepo` from `port.json`, such as `owner/name` on GitHub.
    let repo = std::env::var("RECOMP_REPO").ok().filter(|r| !r.trim().is_empty()).unwrap_or("<owner>/<repo>".into());
    let repo = repo.trim();
    format!(r#"# {title}, recompiled

A native Windows and Linux version of {title}, made by translating the game's original Windows program statically into Rust, on a new runtime built on SDL3. The version is in `VERSION`, and releases are at https://github.com/{repo}/releases.

The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**No game data is included.** You need {needs}. The program reads the game's files from your copy when it starts, and checks every program file by SHA-256 before it runs.

{sections}## Why the source is here

So you don't have to trust a download. Everything that makes the release is in this repository:

- `src/`: the game's code, translated to Rust by the recompiler. Don't edit it by hand: it's regenerated.
- `mods/`: hand-written Rust that replaces some of the game's functions (see below).
- `runtime/`: the hand-written runtime: the CPU state, the Windows functions the game calls, video, sound and input on SDL3, and the launcher.
- `recompiler/`: the static recompiler that wrote `src/`.
- `build/`, `build.sh`, `regenerate.sh`: the Docker build for the release, and the check that the code matches your game.
- `.github/workflows/build.yml`: GitHub builds the release from a tagged commit, with build provenance.
- `CHANGELOG.md`: what changed in each version, and `changes.sh`, which shows the full diff between two versions.

## Building

You need Docker, and nothing else.

    ./build.sh

This builds an image with pinned tools (Rust, MinGW, SDL3, appimagetool), then the Windows zip and the Linux AppImage, into `dist/`.

## Checking the code against your game

    ./regenerate.sh /path/to/your/game

This builds the recompiler, runs it on your copy of the game with the settings in `recomp.json`, and compares the result with `src/` and the rest of this repository. No difference means the published code is exactly what the recompiler makes from the game.

## Checking a release

Release files built by GitHub carry a provenance attestation. With the GitHub CLI:

    gh attestation verify <file> --repo {repo}

It tells you which workflow run and which commit built the file.

## Comparing versions

Each version has a tag `v<version>`, such as `v0.5-beta`.

    ./changes.sh 0.4-beta 0.5-beta > changes.diff

This lists the changelog entries, the commits and the files that changed, then the diff of everything written by hand: the runtime, the mods, the recompiler and the build. The translated code in `src/code/` is only summarised, because a regeneration rewrites much of it. Add `--code` for its full diff.

## Mods

Each `.rs` file in `mods/` is hand-written Rust, and a `pub fn f_XXXXXXXX(cpu: &mut Cpu)` in one replaces the game's function at that address everywhere. The translated original stays callable as `o_XXXXXXXX`. Start a mod file with `use crate::code::*;` and `use recomp_runtime::Cpu;`. A line `//! launcher: LABEL` gives the mod an On/Off row in the launcher (`launcher-off:` for one that starts off, `launcher-advanced:` for one on the advanced page), and its functions then begin with `if !host::mod_enabled(cpu, "FILE") {{ return o_XXXXXXXX(cpu); }}`.

## Licence

The recompiler, the runtime, the mods and the build scripts are under the MIT licence in `LICENSE`.

The code in `src/code/` is translated from the program files of the original game, including the Microsoft C runtime it comes with. That code belongs to its rights holders, so no licence is granted for it here. This project isn't affiliated with or endorsed by them. Please buy the game.

Generated for `{game}` by the recompiler in `recompiler/`.
"#)
}

/// `recompSwitches` from `port.json`, as Rust: each is a command-line switch, the
/// guest address the runtime sets to 1 when the player gives it, and the label
/// the launcher shows, which follows the address.
fn switches() -> String {
    let text = std::env::var("RECOMP_SWITCHES").unwrap_or_default();
    let pairs = text.split(';').filter_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        let (address, label) = value.trim().split_once(' ').unwrap_or((value.trim(), name.trim()));
        let address = u32::from_str_radix(address, 16).ok()?;
        Some(format!("(\"{}\", {address:#x}, \"{}\")", name.trim(), label.trim().replace('"', "'")))
    });
    pairs.collect::<Vec<_>>().join(", ")
}

/// `recompRenderers` or `recompLanguages` from `port.json`, as Rust: each entry is the game's
/// switch that picks a renderer or language, then the label the launcher shows, such as
/// `-3dfx Glide`. The first is the default. `RECOMP_RENDERERS` and `RECOMP_LANGUAGES` hold
/// them separated by semicolons.
fn choices(variable: &str) -> String {
    let text = std::env::var(variable).unwrap_or_default();
    let entries = text.split(';').filter(|entry| !entry.trim().is_empty()).map(|entry| {
        let (switch, label) = entry.trim().split_once(' ').unwrap_or((entry.trim(), entry.trim()));
        format!("(\"{}\", \"{}\")", switch.replace('"', ""), label.trim().replace('"', "'"))
    });
    entries.collect::<Vec<_>>().join(", ")
}

/// `recompRegistry` from `port.json`, as Rust: each entry is `KEY\NAME=TEXT`, a
/// registry string the game finds as if its installer had written it, such as
/// the folder it was installed to. `RECOMP_REGISTRY` holds them separated by
/// semicolons.
fn registry() -> String {
    let text = std::env::var("RECOMP_REGISTRY").unwrap_or_default();
    let entries = text.split(';').filter_map(|entry| {
        let (name, value) = entry.split_once('=')?;
        Some(format!("({:?}, {:?})", name.trim(), value))
    });
    entries.collect::<Vec<_>>().join(", ")
}

/// `recompMusic` from `port.json`, as Rust: each entry is a file prefix for CD
/// tracks, such as `music/Track`, optionally followed by `ADDRESS=VALUE`, the
/// guest value that must hold for it to be used. `RECOMP_MUSIC` holds them
/// separated by semicolons.
fn music() -> String {
    let text = std::env::var("RECOMP_MUSIC").unwrap_or_default();
    let entries = text.split(';').filter(|entry| !entry.trim().is_empty()).map(|entry| {
        let mut words = entry.split_whitespace();
        let prefix = words.next().unwrap_or_default();
        let (address, value) = words
            .next()
            .and_then(|condition| condition.split_once('='))
            .and_then(|(address, value)| Some((u32::from_str_radix(address, 16).ok()?, value.parse::<u32>().ok()?)))
            .unwrap_or((0, 0));
        format!("(\"{}\", {address:#x}, {value})", prefix.replace('"', ""))
    });
    entries.collect::<Vec<_>>().join(", ")
}

/// Hand-written Rust in the generated repo's `mods/` folder, which the
/// recompiler never writes. Each `.rs` file there becomes a module, and every
/// `pub fn f_XXXXXXXX` in one replaces the game's function at that address:
/// direct calls, jumps and the table for indirect calls all reach it, and the
/// translated original stays callable as `o_XXXXXXXX`. Returns the files and the
/// replaced addresses.
///
/// A file with a line `//! launcher: LABEL` gets an On/Off row in the launcher,
/// on by default, or off by default with `//! launcher-off: LABEL`. With
/// `//! launcher-advanced: LABEL` the row is on by default and sits on the
/// launcher's advanced page, for what most players should leave alone. Its
/// functions then check `host::mod_enabled(cpu, "FILE")` and hand over to the
/// original when the player turned it off. A file without one is always on.
fn find_mods(folder: &Path) -> Result<(Vec<Mod>, BTreeSet<u32>), String> {
    let mut files = Vec::new();
    let mut overrides = BTreeSet::new();
    let Ok(entries) = std::fs::read_dir(folder) else { return Ok((files, overrides)) };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(stem) = name.strip_suffix(".rs") else { continue };
        if stem == "mod" || !stem.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(format!("mods/{name}: name a mod file in lower case letters, digits and _, not mod.rs"));
        }
        let text = std::fs::read_to_string(entry.path()).map_err(|error| format!("mods/{name}: {error}"))?;
        let mut launcher = None;
        for line in text.lines() {
            if let Some(label) = line.trim().strip_prefix("//! launcher:") {
                launcher = Some((label.trim().replace('"', "'"), true, false));
            } else if let Some(label) = line.trim().strip_prefix("//! launcher-off:") {
                launcher = Some((label.trim().replace('"', "'"), false, false));
            } else if let Some(label) = line.trim().strip_prefix("//! launcher-advanced:") {
                launcher = Some((label.trim().replace('"', "'"), true, true));
            }
            let Some(rest) = line.trim_start().strip_prefix("pub fn f_") else { continue };
            let Some(address) = rest.get(..8).and_then(|hex| u32::from_str_radix(hex, 16).ok()) else { continue };
            if !overrides.insert(address) {
                return Err(format!("mods/{name}: f_{address:08x} is replaced twice"));
            }
        }
        files.push(Mod { file: stem.to_string(), launcher });
    }
    files.sort_by(|a, b| a.file.cmp(&b.file));
    Ok((files, overrides))
}

/// A file in `mods/`, and its launcher row, if it has one: the label, the
/// default, and whether it's on the advanced page.
struct Mod {
    file: String,
    launcher: Option<(String, bool, bool)>,
}

/// The mods the launcher offers, as Rust: file name, label, whether it starts
/// on, and whether its row is on the advanced page.
fn launcher_mods(mods: &[Mod]) -> String {
    let rows = mods.iter().filter_map(|m| m.launcher.as_ref().map(|(label, on, advanced)| format!("(\"{}\", \"{label}\", {on}, {advanced})", m.file)));
    rows.collect::<Vec<_>>().join(", ")
}

/// `src/mods.rs`, which pulls in each mod file.
fn mods_module(mods: &[Mod]) -> String {
    let mut text = String::from("//! Written by the recompiler: the hand-written files in `mods/`.\n#![allow(warnings)]\n\n");
    for file in mods.iter().map(|m| &m.file) {
        let _ = writeln!(text, "#[path = \"../mods/{file}.rs\"]\npub mod {file};\npub use {file}::*;");
    }
    text
}

fn hex_bytes(text: &str) -> Option<Vec<u8>> {
    (0..text.len()).step_by(2).map(|at| u8::from_str_radix(text.get(at..at + 2)?, 16).ok()).collect()
}

/// Fixes a bug in the code that's translated, from `recompPatches` in `port.json`.
/// Each is "file address old-bytes new-bytes", and `RECOMP_PATCHES` holds them
/// separated by semicolons. The game's own files aren't changed.
fn apply_patches(name: &str, image: &mut pe::Image) -> Result<(), String> {
    let patches = std::env::var("RECOMP_PATCHES").unwrap_or_default();
    for patch in patches.split(';').filter(|patch| !patch.trim().is_empty()) {
        let parts: Vec<&str> = patch.split_whitespace().collect();
        let [file, address, old, new] = parts[..] else { return Err(format!("can't read the patch {patch}")) };
        if !file.eq_ignore_ascii_case(name) {
            continue;
        }
        let at = u32::from_str_radix(address, 16).map_err(|_| format!("bad address in {patch}"))?;
        let (old, new) = hex_bytes(old).zip(hex_bytes(new)).ok_or(format!("bad bytes in {patch}"))?;
        if old.len() != new.len() {
            return Err(format!("the patch at {address} changes the code's length"));
        }
        let start = (at - image.base) as usize;
        if image.memory.get(start..start + old.len()) != Some(&old[..]) {
            return Err(format!("the code at {address} isn't what the patch expects"));
        }
        image.memory[start..start + new.len()].copy_from_slice(&new);
    }
    Ok(())
}

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    let game = arguments.next().unwrap_or_default();
    let info = Info { title: arguments.next().unwrap_or_default(), needs: arguments.next().unwrap_or_default(), command: arguments.next().unwrap_or_default() };
    let mut files = Vec::new();
    let mut problems = Vec::new();
    for argument in arguments {
        let (path, expected) = argument.rsplit_once('=').unwrap_or((&argument, ""));
        let name = path.strip_prefix("/game/").unwrap_or(path).to_string();
        match check_file(path, expected) {
            Ok(file) => files.push((name, expected.to_string(), file)),
            Err(problem) => problems.push(format!("  {name} {problem}")),
        }
    }
    if !problems.is_empty() {
        eprintln!("Can't recompile {game}, because these files don't match the build it was made for:");
        eprintln!("{}", problems.join("\n"));
        eprintln!("\nUse the unmodified files from the release named in the port's README.");
        return ExitCode::FAILURE;
    }

    let (mods, overrides) = match find_mods(Path::new("/generated/mods")) {
        Ok(found) => found,
        Err(problem) => {
            eprintln!("{problem}");
            return ExitCode::FAILURE;
        }
    };
    let mut stats = lift::Stats::default();
    let mut modules = Vec::new();
    let mut chunks = Vec::new();
    for (index, (name, sha256, file)) in files.iter().enumerate() {
        let Some(mut image) = pe::Image::parse(file) else {
            eprintln!("{name}: not a 32-bit PE file");
            return ExitCode::FAILURE;
        };
        // The main program stays where it is, because it has no relocations.
        let base = if image.is_dll { FIRST_DLL_BASE + index as u32 * DLL_SPACING } else { image.base };
        image.rebase(base);
        if let Err(problem) = apply_patches(name, &mut image) {
            eprintln!("{name}: {problem}");
            return ExitCode::FAILURE;
        }
        let report = discover::discover(&image);
        print_report(name, &image, &report);
        let all: HashSet<u32> = report.functions.iter().copied().collect();
        let functions: Vec<u32> = report.functions.iter().chain(&report.entries).copied().collect();
        for group in functions.chunks(FUNCTIONS_PER_FILE) {
            let mut text = String::from("#![allow(warnings)]\nuse recomp_runtime::Cpu;\nuse super::*;\n\n");
            for &function in group {
                text.push_str(&lift::lift_function(function, &report, &all, overrides.contains(&function), &mut stats));
            }
            chunks.push(text);
        }
        modules.push(Module { file: name.clone(), base, sha256: sha256.clone(), functions });
    }
    let known: HashSet<u32> = modules.iter().flat_map(|m| m.functions.iter().copied()).collect();
    let unknown: Vec<String> = overrides.iter().filter(|a| !known.contains(a)).map(|a| format!("f_{a:08x}")).collect();
    if !unknown.is_empty() {
        eprintln!("mods/ replaces {}, which no file has as a function", unknown.join(", "));
        return ExitCode::FAILURE;
    }
    if !overrides.is_empty() {
        println!("== mods replace {} functions", overrides.len());
    }

    let missing: Vec<String> = stats.missing.iter().map(|(name, count)| format!("{name}: {count}")).collect();
    let _ = std::fs::write("/out/untranslated.txt", missing.join("\n"));
    if let Err(error) = write_crate(&game, &info, &modules, &chunks, &mods) {
        eprintln!("Couldn't write the generated crate: {error}");
        return ExitCode::FAILURE;
    }
    println!("== wrote {} source files", chunks.len());
    ExitCode::SUCCESS
}

fn print_report(name: &str, image: &pe::Image, report: &discover::Report) {
    let covered: usize = report.instructions.values().map(|i| i.len()).sum();
    let percent = covered as f64 * 100.0 / image.code_bytes() as f64;
    println!(
        "{name}: {percent:.1}% of code, {} instructions, {} functions, {} jump tables, {} unresolved jumps, {} false starts",
        report.instructions.len(),
        report.functions.len(),
        report.jump_tables.len(),
        report.unresolved_jumps.len(),
        report.invalid.len()
    );
}

