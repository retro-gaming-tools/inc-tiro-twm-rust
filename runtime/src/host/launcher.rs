//! A settings screen before the game starts: the game folder, how sharp 3D is,
//! window or fullscreen, the game's own switches, and the mods that can be
//! turned off. It uses SDL's built-in
//! 8x8 font, works with the mouse, the keyboard and a controller, and keeps the
//! choices for next time.

use crate::host::video::{symbol, Event};
use crate::loader::{self, Game, Settings};
use crate::sys;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::sync::Mutex;

type Pointer = *mut c_void;

const INIT_VIDEO: u32 = 0x20;
const INIT_GAMEPAD: u32 = 0x2000;
const WINDOW_RESIZABLE: u64 = 0x20;
const LOGICAL_LETTERBOX: c_int = 2;
const PIXELFORMAT_RGBA32: u32 = 0x1676_2004;
const ICON_SIZE: c_int = 64;
const EVENT_QUIT: u32 = 0x100;
const EVENT_KEY_DOWN: u32 = 0x300;
const EVENT_MOUSE_MOTION: u32 = 0x400;
const EVENT_MOUSE_BUTTON_DOWN: u32 = 0x401;
const EVENT_GAMEPAD_BUTTON_DOWN: u32 = 0x651;
const EVENT_GAMEPAD_ADDED: u32 = 0x653;
const EVENT_DROP_FILE: u32 = 0x1000;
const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_SPACE: u32 = 32;
const KEY_RIGHT: u32 = 0x4000_004f;
const KEY_LEFT: u32 = 0x4000_0050;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const BUTTON_SOUTH: u8 = 0;
const BUTTON_EAST: u8 = 1;
const BUTTON_START: u8 = 6;
const BUTTON_UP: u8 = 11;
const BUTTON_DOWN: u8 = 12;
const BUTTON_LEFT: u8 = 13;
const BUTTON_RIGHT: u8 = 14;

/// The screen is laid out at 320x240, and scaled up to the window.
const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 240.0;
const CHARACTER: f32 = 8.0;
const MARGIN: f32 = 12.0;
const LONGEST_TEXT: usize = ((WIDTH - 2.0 * MARGIN - 8.0) / CHARACTER) as usize;
const WINDOW_SCALE: c_int = 3;
const FRAME_MILLISECONDS: u32 = 16;
const FIRST_SCALE: u32 = 1;
const LAST_SCALE: u32 = 6;
/// An organisation name for the settings folder, shared by every recompiled game.
const SETTINGS_OWNER: &str = "Recompiled Games";
const USER_SETTINGS_FILE: &str = "launcher.ini";
/// Beside the program.
const SETTINGS_FILE: &str = "settings.ini";
/// Marks a settings file as the launcher's, so a game's own file of that name is never overwritten.
const SETTINGS_HEADER: &str = "; Launcher settings for this recompiled game\n";

pub enum Outcome {
    Start(Settings),
    Quit,
    /// SDL couldn't open a window, so the game goes on with what it had.
    Unavailable(Settings),
}

/// The launcher shows unless `--play` is given, or a run is scripted or headless.
pub fn wanted(arguments: &[String]) -> bool {
    let scripted = std::env::var("RECOMP_SCRIPT").is_ok_and(|script| !script.trim().is_empty());
    let play = arguments.iter().any(|argument| argument.eq_ignore_ascii_case("--play"));
    !play && !scripted && std::env::var_os("RECOMP_HEADLESS").is_none()
}

#[derive(Clone, Copy, PartialEq)]
enum Row {
    Folder,
    Scale,
    /// Whether the graphics card or the processor draws Glide.
    Drawing,
    Display,
    Renderer,
    Language,
    Switch(usize),
    Mod(usize),
    /// Opens the page of settings most players should leave alone.
    Advanced,
    /// Leaves that page.
    Back,
    Start,
    Quit,
}

/// Where a folder chosen in SDL's dialog arrives, from its callback.
static PICKED: Mutex<Option<String>> = Mutex::new(None);

unsafe extern "C" fn folder_picked(_: *mut c_void, list: *const *const c_char, _: c_int) {
    if list.is_null() || (*list).is_null() {
        return; // Cancelled, or the system has no folder dialog.
    }
    let folder = CStr::from_ptr(*list).to_string_lossy().into_owned();
    *PICKED.lock().unwrap() = Some(folder);
}

