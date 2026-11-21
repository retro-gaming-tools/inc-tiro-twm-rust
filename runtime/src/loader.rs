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
    /// The version players know the game by, from `recompVersionNames`, such
    /// as `GOG release 1.52e`, on the program's module. The launcher shows it.
    pub name: &'static str,
}

/// The name of the version in a folder or disc image that passed `check_files` or
/// `check_image`: its files match the hashes, so it's the version the program's module names.
pub(crate) fn version_in(game: &Game) -> Option<&'static str> {
    Some(game.modules.first()?.name).filter(|name| !name.is_empty())
}

/// Where the game's files sit on a disc: the folder holding the program, with
/// the program's own folders in its name taken off, such as `GAME/` for a disc
/// with `GAME/PROGRAM.EXE`, or "" for the disc's top.
fn image_root(game: &Game, files: &[host::disc::Entry]) -> Option<String> {
    let program = game.modules.first()?.file.to_lowercase();
    let suffix = format!("/{program}");
    let mut roots: Vec<String> = files
        .iter()
        .map(|entry| entry.path.to_lowercase())
        .filter_map(|path| if path == program { Some(String::new()) } else { path.strip_suffix(&suffix).map(|root| format!("{root}/")) })
        .collect();
    // The shallowest copy, since a disc can carry a second one deeper down.
    roots.sort_by_key(|root| root.matches('/').count());
    let root = roots.into_iter().next()?;
    // Back to the disc's spelling, for the unpacked folder's name.
    let spelled = files.iter().find(|entry| entry.path.to_lowercase().starts_with(&root)).map(|entry| entry.path[..root.len()].to_string());
    Some(spelled.unwrap_or(root))
}

/// Checks a disc image as `check_files` checks a folder, with a reason short
/// enough for the launcher. Gives where on the disc the game is.
pub(crate) fn check_image(game: &Game, image: &std::path::Path) -> Result<String, String> {
    let mut disc = host::disc::Disc::open(image).map_err(|_| "Not a disc image this can read".to_string())?;
    let files = disc.files();
    // The count tells a disc of something else from an image that couldn't be
    // read: that one has no files at all.
    let root = image_root(game, &files).ok_or_else(|| format!("Not this game's disc ({} files)", files.len()))?;
    for module in game.modules {
        let place = format!("{root}{}", module.file);
        let entry = files.iter().find(|entry| entry.path.eq_ignore_ascii_case(&place)).ok_or_else(|| format!("Not on the disc: {}", module.file))?;
        let data = disc.read(entry).map_err(|_| "The disc image is damaged".to_string())?;
        let found: String = Sha256::digest(&data).iter().map(|byte| format!("{byte:02x}")).collect();
        if found != module.sha256 {
            return Err("This disc is a different version".to_string());
        }
    }
    Ok(root)
}

/// Where a disc image is unpacked: a folder named `game` beside the program,
/// which is also where a start without any path looks first.
pub(crate) fn unpack_target() -> std::path::PathBuf {
    host::launcher::program_folder().unwrap_or_default().join("game")
}

/// Unpacks a disc image that `check_image` passed into `game` beside the program,
/// unless that already holds the right files, and gives the game's folder in it.
/// `progress` gets the bytes written so far and the total.
pub(crate) fn unpack_image(game: &Game, image: &std::path::Path, progress: &mut dyn FnMut(u64, u64)) -> Result<String, String> {
    let root = check_image(game, image)?;
    let target = unpack_target();
    let folder = target.join(root.trim_end_matches('/')).display().to_string();
    if check_files(game, &folder).is_ok() {
        return Ok(folder);
    }
    if target.exists() {
        return Err(format!("{} is in the way: it holds other files. Move or delete it.", target.display()));
    }
    host::disc::unpack(image, &target, progress)?;
    Ok(folder)
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
    /// Mods with a row in the launcher. Mods not listed are always on.
    pub mods: &'static [ModRow],
    /// Mods' checks of the chosen folder and settings (`//! launcher-check:`),
    /// whose warnings the launcher shows.
    pub checks: &'static [LauncherCheck],
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
    /// The launcher's look, from `launcherTheme` in port.json, as
    /// `name=value` pairs split by `;`: see `host::launcher::Theme`. Empty is
    /// the plain look.
    pub theme: &'static str,
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
    // A name may hold folders, such as `Z/WINZ.EXE` on a copy of a disc. Each
    // part is matched without regard to case.
    let mut path = std::path::PathBuf::from(folder);
    for part in name.split('/') {
        path = std::fs::read_dir(&path).ok()?.flatten().find(|entry| entry.file_name().to_string_lossy().eq_ignore_ascii_case(part)).map(|entry| entry.path())?;
    }
    Some(path)
}

