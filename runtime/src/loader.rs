//! Sets up guest memory and starts the game.
//!
//! The game's data sections are read from the player's own files at run time,
//! at the same addresses the recompiler used.

use crate::cpu::{Cpu, GuestFn, Host, IMPORT_BASE};
use crate::{host, pe};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const STACK_TOP: u32 = 0x7000_0000;
const STACK_SIZE: u32 = 0x0040_0000;
const THREAD_BLOCK: u32 = 0x7ffd_0000;
/// Rust frames stand in for the guest's calls, so the host stack has to be deep.
const HOST_STACK: usize = 512 * 1024 * 1024;

/// DLLs Windows would start before the program, dependencies first. The others
/// wait for LoadLibrary.
const STARTED_FIRST: &[&str] = &["msvcrt.dll", "msvcirt.dll", "newtoolsr.dll"];

pub struct Module {
    pub file: &'static str,
    pub base: u32,
    pub sha256: &'static str,
}

pub struct Game {
    pub title: &'static str,
    /// The release, from the generated repo's `VERSION` file, such as `0.5-beta`.
    pub version: &'static str,
    pub needs: &'static str,
    pub command: &'static str,
    /// Command-line switches, the guest address each sets to 1, and its label.
    pub switches: &'static [(&'static str, u32, &'static str)],
    /// The renderers the launcher offers, from `recompRenderers`: the game's own switch that
    /// picks each, and its label. The first is the default.
    pub renderers: &'static [(&'static str, &'static str)],
    /// The languages the launcher offers, from `recompLanguages`, the same way.
    pub languages: &'static [(&'static str, &'static str)],
    /// Mods the player can turn off in the launcher: the file in `mods/`, its
    /// label, whether it starts on, and whether its row is on the launcher's
    /// advanced page. Mods not listed are always on.
    pub mods: &'static [(&'static str, &'static str, bool, bool)],
    pub modules: &'static [Module],
    pub table: &'static [(u32, GuestFn)],
    pub icon: &'static [u8],
    /// CD music files, from `recompMusic`: see `host::State::music`.
    pub music: &'static [(&'static str, u32, u32)],
    /// Registry strings the game expects from its installer, from
    /// `recompRegistry`: the key and name, and the text.
    pub registry: &'static [(&'static str, &'static str)],
    /// The volume label of the game's CD, from `recompCd`. With one, the game
    /// finds a CD drive D: with that label, which is the game folder.
    pub cd: &'static str,
}

/// A program with no console would otherwise end without a word, so a crash
/// leaves a log and tells the player.
fn report_crashes(title: &'static str) {
    std::panic::set_hook(Box::new(move |information| {
        let text = format!("{title} stopped with an error:\n\n{information}");
        let _ = std::fs::write("recomp-crash.log", &text);
        host::video::show_error(title, &text);
    }));
}

/// Without a path, the game folder is `game` here or beside the program, or else
/// this folder or the program's own, for a build copied in with the game's files.
pub(crate) fn default_folder(game: &Game) -> String {
    let program = std::env::current_exe().ok().and_then(|path| Some(path.parent()?.to_path_buf()));
    let mut places = vec![std::path::PathBuf::from("game")];
    places.extend(program.iter().map(|folder| folder.join("game")));
    places.push(std::path::PathBuf::from("."));
    places.extend(program);
    let first_file = game.modules[0].file;
    let found = places.iter().map(|place| place.to_string_lossy().to_string()).find(|place| find_file(place, first_file).is_some());
    found.unwrap_or_else(|| "game".to_string())
}