struct Sdl {
    window: Pointer,
    renderer: Pointer,
    set_color: unsafe extern "C" fn(Pointer, u8, u8, u8, u8) -> bool,
    clear: unsafe extern "C" fn(Pointer) -> bool,
    fill_rect: unsafe extern "C" fn(Pointer, *const f32) -> bool,
    outline: unsafe extern "C" fn(Pointer, *const f32) -> bool,
    text: unsafe extern "C" fn(Pointer, f32, f32, *const c_char) -> bool,
    present: unsafe extern "C" fn(Pointer) -> bool,
    poll: unsafe extern "C" fn(*mut Event) -> bool,
    convert: unsafe extern "C" fn(Pointer, *mut Event) -> bool,
    pick_folder: unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int), *mut c_void, Pointer, *const c_char, bool),
    open_gamepad: unsafe extern "C" fn(u32) -> Pointer,
    delay: unsafe extern "C" fn(u32),
    destroy_renderer: unsafe extern "C" fn(Pointer),
    destroy_window: unsafe extern "C" fn(Pointer),
}

impl Sdl {
    fn open(game: &Game) -> Option<Sdl> {
        unsafe {
            let library = sys::open_sdl();
            if library.is_null() {
                return None;
            }
            let init: unsafe extern "C" fn(u32) -> bool = symbol(library, "SDL_Init");
            if !init(INIT_VIDEO) {
                return None;
            }
            init(INIT_GAMEPAD);
            let create_window: unsafe extern "C" fn(*const c_char, c_int, c_int, u64) -> Pointer = symbol(library, "SDL_CreateWindow");
            let title = CString::new(format!("{} {} - Launcher", game.title, game.version)).unwrap();
            let window = create_window(title.as_ptr(), WIDTH as c_int * WINDOW_SCALE, HEIGHT as c_int * WINDOW_SCALE, WINDOW_RESIZABLE);
            if window.is_null() {
                return None;
            }
            if game.icon.len() == (ICON_SIZE * ICON_SIZE * 4) as usize {
                let create_surface: unsafe extern "C" fn(c_int, c_int, u32, *const u8, c_int) -> Pointer = symbol(library, "SDL_CreateSurfaceFrom");
                let set_icon: unsafe extern "C" fn(Pointer, Pointer) -> bool = symbol(library, "SDL_SetWindowIcon");
                let surface = create_surface(ICON_SIZE, ICON_SIZE, PIXELFORMAT_RGBA32, game.icon.as_ptr(), ICON_SIZE * 4);
                if !surface.is_null() {
                    set_icon(window, surface);
                }
            }
            let create_renderer: unsafe extern "C" fn(Pointer, *const c_char) -> Pointer = symbol(library, "SDL_CreateRenderer");
            let renderer = create_renderer(window, std::ptr::null());
            if renderer.is_null() {
                return None;
            }
            let set_logical: unsafe extern "C" fn(Pointer, c_int, c_int, c_int) -> bool = symbol(library, "SDL_SetRenderLogicalPresentation");
            set_logical(renderer, WIDTH as c_int, HEIGHT as c_int, LOGICAL_LETTERBOX);
            Some(Sdl {
                window,
                renderer,
                set_color: symbol(library, "SDL_SetRenderDrawColor"),
                clear: symbol(library, "SDL_RenderClear"),
                fill_rect: symbol(library, "SDL_RenderFillRect"),
                outline: symbol(library, "SDL_RenderRect"),
                text: symbol(library, "SDL_RenderDebugText"),
                present: symbol(library, "SDL_RenderPresent"),
                poll: symbol(library, "SDL_PollEvent"),
                convert: symbol(library, "SDL_ConvertEventToRenderCoordinates"),
                pick_folder: symbol(library, "SDL_ShowOpenFolderDialog"),
                open_gamepad: symbol(library, "SDL_OpenGamepad"),
                delay: symbol(library, "SDL_Delay"),
                destroy_renderer: symbol(library, "SDL_DestroyRenderer"),
                destroy_window: symbol(library, "SDL_DestroyWindow"),
            })
        }
    }