fn load_images(modules: &[Module], folder: &str) -> Vec<(String, pe::Image)> {
    modules
        .iter()
        .map(|module| {
            let path = find_file(folder, module.file).unwrap_or_else(|| panic!("{} is missing from {folder}", module.file));
            let file = std::fs::read(path).expect("a game file can't be read");
            let mut image = pe::Image::parse(&file).expect("a game file isn't a 32-bit Windows program");
            image.rebase(module.base);
            // Other modules import it by its name alone, whatever folder it's in.
            (module.file.rsplit('/').next().unwrap_or(module.file).to_lowercase(), image)
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

fn start(game: &'static Game, folder: String, switches: Vec<bool>, mods: Vec<usize>, command: &'static str) {
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
    cpu.host.mod_choices = game.mods.iter().zip(mods).map(|(row, chosen)| (row.file, chosen)).collect();
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
    /// One for each of the game's mod rows: the chosen value's index (0 is Off).
    pub mods: Vec<usize>,
}

/// What a mod's launcher check is told: the chosen game folder, and the
/// chosen text language's label, such as `English` (empty for a game with
/// no language list).
pub struct Choices<'a> {
    pub folder: &'a std::path::Path,
    pub language: &'a str,
}

/// A mod's check of the launcher's choices: two short lines of warning, or
/// `None` when all is well.
pub type LauncherCheck = fn(&Choices) -> Option<[String; 2]>;

/// The first warning any of the game's checks gives for these settings.
pub fn check_choices(game: &Game, settings: &Settings) -> Option<[String; 2]> {
    let language = game.languages.get(settings.language).map_or("", |(_, label)| *label);
    let choices = Choices { folder: std::path::Path::new(&settings.folder), language };
    game.checks.iter().find_map(|check| check(&choices))
}

/// A mod's row in the launcher: the file in `mods/`, its label, the values it
/// steps through (`Off`, `On` for an On/Off row), the index it starts with,
/// and whether it's on the launcher's advanced page.
pub struct ModRow {
    pub file: &'static str,
    pub label: &'static str,
    pub values: &'static [&'static str],
    pub default: usize,
    pub advanced: bool,
}

impl ModRow {
    /// A value's index, from its name (ignoring case) or its number.
    pub fn find(&self, value: &str) -> Option<usize> {
        let value = value.trim();
        self.values.iter().position(|name| name.eq_ignore_ascii_case(value)).or_else(|| value.parse().ok().filter(|&at| at < self.values.len()))
    }
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
/// [--mod-on=NAME] [--mod=NAME:VALUE] [--play] [game switches]`. `--mod-off` picks a row's
/// first value and `--mod-on` its second; `--mod=` takes a value's name or number.
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
        mods: game.mods.iter().map(|row| mod_from_arguments(row, arguments).unwrap_or(row.default)).collect(),
    }
}

/// The value a command line chose for a mod row, if it named the row.
pub fn mod_from_arguments(row: &ModRow, arguments: &[String]) -> Option<usize> {
    let named = |prefix: &str| arguments.iter().any(|argument| argument.strip_prefix(prefix).is_some_and(|name| name.eq_ignore_ascii_case(row.file)));
    let chosen = arguments.iter().find_map(|argument| {
        let (file, value) = argument.strip_prefix("--mod=")?.split_once(':')?;
        file.eq_ignore_ascii_case(row.file).then(|| row.find(value)).flatten()
    });
    chosen.or(if named("--mod-off=") { Some(0) } else if named("--mod-on=") { Some(1) } else { None })
}

/// Runs the game on a thread with a host stack deep enough for its calls.
pub fn run(game: &'static Game) {
    report_crashes(game.title);
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut settings = settings_from_arguments(game, &arguments);
    host::launcher::snapshot(game, &settings);
    if host::launcher::wanted(&arguments) {
        // The launcher guesses no folder: it shows the one saved last time, or none to choose.
        if !arguments.iter().any(|argument| !argument.starts_with("--")) {
            settings.folder = String::new();
        }
        // A player starts in fullscreen unless they chose a window before
        // (the saved setting wins) or asked for one with `--window`.
        settings.fullscreen = !arguments.iter().any(|argument| argument == "--window");
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
    let Settings { mut folder, switches, mods, .. } = settings;
    // A disc image given on the command line, with no launcher to unpack it.
    if host::disc::is_image(std::path::Path::new(&folder)) {
        match unpack_image(game, std::path::Path::new(&folder), &mut |_, _| {}) {
            Ok(unpacked) => folder = unpacked,
            Err(problem) => {
                host::video::show_error(game.title, &format!("{} can't use this disc image:\n{folder}\n\n{problem}.\n\n{} needs {}.", game.title, game.title, game.needs));
                std::process::exit(1);
            }
        }
    }
    if let Err(text) = check_files(game, &folder) {
        host::video::show_error(game.title, &text);
        std::process::exit(1);
    }
    let thread = std::thread::Builder::new().stack_size(HOST_STACK).spawn(move || start(game, folder, switches, mods, command));
    if thread.expect("couldn't start the game's thread").join().is_err() {
        std::process::exit(1);
    }
}