/// Checks the files the code was recompiled from, before anything starts.
pub(crate) fn check_files(game: &Game, folder: &str) -> Result<(), String> {
    let mut missing = Vec::new();
    let mut different = Vec::new();
    for module in game.modules {
        match find_file(folder, module.file) {
            None => missing.push(module.file),
            Some(path) => {
                let file = std::fs::read(path).unwrap_or_default();
                let found: String = Sha256::digest(&file).iter().map(|byte| format!("{byte:02x}")).collect();
                if found != module.sha256 {
                    different.push(module.file);
                }
            }
        }
    }
    if missing.is_empty() && different.is_empty() {
        return Ok(());
    }
    let absolute = std::fs::canonicalize(folder).map_or(folder.to_string(), |path| path.display().to_string());
    let mut text = format!("{} can't start, because of the files in this folder:\n{absolute}\n", game.title);
    if !missing.is_empty() {
        text.push_str(&format!("\nMissing: {}\n", missing.join(", ")));
    }
    if !different.is_empty() {
        text.push_str(&format!("\nFrom a different version: {}\n", different.join(", ")));
    }
    text.push_str(&format!(
        "\n{} needs {}.\n\nPut this program in the game's folder, or put the game's files in a folder named game beside it, or start it with the folder as its argument.",
        game.title, game.needs
    ));
    Err(text)
}

fn find_file(folder: &str, name: &str) -> Option<std::path::PathBuf> {
    std::fs::read_dir(folder).ok()?.flatten().find(|entry| entry.file_name().to_string_lossy().eq_ignore_ascii_case(name)).map(|entry| entry.path())
}

fn load_images(modules: &[Module], folder: &str) -> Vec<(String, pe::Image)> {
    modules
        .iter()
        .map(|module| {
            let path = find_file(folder, module.file).unwrap_or_else(|| panic!("{} is missing from {folder}", module.file));
            let file = std::fs::read(path).expect("a game file can't be read");
            let mut image = pe::Image::parse(&file).expect("a game file isn't a 32-bit Windows program");
            image.rebase(module.base);
            (module.file.to_lowercase(), image)
        })
        .collect()
}

fn resolve(image: &pe::Image, others: &[(String, pe::Image)], cpu: &mut Cpu, known: &mut HashMap<String, u32>) {
    for import in &image.imports {
        let label = match &import.name {
            Some(name) => format!("{}!{name}", import.dll),
            None => format!("{}!#{}", import.dll, import.ordinal),
        };
        let export = others.iter().find(|(file, _)| *file == import.dll).and_then(|(_, other)| {
            other.exports.iter().find(|e| match &import.name {
                Some(name) => e.name.as_deref() == Some(name),
                None => e.ordinal == import.ordinal,
            })
        });
        let address = match export {
            Some(export) => export.address,
            None => *known.entry(label.clone()).or_insert_with(|| {
                let key = import.name.clone().unwrap_or_else(|| format!("#{}", import.ordinal));
                let function = host::find(&import.dll, &key);
                cpu.imports.push(Host { name: label.clone(), function });
                IMPORT_BASE + cpu.imports.len() as u32 - 1
            }),
        };
        cpu.w32(import.slot, address);
    }
}