    fn color(&self, (red, green, blue): (u8, u8, u8)) {
        unsafe { (self.set_color)(self.renderer, red, green, blue, 255) };
    }

    fn rect(&self, area: [f32; 4], colour: (u8, u8, u8), filled: bool) {
        self.color(colour);
        unsafe {
            if filled {
                (self.fill_rect)(self.renderer, area.as_ptr());
            } else {
                (self.outline)(self.renderer, area.as_ptr());
            }
        }
    }

    fn write(&self, x: f32, y: f32, text: &str, colour: (u8, u8, u8)) {
        self.color(colour);
        // The built-in font has only ASCII.
        let plain: String = text.chars().map(|c| if c.is_ascii() && !c.is_ascii_control() { c } else { '?' }).collect();
        let text = CString::new(plain).unwrap();
        unsafe { (self.text)(self.renderer, x, y, text.as_ptr()) };
    }

    fn write_right(&self, y: f32, text: &str, colour: (u8, u8, u8)) {
        self.write(WIDTH - MARGIN - 4.0 - text.len() as f32 * CHARACTER, y, text, colour);
    }

    fn close(self) {
        unsafe {
            (self.destroy_renderer)(self.renderer);
            (self.destroy_window)(self.window);
        }
    }
}

const BACKGROUND: (u8, u8, u8) = (14, 18, 16);
const PANEL: (u8, u8, u8) = (26, 34, 30);
const SELECTED: (u8, u8, u8) = (46, 92, 64);
const BORDER: (u8, u8, u8) = (90, 150, 110);
const TEXT: (u8, u8, u8) = (225, 230, 220);
const VALUE: (u8, u8, u8) = (235, 205, 110);
const GOOD: (u8, u8, u8) = (110, 225, 120);
const BAD: (u8, u8, u8) = (245, 105, 90);
const DIM: (u8, u8, u8) = (120, 130, 125);

/// The folder's state, in a line short enough for the screen.
fn folder_status(game: &Game, folder: &str) -> (String, bool) {
    match loader::check_files(game, folder) {
        Ok(()) => ("Game files found".to_string(), true),
        Err(text) => {
            let problem = text.lines().find(|line| line.starts_with("Missing") || line.starts_with("From a different")).unwrap_or("Game files not found");
            // A long list doesn't fit, and names a wrong folder better than any file does.
            let files = problem.split_once(": ").map_or(0, |(_, list)| list.split(", ").count());
            let short = match problem {
                _ if files <= 2 => problem,
                _ if problem.starts_with("Missing") => "Game files are missing",
                _ => "Game files are from a different version",
            };
            (short.to_string(), false)
        }
    }
}

/// Counts names in the game folder that another name in the same folder matches when case is
/// ignored. Windows can't hold both, so copying a patch over the game there replaces the old file,
/// but on Linux both stay, such as a translation's speech beside the original's, and the game
/// gets either one.
fn case_duplicates(folder: &str) -> usize {
    let mut count = 0;
    let mut folders = vec![std::path::PathBuf::from(folder)];
    while let Some(path) = folders.pop() {
        let Ok(entries) = std::fs::read_dir(&path) else { continue };
        let mut names = Vec::new();
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                folders.push(entry.path());
            }
            names.push(entry.file_name().to_string_lossy().to_lowercase());
        }
        names.sort();
        let all = names.len();
        names.dedup();
        if names.len() < all {
            eprintln!("launcher: {} has names that differ only in case", path.display());
        }
        count += all - names.len();
    }
    count
}

/// A warning about the folder, in two short lines, when files of a patch sit beside the ones it
/// was meant to replace.
fn folder_warning(folder: &str) -> Option<[String; 2]> {
    match case_duplicates(folder) {
        0 => None,
        1 => Some(["Warning: 1 file is there twice".to_string(), "Patch not copied over the old one?".to_string()]),
        count => Some([format!("Warning: {count} files are there twice"), "Patch not copied over the old ones?".to_string()]),
    }
}

/// The last characters of a long path, which are the telling part.
fn shorten(text: &str) -> String {
    let count = text.chars().count();
    if count <= LONGEST_TEXT {
        return text.to_string();
    }
    let tail: String = text.chars().skip(count - (LONGEST_TEXT - 3)).collect();
    format!("...{tail}")
}

fn absolute(folder: &str) -> String {
    let full = std::fs::canonicalize(folder).map_or(folder.to_string(), |path| path.display().to_string());
    // Windows gives the long-path form, which nobody types.
    full.strip_prefix(r"\\?\").map(str::to_string).unwrap_or(full)
}

/// The folder of the program the player started. An AppImage's program sits in a read-only
/// mount, so for one it's the folder holding the AppImage file.
fn program_folder() -> Option<std::path::PathBuf> {
    let program = std::env::var_os("APPIMAGE").map(std::path::PathBuf::from).or_else(|| std::env::current_exe().ok())?;
    program.parent().map(|folder| folder.to_path_buf())
}

/// Where the settings go: beside the program, unless a game's own file of that name is there.
fn settings_beside_program() -> Option<std::path::PathBuf> {
    let path = program_folder()?.join(SETTINGS_FILE);
    let ours = std::fs::read_to_string(&path).map_or(!path.exists(), |text| text.starts_with(SETTINGS_HEADER));
    ours.then_some(path)
}

/// SDL's settings folder for the game, used when the program's folder can't be written, and
/// where earlier builds kept their settings.
fn settings_path(game: &Game) -> Option<std::path::PathBuf> {
    unsafe {
        let library = sys::open_sdl();
        if library.is_null() {
            return None;
        }
        let pref_path: unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_char = symbol(library, "SDL_GetPrefPath");
        let (owner, title) = (CString::new(SETTINGS_OWNER).unwrap(), CString::new(game.title).unwrap());
        let folder = pref_path(owner.as_ptr(), title.as_ptr());
        if folder.is_null() {
            return None;
        }
        Some(std::path::PathBuf::from(CStr::from_ptr(folder).to_string_lossy().into_owned()).join(USER_SETTINGS_FILE))
    }
}

/// Fills in what the last run chose, where the command line didn't say.
fn load(game: &Game, settings: &mut Settings, arguments_gave_folder: bool) {
    let arguments_gave_renderer = std::env::args().any(|argument| argument.starts_with("--renderer="));
    let arguments_gave_language = std::env::args().any(|argument| argument.starts_with("--language="));
    let beside = settings_beside_program().and_then(|path| std::fs::read_to_string(path).ok());
    let Some(text) = beside.or_else(|| settings_path(game).and_then(|path| std::fs::read_to_string(path).ok())) else { return };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else { continue };
        match key.trim() {
            "folder" if !arguments_gave_folder && !value.trim().is_empty() => settings.folder = value.trim().to_string(),
            "scale" => settings.scale = value.trim().parse().unwrap_or(settings.scale).clamp(FIRST_SCALE, LAST_SCALE),
            "fullscreen" => settings.fullscreen = value.trim() == "1",
            "drawing" if !std::env::args().any(|argument| argument.starts_with("--drawing=")) => settings.processor = value.trim() == "processor",
            "renderer" if !arguments_gave_renderer => settings.renderer = loader::find_choice(game.renderers, value.trim()).unwrap_or(settings.renderer),
            "language" if !arguments_gave_language => settings.language = loader::find_choice(game.languages, value.trim()).unwrap_or(settings.language),
            name => {
                if let Some(index) = game.switches.iter().position(|(switch, _, _)| *switch == name) {
                    settings.switches[index] = value.trim() == "1";
                }
                let file = name.strip_prefix("mod.").unwrap_or_default();
                let named_on_command_line = std::env::args().any(|argument| {
                    argument.strip_prefix("--mod-off=").or_else(|| argument.strip_prefix("--mod-on=")).is_some_and(|named| named.eq_ignore_ascii_case(file))
                });
                if let Some(index) = game.mods.iter().position(|(mod_file, ..)| *mod_file == file) {
                    if !named_on_command_line {
                        settings.mods[index] = value.trim() == "1";
                    }
                }
            }
        }
    }
}