fn start(game: &'static Game, folder: String, switches: Vec<bool>, mods: Vec<bool>, command: &'static str) {
    let table = game.table;
    let images = load_images(game.modules, &folder);
    let mut cpu = Cpu {
        eax: 0, ecx: 0, edx: 0, ebx: 0, esp: STACK_TOP - 0x100, ebp: 0, esi: 0, edi: 0,
        cf: false, zf: false, sf: false, of: false, df: false, parity: 1,
        fs_base: THREAD_BLOCK,
        st: [0.0; 8], top: 0, c0: false, c1: false, c2: false, c3: false, fpu_control: 0x27f,
        memory: crate::sys::reserve_memory(),
        functions: table.iter().copied().collect(),
        imports: Vec::new(),
        host: Box::new(host::State::new(&folder)),
        trace: std::env::var_os("RECOMP_TRACE").is_some(),
        trace_filter: std::env::var("RECOMP_TRACE").ok().filter(|value| value != "1"),
        watched: HashMap::new(),
        watch_counts: HashMap::new(),
        profiling: false,
        hits: HashMap::new(),
        error_function: None,
    };
    let errors = cpu.host.script.steps.front().and_then(|step| step.strip_prefix("errors:").map(str::to_string));
    if let Some(address) = errors {
        cpu.host.script.steps.pop_front();
        cpu.error_function = u32::from_str_radix(&address, 16).ok();
        cpu.profiling = true;
    }
    // A script can name a DLL with a `watch:` first step, to log its exports.
    let watched = cpu.host.script.steps.front().and_then(|step| step.strip_prefix("watch:").map(str::to_string));
    if let Some(watched) = watched {
        cpu.host.script.steps.pop_front();
        for (name, image) in &images {
            if *name == watched.to_lowercase() {
                for export in &image.exports {
                    cpu.watched.insert(export.address, export.name.clone().unwrap_or_default());
                }
            }
        }
    }
    for (_, image) in &images {
        unsafe { std::ptr::copy_nonoverlapping(image.memory.as_ptr(), cpu.memory.add(image.base as usize), image.memory.len()) };
    }
    let mut known = HashMap::new();
    for (_, image) in &images {
        resolve(image, &images, &mut cpu, &mut known);
    }
    for ((_, address, _), on) in game.switches.iter().zip(switches) {
        if on {
            cpu.w32(*address, 1);
        }
    }
    cpu.w32(THREAD_BLOCK, 0xffff_ffff); // No exception handlers yet.
    cpu.w32(THREAD_BLOCK + 4, STACK_TOP);
    cpu.w32(THREAD_BLOCK + 8, STACK_TOP - STACK_SIZE);
    cpu.w32(THREAD_BLOCK + 0x18, THREAD_BLOCK);

    println!("{} modules loaded, {} functions, {} imports for the runtime", images.len(), table.len(), cpu.imports.len());
    if let Ok(path) = std::env::var("RECOMP_IMPORTS") {
        let missing: Vec<&str> = cpu.imports.iter().filter(|host| host.function.is_none()).map(|host| host.name.as_str()).collect();
        let _ = std::fs::write(path, missing.join("\n"));
    }
    cpu.host.title = game.title;
    cpu.host.command = command;
    cpu.host.icon = game.icon;
    cpu.host.music = game.music;
    cpu.host.cd = game.cd;
    cpu.host.registry = host::registry::Registry::load(&cpu.host.folder, game.registry);
    cpu.host.mods_off = game.mods.iter().zip(mods).filter(|(_, on)| !on).map(|((file, ..), _)| *file).collect();
    let entry = images[0].1.entry;
    for (name, image) in images {
        cpu.host.modules.push(host::LoadedModule {
            name,
            base: image.base,
            entry: image.entry,
            exports: image.exports,
            started: !image.is_dll, // The program's entry point runs last, below.
        });
    }
    for name in STARTED_FIRST {
        if let Some(index) = cpu.host.modules.iter().position(|module| module.name == *name) {
            cpu.start_module(index);
        }
    }
    cpu.push(0); // Returning from here ends the game.
    cpu.call(entry);
    println!("the game's entry point returned");
}

/// What the player chose, on the command line or in the launcher.
pub struct Settings {
    pub folder: String,
    /// How many times 640x480 3D is drawn at, from 1 to 6.
    pub scale: u32,
    pub fullscreen: bool,
    /// Whether Glide is drawn by the processor instead of the graphics card.
    pub processor: bool,
    /// One for each of the game's switches.
    pub switches: Vec<bool>,
    /// An index into the game's renderers.
    pub renderer: usize,
    /// An index into the game's languages.
    pub language: usize,
    /// One for each of the game's optional mods: whether it's on.
    pub mods: Vec<bool>,
}

/// The renderer or language named by a label or a switch, ignoring case.
pub fn find_choice(choices: &[(&str, &str)], name: &str) -> Option<usize> {
    choices.iter().position(|(switch, label)| label.eq_ignore_ascii_case(name) || switch.eq_ignore_ascii_case(name))
}