fn save(game: &Game, settings: &Settings) {
    let mut text = format!("{SETTINGS_HEADER}folder={}\nscale={}\nfullscreen={}\n", absolute(&settings.folder), settings.scale, settings.fullscreen as u8);
    text.push_str(&format!("drawing={}\n", if settings.processor { "processor" } else { "card" }));
    if let Some((_, label)) = game.renderers.get(settings.renderer) {
        text.push_str(&format!("renderer={label}\n"));
    }
    if let Some((_, label)) = game.languages.get(settings.language) {
        text.push_str(&format!("language={label}\n"));
    }
    for ((name, _, _), on) in game.switches.iter().zip(&settings.switches) {
        text.push_str(&format!("{name}={}\n", *on as u8));
    }
    for ((file, ..), on) in game.mods.iter().zip(&settings.mods) {
        text.push_str(&format!("mod.{file}={}\n", *on as u8));
    }
    // A folder such as Program Files can refuse the write.
    let written = settings_beside_program().is_some_and(|path| std::fs::write(path, &text).is_ok());
    if !written {
        if let Some(path) = settings_path(game) {
            let _ = std::fs::write(path, text);
        }
    }
}

const FOLDER_TOP: f32 = 34.0;
const FOLDER_HEIGHT: f32 = 42.0;
const ROWS_TOP: f32 = 84.0;
const ROW_HEIGHT: f32 = 18.0;
const ROW_GAP: f32 = 2.0;
/// Where the rows must end, above the two hint lines.
const ROWS_BOTTOM: f32 = HEIGHT - 30.0;

/// The rows of the first page, or of the advanced one. That page holds what
/// the defaults get right for nearly everyone: who draws the 3D, the game's
/// renderer, and the mods marked `launcher-advanced`. The first page offers
/// it only if it has rows.
fn rows(game: &Game, advanced: bool) -> Vec<Row> {
    let mut more = Vec::new();
    // Only Glide is drawn by the runtime, so only it can go to the graphics card.
    if game.renderers.iter().any(|(_, label)| label.eq_ignore_ascii_case("Glide")) {
        more.push(Row::Drawing);
    }
    if game.renderers.len() > 1 {
        more.push(Row::Renderer);
    }
    let mods = |on_advanced: bool| (0..game.mods.len()).filter(move |index| game.mods[*index].3 == on_advanced).map(Row::Mod);
    more.extend(mods(true));
    if advanced {
        more.push(Row::Back);
        return more;
    }
    let mut rows = vec![Row::Folder, Row::Scale, Row::Display];
    if game.languages.len() > 1 {
        rows.push(Row::Language);
    }
    rows.extend((0..game.switches.len()).map(Row::Switch));
    rows.extend(mods(false));
    if !more.is_empty() {
        rows.push(Row::Advanced);
    }
    rows.extend([Row::Start, Row::Quit]);
    rows
}

/// A row's top and height on the screen. Rows get shorter when there are too
/// many to fit at the usual height, down to a little more than the font. The
/// first page's first row is the folder's, which is taller and set apart.
fn place(index: usize, count: usize, advanced: bool) -> (f32, f32) {
    let folder = usize::from(!advanced);
    if index < folder {
        return (FOLDER_TOP, FOLDER_HEIGHT);
    }
    let pitch = ((ROWS_BOTTOM - ROWS_TOP) / (count - folder).max(1) as f32).min(ROW_HEIGHT + ROW_GAP);
    let gap = (pitch * ROW_GAP / (ROW_HEIGHT + ROW_GAP)).max(1.0);
    (ROWS_TOP + (index - folder) as f32 * pitch, pitch - gap)
}

fn draw(sdl: &Sdl, game: &Game, settings: &Settings, status: &(String, bool), warning: &Option<[String; 2]>, selected: usize, advanced: bool) {
    unsafe {
        sdl.color(BACKGROUND);
        (sdl.clear)(sdl.renderer);
    }
    let title = game.title.to_uppercase();
    sdl.write((WIDTH - title.len() as f32 * CHARACTER) / 2.0, 12.0, &title, VALUE);
    sdl.write_right(12.0, game.version, DIM);
    let all = rows(game, advanced);
    if advanced {
        let left = MARGIN + 4.0;
        sdl.write(left, FOLDER_TOP + 4.0, "Advanced settings", TEXT);
        sdl.write(left, FOLDER_TOP + 17.0, "The defaults suit most machines.", DIM);
        sdl.write(left, FOLDER_TOP + 30.0, "Change them if the game runs badly.", DIM);
    }
    for (index, row) in all.iter().enumerate() {
        let (top, height) = place(index, all.len(), advanced);
        let area = [MARGIN, top, WIDTH - 2.0 * MARGIN, height];
        sdl.rect(area, if index == selected { SELECTED } else { PANEL }, true);
        if index == selected {
            sdl.rect(area, BORDER, false);
        }
        let line = top + (height - CHARACTER) / 2.0;
        let left = MARGIN + 4.0;
        match row {
            Row::Folder => {
                sdl.write(left, top + 4.0, "Game folder", TEXT);
                sdl.write_right(top + 4.0, "Choose...", DIM);
                sdl.write(left, top + 17.0, &shorten(&absolute(&settings.folder)), VALUE);
                sdl.write(left, top + 30.0, &shorten(&status.0), if status.1 { GOOD } else { BAD });
            }
            Row::Scale | Row::Drawing | Row::Display | Row::Renderer | Row::Language | Row::Switch(_) | Row::Mod(_) => {
                let label = match row {
                    Row::Scale => "3D resolution",
                    Row::Drawing => "3D drawn by",
                    Row::Display => "Display",
                    Row::Renderer => "Renderer",
                    Row::Language => "Text language",
                    Row::Switch(switch) => game.switches[*switch].2,
                    Row::Mod(index) => game.mods[*index].1,
                    _ => "",
                };
                if *row == Row::Scale && fixed_resolution(game, settings) {
                    // Greyed out, with no arrows: there's nothing to choose.
                    sdl.write(left, line, label, DIM);
                    sdl.write_right(line, "640 x 480", DIM);
                } else if *row == Row::Drawing && fixed_resolution(game, settings) {
                    // The game's own software renderer runs on the processor.
                    sdl.write(left, line, label, DIM);
                    sdl.write_right(line, "CPU", DIM);
                } else {
                    sdl.write(left, line, label, TEXT);
                    sdl.write_right(line, &value_text(game, *row, settings).unwrap_or_default(), VALUE);
                }
            }
            Row::Start => {
                let label = "Start game";
                let colour = if status.1 { GOOD } else { DIM };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, colour);
            }
            Row::Advanced | Row::Back | Row::Quit => {
                let label = match row {
                    Row::Advanced => "Advanced settings...",
                    Row::Back => "Back",
                    _ => "Quit",
                };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, TEXT);
            }
        }
    }
    // A warning takes the place of the two hint lines, which are the least needed.
    let lines = match warning {
        Some([first, second]) => [(first.as_str(), VALUE), (second.as_str(), VALUE)],
        None => {
            let hint = if fixed_resolution(game, settings) {
                "Software draws 3D at 640 x 480"
            } else if settings.processor && settings.scale >= 4 {
                "Higher 3D resolutions need a fast CPU"
            } else {
                "Drop the game folder on this window"
            };
            [(hint, DIM), (if advanced { "Arrows/d-pad, Enter/A, Esc to go back" } else { "Arrows/d-pad, Enter/A, Esc to quit" }, DIM)]
        }
    };
    for (index, (text, colour)) in lines.iter().enumerate() {
        sdl.write((WIDTH - text.len() as f32 * CHARACTER) / 2.0, HEIGHT - 26.0 + index as f32 * 12.0, text, *colour);
    }
    unsafe { (sdl.present)(sdl.renderer) };
}

/// Whether the chosen renderer draws 3D at 640x480 whatever the resolution row says. Only Glide
/// is drawn by the runtime, at any size. The chosen resolution is kept for when Glide is chosen
/// again.
fn fixed_resolution(game: &Game, settings: &Settings) -> bool {
    game.renderers.get(settings.renderer).is_some_and(|(_, label)| label.eq_ignore_ascii_case("Software"))
}