/// The game's command line, with the chosen renderer's and language's switches in place of
/// any others from those lists.
fn command_with_choices(game: &Game, settings: &Settings) -> String {
    let lists = [(game.renderers, settings.renderer), (game.languages, settings.language)];
    let listed = |word: &str| lists.iter().any(|(list, _)| list.iter().any(|(switch, _)| switch.eq_ignore_ascii_case(word)));
    let mut words: Vec<&str> = game.command.split_whitespace().filter(|word| !listed(word)).collect();
    for (list, index) in lists {
        if let Some((chosen, _)) = list.get(index) {
            words.push(chosen);
        }
    }
    words.join(" ")
}

pub const DEFAULT_SCALE: u32 = 3;

/// Reads `[folder] [--scale=N] [--fullscreen] [--drawing=processor|card] [--renderer=NAME] [--language=NAME] [--mod-off=NAME]
/// [--mod-on=NAME] [--play] [game switches]`.
fn settings_from_arguments(game: &Game, arguments: &[String]) -> Settings {
    let has = |wanted: &str| arguments.iter().any(|argument| argument.eq_ignore_ascii_case(wanted));
    let scale = arguments.iter().find_map(|argument| argument.strip_prefix("--scale=")?.parse().ok());
    let scale = scale.or_else(|| std::env::var("RECOMP_SCALE").ok()?.parse().ok());
    Settings {
        folder: arguments.iter().find(|argument| !argument.starts_with("--")).cloned().unwrap_or_else(|| default_folder(game)),
        scale: scale.unwrap_or(DEFAULT_SCALE).clamp(1, 6),
        fullscreen: has("--fullscreen"),
        processor: has("--drawing=processor"),
        switches: game.switches.iter().map(|(name, _, _)| has(name)).collect(),
        renderer: arguments.iter().find_map(|argument| find_choice(game.renderers, argument.strip_prefix("--renderer=")?)).unwrap_or(0),
        language: arguments.iter().find_map(|argument| find_choice(game.languages, argument.strip_prefix("--language=")?)).unwrap_or(0),
        mods: game.mods.iter().map(|(file, _, on, _)| {
            let named = |prefix: &str| arguments.iter().any(|argument| argument.strip_prefix(prefix).is_some_and(|name| name.eq_ignore_ascii_case(file)));
            if named("--mod-off=") { false } else if named("--mod-on=") { true } else { *on }
        }).collect(),
    }
}

/// Runs the game on a thread with a host stack deep enough for its calls.
pub fn run(game: &'static Game) {
    report_crashes(game.title);
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut settings = settings_from_arguments(game, &arguments);
    if host::launcher::wanted(&arguments) {
        // The launcher guesses no folder: it shows the one saved last time, or none to choose.
        if !arguments.iter().any(|argument| !argument.starts_with("--")) {
            settings.folder = String::new();
        }
        match host::launcher::show(game, settings) {
            host::launcher::Outcome::Start(chosen) => settings = chosen,
            host::launcher::Outcome::Quit => std::process::exit(0),
            host::launcher::Outcome::Unavailable(unchanged) => settings = unchanged,
        }
    }
    // The graphics code reads these when the game opens its window.
    std::env::set_var("RECOMP_SCALE", settings.scale.to_string());
    if settings.fullscreen {
        std::env::set_var("RECOMP_FULLSCREEN", "1");
    }
    // The player's own RECOMP_GLIDE wins, as it does for tests.
    if settings.processor && std::env::var_os("RECOMP_GLIDE").is_none() {
        std::env::set_var("RECOMP_GLIDE", "cpu");
    }
    let command: &'static str = Box::leak(command_with_choices(game, &settings).into_boxed_str());
    let Settings { folder, switches, mods, .. } = settings;
    if let Err(text) = check_files(game, &folder) {
        host::video::show_error(game.title, &text);
        std::process::exit(1);
    }
    let thread = std::thread::Builder::new().stack_size(HOST_STACK).spawn(move || start(game, folder, switches, mods, command));
    if thread.expect("couldn't start the game's thread").join().is_err() {
        std::process::exit(1);
    }
}