/// The value a setting row shows on its right, between its arrows.
fn value_text(game: &Game, row: Row, settings: &Settings) -> Option<String> {
    let value = match row {
        Row::Scale if fixed_resolution(game, settings) => return None,
        Row::Scale => format!("{} x {}", 640 * settings.scale, 480 * settings.scale),
        Row::Drawing if fixed_resolution(game, settings) => return None,
        // "Graphics card (OpenGL)" doesn't fit beside the label.
        Row::Drawing => (if settings.processor { "CPU" } else { "GPU (OpenGL)" }).to_string(),
        Row::Display => (if settings.fullscreen { "Fullscreen" } else { "Window" }).to_string(),
        Row::Renderer => game.renderers.get(settings.renderer)?.1.to_string(),
        Row::Language => game.languages.get(settings.language)?.1.to_string(),
        Row::Switch(switch) => (if settings.switches[switch] { "On" } else { "Off" }).to_string(),
        Row::Mod(index) => (if settings.mods[index] { "On" } else { "Off" }).to_string(),
        _ => return None,
    };
    Some(format!("< {value} >"))
}

/// The step a click at `x` on a row means: down on its `<`, up on its `>`, and
/// otherwise what Enter does.
fn click_step(game: &Game, row: Row, settings: &Settings, x: f32) -> i32 {
    let Some(text) = value_text(game, row, settings) else { return 0 };
    let right = WIDTH - MARGIN - 4.0;
    let left = right - text.len() as f32 * CHARACTER;
    // A little wider than the arrow itself, which is one character.
    if (left - CHARACTER..left + 1.5 * CHARACTER).contains(&x) {
        -1
    } else if (right - 1.5 * CHARACTER..right + CHARACTER).contains(&x) {
        1
    } else {
        0
    }
}

/// Changes a setting by one step, or does what a button row does. Returns the
/// outcome when the launcher is done.
fn change(sdl: &Sdl, game: &Game, row: Row, step: i32, settings: &mut Settings, ready: bool) -> Option<bool> {
    match row {
        Row::Folder => {
            let start = CString::new(absolute(&settings.folder)).unwrap();
            unsafe { (sdl.pick_folder)(folder_picked, std::ptr::null_mut(), sdl.window, start.as_ptr(), false) };
        }
        Row::Scale if fixed_resolution(game, settings) => {}
        Row::Scale => {
            let next = settings.scale as i32 + if step == 0 { 1 } else { step };
            settings.scale = if next > LAST_SCALE as i32 { FIRST_SCALE } else if next < FIRST_SCALE as i32 { LAST_SCALE } else { next as u32 };
        }
        Row::Drawing if fixed_resolution(game, settings) => {}
        Row::Drawing => settings.processor = !settings.processor,
        Row::Display => settings.fullscreen = !settings.fullscreen,
        Row::Renderer => {
            let count = game.renderers.len() as i32;
            settings.renderer = (settings.renderer as i32 + if step == 0 { 1 } else { step }).rem_euclid(count) as usize;
        }
        Row::Language => {
            let count = game.languages.len() as i32;
            settings.language = (settings.language as i32 + if step == 0 { 1 } else { step }).rem_euclid(count) as usize;
        }
        Row::Switch(index) => settings.switches[index] = !settings.switches[index],
        Row::Mod(index) => settings.mods[index] = !settings.mods[index],
        Row::Start if step == 0 && ready => return Some(true),
        Row::Quit if step == 0 => return Some(false),
        _ => {}
    }
    None
}

pub fn show(game: &Game, mut settings: Settings) -> Outcome {
    let gave_folder = std::env::args().skip(1).any(|argument| !argument.starts_with("--"));
    load(game, &mut settings, gave_folder);
    let Some(sdl) = Sdl::open(game) else { return Outcome::Unavailable(settings) };
    let mut advanced = false;
    let mut all = rows(game, advanced);
    let start_row = all.iter().position(|row| *row == Row::Start).unwrap_or(0);
    let mut status = folder_status(game, &settings.folder);
    let mut warning = folder_warning(&settings.folder);
    // Start is ready to press, unless the folder needs choosing first.
    let mut selected = if status.1 { start_row } else { 0 };
    let mut event = Event([0; 128]);
    let finished = loop {
        let mut result = None;
        while unsafe { (sdl.poll)(&mut event) } {
            unsafe { (sdl.convert)(sdl.renderer, &mut event) };
            let action = match event.u32_at(0) {
                EVENT_QUIT => Some((None, 0, true)),
                EVENT_KEY_DOWN => match event.u32_at(28) {
                    KEY_UP => Some((Some(-1), 0, false)),
                    KEY_DOWN => Some((Some(1), 0, false)),
                    KEY_LEFT => Some((None, -1, false)),
                    KEY_RIGHT => Some((None, 1, false)),
                    KEY_RETURN | KEY_SPACE => Some((None, 0, false)),
                    KEY_ESCAPE => Some((None, 0, true)),
                    _ => None,
                },
                EVENT_GAMEPAD_ADDED => {
                    unsafe { (sdl.open_gamepad)(event.u32_at(16)) };
                    None
                }
                EVENT_GAMEPAD_BUTTON_DOWN => match event.0[20] {
                    BUTTON_UP => Some((Some(-1), 0, false)),
                    BUTTON_DOWN => Some((Some(1), 0, false)),
                    BUTTON_LEFT => Some((None, -1, false)),
                    BUTTON_RIGHT => Some((None, 1, false)),
                    BUTTON_SOUTH => Some((None, 0, false)),
                    BUTTON_START => {
                        advanced = false;
                        all = rows(game, advanced);
                        selected = start_row;
                        Some((None, 0, false))
                    }
                    BUTTON_EAST => Some((None, 0, true)),
                    _ => None,
                },
                kind @ (EVENT_MOUSE_MOTION | EVENT_MOUSE_BUTTON_DOWN) => {
                    let (x, y) = (event.f32_at(28), event.f32_at(32));
                    let hit = (0..all.len()).find(|index| {
                        let (top, height) = place(*index, all.len(), advanced);
                        (MARGIN..WIDTH - MARGIN).contains(&x) && (top..top + height).contains(&y)
                    });
                    match hit {
                        Some(index) => {
                            selected = index;
                            (kind == EVENT_MOUSE_BUTTON_DOWN).then(|| (None, click_step(game, all[index], &settings, x), false))
                        }
                        None => None,
                    }
                }
                EVENT_DROP_FILE => {
                    let pointer = usize::from_le_bytes(event.0[40..48].try_into().unwrap()) as *const c_char;
                    if !pointer.is_null() {
                        let dropped = unsafe { CStr::from_ptr(pointer) }.to_string_lossy().into_owned();
                        let path = std::path::Path::new(&dropped);
                        // A dropped file means the folder it's in.
                        let folder = if path.is_dir() { path.to_path_buf() } else { path.parent().map(|p| p.to_path_buf()).unwrap_or_default() };
                        *PICKED.lock().unwrap() = Some(folder.display().to_string());
                    }
                    None
                }
                _ => None,
            };
            let Some((movement, step, quit)) = action else { continue };
            // Escape and B leave the advanced page before they leave the
            // launcher, and closing the window always quits.
            let closing = event.u32_at(0) == EVENT_QUIT;
            let leave = advanced && ((quit && !closing) || (step == 0 && movement.is_none() && all[selected] == Row::Back));
            let enter = !advanced && !quit && step == 0 && movement.is_none() && all[selected] == Row::Advanced;
            if leave || enter {
                advanced = enter;
                all = rows(game, advanced);
                selected = if enter { 0 } else { all.iter().position(|row| *row == Row::Advanced).unwrap_or(0) };
                continue;
            }
            if quit {
                result = Some(false);
                break;
            }
            if let Some(movement) = movement {
                selected = (selected as i32 + movement).rem_euclid(all.len() as i32) as usize;
                continue;
            }
            if let Some(done) = change(&sdl, game, all[selected], step, &mut settings, status.1) {
                result = Some(done);
                break;
            }
        }
        if let Some(folder) = PICKED.lock().unwrap().take() {
            settings.folder = folder;
            status = folder_status(game, &settings.folder);
            warning = folder_warning(&settings.folder);
            // The folder's row is on the first page.
            advanced = false;
            all = rows(game, advanced);
            selected = if status.1 { start_row } else { 0 };
        }
        if let Some(done) = result {
            break done;
        }
        draw(&sdl, game, &settings, &status, &warning, selected, advanced);
        unsafe { (sdl.delay)(FRAME_MILLISECONDS) };
    };
    sdl.close();
    if finished {
        save(game, &settings);
        Outcome::Start(settings)
    } else {
        Outcome::Quit
    }
}
