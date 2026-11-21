//! A settings screen before the game starts: the game folder, how sharp 3D is,
//! window or fullscreen, the game's own switches, and the mods that can be
//! turned off. It uses SDL's built-in
//! 8x8 font, works with the mouse, the keyboard and a controller, and keeps the
//! choices for next time.

use crate::host::disc;
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
/// The window opens at the largest whole scale of 320x240 that fits the screen's usable area
/// with room for a title bar, up to this.
const WINDOW_SCALE: c_int = 4;
/// When the screen's size isn't known: 640x480, which fits a 1024x768 screen.
const FALLBACK_WINDOW_SCALE: c_int = 2;
/// Room for the window's title bar and frame, which the usable area doesn't take off.
const WINDOW_FRAME: c_int = 48;

/// The window's scale for a usable screen area of `usable` (width, height), if known.
fn window_scale(usable: Option<(c_int, c_int)>) -> c_int {
    match usable {
        Some((width, height)) if width > 0 && height > 0 => {
            let fits = (width / WIDTH as c_int).min((height - WINDOW_FRAME) / HEIGHT as c_int);
            fits.clamp(1, WINDOW_SCALE)
        }
        _ => FALLBACK_WINDOW_SCALE,
    }
}
/// The game folder's path is written at 3/4 size, so a long one fits on two lines. At the
/// window's scale of 4, that's still whole pixels.
const SMALL: f32 = 0.75;
const SMALL_LONGEST: usize = ((WIDTH - 2.0 * MARGIN - 8.0) / (CHARACTER * SMALL)) as usize;
/// The header line `TITLE - RUST - version` gets the window's width less 4
/// pixels at each side. The recompiler refuses a title too long for it even at
/// 3/4 size (`HEADER_LONGEST` in its `main.rs`), so it's never cut off.
const HEADER_ROOM: f32 = WIDTH - 8.0;

/// The size the header is written at: full, or 3/4 when that doesn't fit.
fn header_size(title: &str, version: &str) -> f32 {
    let characters = (title.len() + " - RUST - ".len() + version.len()) as f32;
    if characters * CHARACTER <= HEADER_ROOM { 1.0 } else { SMALL }
}
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
    /// Shows the game's folder or disc image, and picks a folder in a folder dialog.
    Folder,
    /// Picks a disc image, or the game folder by a file in it such as the program.
    File,
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

/// What the player picked or dropped, as the game's place: a disc image stays as it is, and
/// any other file, such as the game's program, stands for the folder it's in.
fn game_place(picked: &str) -> String {
    let path = std::path::Path::new(picked);
    if path.is_dir() || (disc::is_image(path) && disc::Disc::open(path).is_ok()) {
        return picked.to_string();
    }
    path.parent().map_or(picked.to_string(), |folder| folder.display().to_string())
}

/// Unpacks a chosen disc image into `game` beside the program, showing how far it is, and
/// returns the game's folder there.
fn unpack_image(sdl: &Sdl, game: &Game, image: &str) -> Result<String, String> {
    let mut shown = u64::MAX;
    let mut event = Event([0; 128]);
    loader::unpack_image(game, std::path::Path::new(image), &mut |written, total| {
        let percent = written * 100 / total.max(1);
        if percent == shown {
            return;
        }
        shown = percent;
        // Reading the events keeps the system from calling the window frozen.
        while unsafe { (sdl.poll)(&mut event) } {}
        sdl.background();
        sdl.header(&game.title.to_uppercase(), game.version);
        let text = format!("Unpacking the disc: {percent}%");
        sdl.write((WIDTH - text.len() as f32 * CHARACTER) / 2.0, HEIGHT / 2.0 - 12.0, &text, theme().text);
        let bar = [MARGIN, HEIGHT / 2.0 + 4.0, WIDTH - 2.0 * MARGIN, 10.0];
        sdl.rect(bar, theme().panel, true);
        sdl.rect([bar[0], bar[1], bar[2] * percent as f32 / 100.0, bar[3]], theme().border, true);
        unsafe { (sdl.present)(sdl.renderer) };
    })
}

/// Where a dialog opens: the chosen folder, or the folder of the chosen file, if it's there,
/// else the program's folder. A folder that doesn't exist, such as the default `game`, makes
/// some dialogs fail to open.
fn dialog_start(chosen: &str) -> CString {
    let path = std::path::Path::new(chosen);
    let folder = if path.is_dir() { Some(path.to_path_buf()) } else if path.is_file() { path.parent().map(|p| p.to_path_buf()) } else { None };
    let start = match folder {
        Some(folder) => absolute(&folder.display().to_string()),
        None => program_folder().map(|p| p.display().to_string()).unwrap_or_default(),
    };
    let start = if start.ends_with(std::path::MAIN_SEPARATOR) { start } else { format!("{start}{}", std::path::MAIN_SEPARATOR) };
    CString::new(start).unwrap_or_default()
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
    pick_file: unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void, *const *const c_char, c_int), *mut c_void, Pointer, *const c_void, c_int, *const c_char, bool),
    set_scale: unsafe extern "C" fn(Pointer, f32, f32) -> bool,
    open_gamepad: unsafe extern "C" fn(u32) -> Pointer,
    delay: unsafe extern "C" fn(u32),
    destroy_renderer: unsafe extern "C" fn(Pointer),
    destroy_window: unsafe extern "C" fn(Pointer),
    /// For the `launcher:NAME` script step: the drawn screen as a surface,
    /// turned into RGB.
    read_pixels: unsafe extern "C" fn(Pointer, *const c_void) -> Pointer,
    convert_surface: unsafe extern "C" fn(Pointer, u32) -> Pointer,
    destroy_surface: unsafe extern "C" fn(Pointer),
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
            let title = CString::new(format!("{} - Rust - {} - Launcher", game.title, game.version)).unwrap();
            let primary: unsafe extern "C" fn() -> u32 = symbol(library, "SDL_GetPrimaryDisplay");
            let usable_bounds: unsafe extern "C" fn(u32, *mut [c_int; 4]) -> bool = symbol(library, "SDL_GetDisplayUsableBounds");
            let mut bounds = [0; 4];
            let usable = usable_bounds(primary(), &mut bounds).then_some((bounds[2], bounds[3]));
            let scale = window_scale(usable);
            let window = create_window(title.as_ptr(), WIDTH as c_int * scale, HEIGHT as c_int * scale, WINDOW_RESIZABLE);
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
                pick_file: symbol(library, "SDL_ShowOpenFileDialog"),
                set_scale: symbol(library, "SDL_SetRenderScale"),
                open_gamepad: symbol(library, "SDL_OpenGamepad"),
                delay: symbol(library, "SDL_Delay"),
                destroy_renderer: symbol(library, "SDL_DestroyRenderer"),
                destroy_window: symbol(library, "SDL_DestroyWindow"),
                read_pixels: symbol(library, "SDL_RenderReadPixels"),
                convert_surface: symbol(library, "SDL_ConvertSurface"),
                destroy_surface: symbol(library, "SDL_DestroySurface"),
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

    /// Writes at 3/4 size, at a place on the usual 320x240 screen.
    fn write_small(&self, x: f32, y: f32, text: &str, colour: (u8, u8, u8)) {
        unsafe { (self.set_scale)(self.renderer, SMALL, SMALL) };
        self.write(x / SMALL, y / SMALL, text, colour);
        unsafe { (self.set_scale)(self.renderer, 1.0, 1.0) };
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


/// How the launcher looks, from the game's `launcherTheme` (port.json): every
/// port gets one in the look of its game, drawn here in code, with no art of
/// the game's. `style=plain` is flat panels; `style=metal` is riveted metal
/// plates with bevelled edges and a striped header, for games with metal
/// menus such as Z; `style=leather` is dark crackled leather in a bevelled
/// metal frame, the title on a metal plaque and the rows as rimmed buttons
/// with a light at their left, lit on the chosen one, as in Incubation's
/// menus. Colours are six hex digits: `background`, `panel`, `selected`,
/// `border` (also the accent: the header stripes, the frame and the chosen
/// row's rim), `text`, `value`, `good` (also the lit light), `bad`, `dim` and
/// `warning` (the warning lines at the bottom; yellow unless set).
#[derive(Clone, Copy)]
pub struct Theme {
    pub style: Style,
    pub background: (u8, u8, u8),
    pub panel: (u8, u8, u8),
    pub selected: (u8, u8, u8),
    pub border: (u8, u8, u8),
    pub text: (u8, u8, u8),
    pub value: (u8, u8, u8),
    pub good: (u8, u8, u8),
    pub bad: (u8, u8, u8),
    pub dim: (u8, u8, u8),
    /// The warning lines at the bottom, such as files there twice.
    pub warning: (u8, u8, u8),
}

/// The launcher's drawing styles (see `Theme`).
#[derive(Clone, Copy, PartialEq)]
pub enum Style {
    Plain,
    Metal,
    Leather,
}

impl Theme {
    const PLAIN: Theme = Theme {
        style: Style::Plain,
        background: (14, 18, 16),
        panel: (26, 34, 30),
        selected: (46, 92, 64),
        border: (90, 150, 110),
        text: (225, 230, 220),
        value: (235, 205, 110),
        good: (110, 225, 120),
        bad: (245, 105, 90),
        dim: (120, 130, 125),
        warning: (240, 200, 70),
    };

    /// `name=value` pairs split by `;`; what's missing keeps the plain look.
    pub fn parse(text: &str) -> Theme {
        let mut theme = Theme::PLAIN;
        for pair in text.split(';') {
            let Some((name, value)) = pair.split_once('=') else { continue };
            let value = value.trim().trim_start_matches('#');
            let colour = (value.len() == 6)
                .then(|| u32::from_str_radix(value, 16).ok())
                .flatten()
                .map(|rgb| ((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8));
            match (name.trim(), colour) {
                ("style", _) => {
                    theme.style = match value.to_ascii_lowercase().as_str() {
                        "metal" => Style::Metal,
                        "leather" => Style::Leather,
                        _ => Style::Plain,
                    }
                }
                ("background", Some(c)) => theme.background = c,
                ("panel", Some(c)) => theme.panel = c,
                ("selected", Some(c)) => theme.selected = c,
                ("border", Some(c)) => theme.border = c,
                ("text", Some(c)) => theme.text = c,
                ("value", Some(c)) => theme.value = c,
                ("good", Some(c)) => theme.good = c,
                ("bad", Some(c)) => theme.bad = c,
                ("dim", Some(c)) => theme.dim = c,
                ("warning", Some(c)) => theme.warning = c,
                _ => {}
            }
        }
        theme
    }
}

static THEME: std::sync::OnceLock<Theme> = std::sync::OnceLock::new();

/// The game's theme, set when the launcher opens.
fn theme() -> Theme {
    *THEME.get().unwrap_or(&Theme::PLAIN)
}

/// A colour made lighter (`by` > 0) or darker, for bevelled edges.
fn shade((red, green, blue): (u8, u8, u8), by: i32) -> (u8, u8, u8) {
    let step = |c: u8| (c as i32 + by).clamp(0, 255) as u8;
    (step(red), step(green), step(blue))
}

impl Sdl {
    fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32, colour: (u8, u8, u8)) {
        self.rect([x1.min(x2), y1.min(y2), (x2 - x1).abs() + 1.0, (y2 - y1).abs() + 1.0], colour, true);
    }

    /// A rivet: a bright dot with a shadow, at its top left.
    fn rivet(&self, x: f32, y: f32) {
        let base = theme().panel;
        self.rect([x + 1.0, y + 1.0, 2.0, 2.0], shade(base, -40), true);
        self.rect([x, y, 2.0, 2.0], shade(base, 70), true);
    }

    /// The screen behind everything: flat, or metal plates with seams and rivets.
    fn background(&self) {
        let theme = theme();
        self.rect([0.0, 0.0, WIDTH, HEIGHT], theme.background, true);
        if theme.style == Style::Leather {
            self.leather();
            return;
        }
        if theme.style != Style::Metal {
            return;
        }
        // Plates 80 pixels square, their seams a dark line over a light one.
        let mut y = 0.0;
        while y < HEIGHT {
            self.line(0.0, y, WIDTH, y, shade(theme.background, -12));
            self.line(0.0, y + 1.0, WIDTH, y + 1.0, shade(theme.background, 14));
            y += 80.0;
        }
        let mut x = 0.0;
        while x < WIDTH {
            self.line(x, 0.0, x, HEIGHT, shade(theme.background, -12));
            self.line(x + 1.0, 0.0, x + 1.0, HEIGHT, shade(theme.background, 14));
            x += 80.0;
        }
    }

    /// Dark crackled leather in a bevelled metal frame with rivets, as behind
    /// Incubation's menus. The cracks are short dark lines with a lighter edge
    /// beside them, from a fixed sequence, so every frame looks the same.
    fn leather(&self) {
        let theme = theme();
        let mut seed: u32 = 0x1234_5678;
        let mut next = |range: u32| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) % range.max(1)
        };
        for _ in 0..1500 {
            let (mut x, mut y) = (next(WIDTH as u32) as f32, next(HEIGHT as u32) as f32);
            // A crack wanders a few pixels in one general direction.
            let (dx, dy) = ([-1.0, 1.0][next(2) as usize], [-1.0, 0.0, 1.0][next(3) as usize]);
            for _ in 0..(3 + next(7)) {
                self.rect([x, y, 1.0, 1.0], shade(theme.background, -14), true);
                self.rect([x, y + 1.0, 1.0, 1.0], shade(theme.background, 11), true);
                x += dx;
                y += if next(3) == 0 { dy } else { 0.0 };
            }
        }
        // The frame: four bevelled metal bars, lit at the top and left.
        const FRAME: f32 = 6.0;
        let metal = shade(theme.border, -75);
        for bar in [[0.0, 0.0, WIDTH, FRAME], [0.0, HEIGHT - FRAME, WIDTH, FRAME], [0.0, 0.0, FRAME, HEIGHT], [WIDTH - FRAME, 0.0, FRAME, HEIGHT]] {
            self.rect(bar, metal, true);
        }
        self.line(0.0, 0.0, WIDTH - 1.0, 0.0, shade(metal, 50));
        self.line(0.0, 0.0, 0.0, HEIGHT - 1.0, shade(metal, 35));
        self.line(FRAME, HEIGHT - FRAME, WIDTH - FRAME, HEIGHT - FRAME, shade(metal, 35));
        self.line(WIDTH - FRAME, FRAME, WIDTH - FRAME, HEIGHT - FRAME, shade(metal, 35));
        self.line(0.0, HEIGHT - 1.0, WIDTH - 1.0, HEIGHT - 1.0, shade(metal, -40));
        self.line(WIDTH - 1.0, 0.0, WIDTH - 1.0, HEIGHT - 1.0, shade(metal, -40));
        self.line(FRAME - 1.0, FRAME - 1.0, WIDTH - FRAME, FRAME - 1.0, shade(metal, -45));
        self.line(FRAME - 1.0, FRAME - 1.0, FRAME - 1.0, HEIGHT - FRAME, shade(metal, -45));
        for (x, y) in [(2.0, 2.0), (WIDTH - 4.0, 2.0), (2.0, HEIGHT - 4.0), (WIDTH - 4.0, HEIGHT - 4.0), (WIDTH / 2.0 - 1.0, HEIGHT - 4.0), (2.0, HEIGHT / 2.0), (WIDTH - 4.0, HEIGHT / 2.0)] {
            self.rect([x + 1.0, y + 1.0, 2.0, 2.0], shade(metal, -50), true);
            self.rect([x, y, 2.0, 2.0], shade(metal, 70), true);
        }
    }

    /// A row's light, left of it in the margin: lit on the chosen row, as the
    /// lights on Incubation's menu buttons.
    fn light(&self, x: f32, y: f32, lit: bool) {
        let theme = theme();
        let colour = if lit { theme.good } else { shade(theme.good, -130) };
        // A round light in a dark metal socket, 4 pixels across.
        self.rect([x, y - 1.0, 4.0, 6.0], shade(theme.border, -95), true);
        self.rect([x - 1.0, y, 6.0, 4.0], shade(theme.border, -95), true);
        self.rect([x + 1.0, y, 2.0, 4.0], colour, true);
        self.rect([x, y + 1.0, 4.0, 2.0], colour, true);
        if lit {
            self.rect([x + 1.0, y + 1.0, 1.0, 1.0], shade(colour, 90), true);
        }
    }

    /// The title's band at the top: plain text, or a metal bar with hazard
    /// stripes at both ends and the accent along its bottom.
    /// The line reads `TITLE - RUST - version` in every port.
    fn header(&self, title: &str, version: &str) {
        let theme = theme();
        let rest = format!(" - RUST - {version}");
        let size = header_size(title, version);
        let width = (title.len() + rest.len()) as f32 * CHARACTER * size;
        let left = (WIDTH - width) / 2.0;
        if theme.style == Style::Leather {
            // A dark metal plaque just wider than the title, as "Main Menu" sits on.
            let plaque = [(left - 10.0).max(8.0), 8.0, (width + 20.0).min(WIDTH - 16.0), 20.0];
            self.plate(plaque, shade(theme.panel, -6));
            self.rect([plaque[0] - 1.0, plaque[1] - 1.0, plaque[2] + 2.0, plaque[3] + 2.0], shade(theme.border, -20), false);
        }
        if theme.style == Style::Metal {
            let bar = [0.0, 4.0, WIDTH, 22.0];
            self.plate(bar, theme.panel);
            self.line(0.0, 25.0, WIDTH, 25.0, theme.border);
            // Diagonal stripes in the accent colour, up to 30 pixels at each
            // end, shorter when a long title needs the room.
            let reach = (left - 6.0).clamp(0.0, 30.0);
            for (left, right) in [(2.0f32, 2.0 + reach), (WIDTH - 2.0 - reach, WIDTH - 2.0)] {
                let mut x = left;
                while x < right {
                    for step in 0..18 {
                        let (px, py) = (x + step as f32 * 0.5, 23.0 - step as f32);
                        if px < right {
                            self.rect([px, py, 3.0, 1.0], theme.border, true);
                        }
                    }
                    x += 8.0;
                }
            }
        }
        if size < 1.0 {
            self.write_small(left, 13.0, title, theme.value);
            self.write_small(left + title.len() as f32 * CHARACTER * SMALL, 13.0, &rest, theme.dim);
        } else {
            self.write(left, 12.0, title, theme.value);
            self.write(left + title.len() as f32 * CHARACTER, 12.0, &rest, theme.dim);
        }
    }

    /// A raised metal plate: lit at the top and left, shadowed at the other
    /// sides, with a rivet in each corner when it's tall enough.
    fn plate(&self, [x, y, w, h]: [f32; 4], colour: (u8, u8, u8)) {
        self.rect([x, y, w, h], colour, true);
        self.line(x, y, x + w - 1.0, y, shade(colour, 45));
        self.line(x, y, x, y + h - 1.0, shade(colour, 30));
        self.line(x, y + h - 1.0, x + w - 1.0, y + h - 1.0, shade(colour, -35));
        self.line(x + w - 1.0, y, x + w - 1.0, y + h - 1.0, shade(colour, -35));
        // Rows are too low for rivets beside their text.
        if h >= 20.0 {
            for (rx, ry) in [(x + 3.0, y + 3.0), (x + w - 5.0, y + 3.0), (x + 3.0, y + h - 5.0), (x + w - 5.0, y + h - 5.0)] {
                self.rivet(rx, ry);
            }
        }
    }

    /// A row's panel, the chosen one in the chosen colour and framed.
    fn panel(&self, area: [f32; 4], chosen: bool) {
        let theme = theme();
        let colour = if chosen { theme.selected } else { theme.panel };
        if theme.style == Style::Leather {
            // A dark button with a metal rim, lit at the top, and a light at its left.
            let [x, y, w, h] = area;
            self.rect(area, colour, true);
            let rim = if chosen { theme.border } else { shade(theme.border, -60) };
            self.rect(area, rim, false);
            self.line(x + 1.0, y + 1.0, x + w - 2.0, y + 1.0, shade(colour, 22));
            self.line(x + 1.0, y + h - 2.0, x + w - 2.0, y + h - 2.0, shade(colour, -14));
            // Rounded ends: the corner pixels show the leather behind.
            for (cx, cy) in [(x, y), (x + w - 1.0, y), (x, y + h - 1.0), (x + w - 1.0, y + h - 1.0)] {
                self.rect([cx, cy, 1.0, 1.0], theme.background, true);
            }
            self.light(x - 7.0, y + (h - 4.0) / 2.0, chosen);
            return;
        }
        if theme.style == Style::Metal {
            self.plate(area, colour);
            if chosen {
                self.rect([area[0] - 1.0, area[1] - 1.0, area[2] + 2.0, area[3] + 2.0], theme.border, false);
            }
        } else {
            self.rect(area, colour, true);
            if chosen {
                self.rect(area, theme.border, false);
            }
        }
    }
}

/// The folder's state, in a line short enough for the screen.
fn folder_status(game: &Game, folder: &str) -> (String, bool) {
    if disc::is_image(std::path::Path::new(folder)) {
        return match loader::check_image(game, std::path::Path::new(folder)) {
            Ok(_) => match loader::version_in(game) {
                Some(name) => (format!("Found: {name}, disc image"), true),
                None => ("Disc image found, unpacked at start".to_string(), true),
            },
            Err(problem) => (problem, false),
        };
    }
    match loader::check_files(game, folder) {
        Ok(()) => match loader::version_in(game) {
            Some(name) => (format!("Found: {name}"), true),
            None => ("Game files found".to_string(), true),
        },
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

/// A path in two lines of the small font, broken after a separator. A path too long even for
/// that loses its start, which is the least telling part.
fn path_lines(path: &str) -> [String; 2] {
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= SMALL_LONGEST {
        return [path.to_string(), String::new()];
    }
    let chars = if chars.len() > 2 * SMALL_LONGEST {
        let mut short: Vec<char> = "...".chars().collect();
        short.extend(&chars[chars.len() - (2 * SMALL_LONGEST - 3)..]);
        short
    } else {
        chars
    };
    // The second line takes at most a line's worth; the break goes after the last separator
    // that leaves the first line short enough.
    let earliest = chars.len().saturating_sub(SMALL_LONGEST);
    let split = (earliest..=SMALL_LONGEST.min(chars.len())).rev().find(|&at| at > 0 && matches!(chars[at - 1], '/' | '\\')).unwrap_or(SMALL_LONGEST.min(chars.len()));
    [chars[..split].iter().collect(), chars[split..].iter().collect()]
}

fn absolute(folder: &str) -> String {
    let full = std::fs::canonicalize(folder).map_or(folder.to_string(), |path| path.display().to_string());
    // Windows gives the long-path form, which nobody types.
    full.strip_prefix(r"\\?\").map(str::to_string).unwrap_or(full)
}

/// The folder of the program the player started. An AppImage's program sits in a read-only
/// mount, so for one it's the folder holding the AppImage file.
pub(crate) fn program_folder() -> Option<std::path::PathBuf> {
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
            // Earlier releases, such as Incubation's, wrote the same key, always a folder. A
            // file there means the folder it's in.
            "folder" if !arguments_gave_folder && !value.trim().is_empty() => {
                let path = std::path::Path::new(value.trim());
                settings.folder = match path.parent() {
                    Some(parent) if path.is_file() && !disc::is_image(path) => parent.display().to_string(),
                    _ => value.trim().to_string(),
                };
            }
            "scale" => settings.scale = value.trim().parse().unwrap_or(settings.scale).clamp(FIRST_SCALE, LAST_SCALE),
            "fullscreen" => settings.fullscreen = value.trim() == "1",
            "drawing" if !std::env::args().any(|argument| argument.starts_with("--drawing=")) => settings.processor = value.trim() == "processor",
            "renderer" if !arguments_gave_renderer => settings.renderer = loader::find_choice(game.renderers, value.trim()).unwrap_or(settings.renderer),
            "language" if !arguments_gave_language => settings.language = loader::find_choice(game.languages, value.trim()).unwrap_or(settings.language),
            name => {
                if let Some(index) = game.switches.iter().position(|(switch, _, _)| *switch == name) {
                    settings.switches[index] = value.trim() == "1";
                }
                // `mod.FILE=N` is the value's index; an On/Off row's 0 and 1
                // read as before.
                let file = name.strip_prefix("mod.").unwrap_or_default();
                let arguments: Vec<String> = std::env::args().collect();
                if let Some(index) = game.mods.iter().position(|row| row.file == file) {
                    let row = &game.mods[index];
                    if loader::mod_from_arguments(row, &arguments).is_none() {
                        settings.mods[index] = row.find(value).unwrap_or(row.default);
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
    for (row, chosen) in game.mods.iter().zip(&settings.mods) {
        text.push_str(&format!("mod.{}={chosen}\n", row.file));
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
const FOLDER_HEIGHT: f32 = 46.0;
const ROWS_TOP: f32 = 86.0;
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
    let mods = |on_advanced: bool| (0..game.mods.len()).filter(move |index| game.mods[*index].advanced == on_advanced).map(Row::Mod);
    more.extend(mods(true));
    if advanced {
        more.push(Row::Back);
        return more;
    }
    // Only rows that change something: the 3D resolution is Glide's, so only
    // a game that draws with it gets the row.
    let glide = game.renderers.iter().any(|(_, label)| label.eq_ignore_ascii_case("Glide"));
    let mut rows = if glide { vec![Row::Folder, Row::File, Row::Scale, Row::Display] } else { vec![Row::Folder, Row::File, Row::Display] };
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
    sdl.background();
    sdl.header(&game.title.to_uppercase(), game.version);
    let all = rows(game, advanced);
    if advanced {
        let left = MARGIN + 4.0;
        sdl.write(left, FOLDER_TOP + 4.0, "Advanced settings", theme().text);
        sdl.write(left, FOLDER_TOP + 17.0, "The defaults suit most machines.", theme().dim);
        sdl.write(left, FOLDER_TOP + 30.0, "Change them if the game runs badly.", theme().dim);
    }
    for (index, row) in all.iter().enumerate() {
        let (top, height) = place(index, all.len(), advanced);
        let area = [MARGIN, top, WIDTH - 2.0 * MARGIN, height];
        sdl.panel(area, index == selected);
        let line = top + (height - CHARACTER) / 2.0;
        let left = MARGIN + 4.0;
        match row {
            Row::Folder => {
                sdl.write(left, top + 4.0, "Game files", theme().text);
                sdl.write_right(top + 4.0, "Choose folder...", theme().dim);
                let [first, second] = path_lines(&absolute(&settings.folder));
                sdl.write_small(left, top + 15.0, &first, theme().value);
                sdl.write_small(left, top + 23.0, &second, theme().value);
                sdl.write(left, top + 34.0, &shorten(&status.0), if status.1 { theme().good } else { theme().bad });
            }
            Row::File => {
                sdl.write(left, line, "Choose a disc image or game file", theme().text);
            }
            Row::Scale | Row::Drawing | Row::Display | Row::Renderer | Row::Language | Row::Switch(_) | Row::Mod(_) => {
                let label = match row {
                    Row::Scale => "3D resolution",
                    Row::Drawing => "3D drawn by",
                    Row::Display => "Display",
                    Row::Renderer => "Renderer",
                    Row::Language => "Text language",
                    Row::Switch(switch) => game.switches[*switch].2,
                    Row::Mod(index) => game.mods[*index].label,
                    _ => "",
                };
                if *row == Row::Scale && fixed_resolution(game, settings) {
                    // Greyed out, with no arrows: there's nothing to choose.
                    sdl.write(left, line, label, theme().dim);
                    sdl.write_right(line, "640 x 480", theme().dim);
                } else if *row == Row::Drawing && fixed_resolution(game, settings) {
                    // The game's own software renderer runs on the processor.
                    sdl.write(left, line, label, theme().dim);
                    sdl.write_right(line, "CPU", theme().dim);
                } else {
                    sdl.write(left, line, label, theme().text);
                    sdl.write_right(line, &value_text(game, *row, settings).unwrap_or_default(), theme().value);
                }
            }
            Row::Start => {
                let label = "Start game";
                let colour = if status.1 { theme().good } else { theme().dim };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, colour);
            }
            Row::Advanced | Row::Back | Row::Quit => {
                let label = match row {
                    Row::Advanced => "Advanced settings...",
                    Row::Back => "Back",
                    _ => "Quit",
                };
                sdl.write((WIDTH - label.len() as f32 * CHARACTER) / 2.0, line, label, theme().text);
            }
        }
    }
    // A warning takes the place of the two hint lines, which are the least needed.
    let lines = match warning {
        Some([first, second]) => [(first.as_str(), theme().warning), (second.as_str(), theme().warning)],
        None => {
            let hint = if fixed_resolution(game, settings) {
                "Software draws 3D at 640 x 480"
            } else if settings.processor && settings.scale >= 4 {
                "Higher 3D resolutions need a fast CPU"
            } else {
                "Drop a disc image or folder here"
            };
            [(hint, theme().dim), (if advanced { "Arrows/d-pad, Enter/A, Esc to go back" } else { "Arrows/d-pad, Enter/A, Esc to quit" }, theme().dim)]
        }
    };
    for (index, (text, colour)) in lines.iter().enumerate() {
        sdl.write((WIDTH - text.len() as f32 * CHARACTER) / 2.0, HEIGHT - 26.0 + index as f32 * 12.0, text, *colour);
    }
    if !TAKING_PICTURE.load(std::sync::atomic::Ordering::Relaxed) {
        unsafe { (sdl.present)(sdl.renderer) };
    }
}

/// Set while `snapshot` draws, so the drawing stays in the renderer to be read.
static TAKING_PICTURE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The `launcher:NAME` script step, for the README's picture of the launcher
/// (`release/launcher.png`): with a display (`./port recomp NAME "display
/// launcher:launcher"`), the launcher is drawn once and saved as NAME.png
/// beside the run's other pictures, and the scripted run goes on without it.
pub fn snapshot(game: &Game, settings: &Settings) {
    const PIXELFORMAT_RGB24: u32 = 0x1710_1803;
    let Ok(script) = std::env::var("RECOMP_SCRIPT") else { return };
    let Some(name) = script.split_whitespace().find_map(|step| step.strip_prefix("launcher:")) else { return };
    // Printed first, so that a run without a window can test the checks.
    let warning = loader::check_choices(game, settings).or_else(|| folder_warning(&settings.folder));
    if let Some([first, second]) = &warning {
        println!("launcher warning: {first} / {second}");
    }
    let _ = THEME.set(Theme::parse(game.theme));
    let Some(sdl) = Sdl::open(game) else {
        println!("no window for the launcher's picture");
        return;
    };
    let all = rows(game, false);
    let start = all.iter().position(|row| *row == Row::Start).unwrap_or(0);
    TAKING_PICTURE.store(true, std::sync::atomic::Ordering::Relaxed);
    draw(&sdl, game, settings, &folder_status(game, &settings.folder), &warning, start, false);
    TAKING_PICTURE.store(false, std::sync::atomic::Ordering::Relaxed);
    unsafe {
        let read = (sdl.read_pixels)(sdl.renderer, std::ptr::null());
        let rgb = if read.is_null() { read } else { (sdl.convert_surface)(read, PIXELFORMAT_RGB24) };
        if !rgb.is_null() {
            let base = rgb as *const u8;
            let (width, height, pitch) = (*(base.add(8) as *const i32), *(base.add(12) as *const i32), *(base.add(16) as *const i32));
            let pixels = *(base.add(24) as *const *const u8);
            let mut rows = Vec::with_capacity((width * height * 3) as usize);
            for y in 0..height {
                rows.extend_from_slice(std::slice::from_raw_parts(pixels.add((y * pitch) as usize), (width * 3) as usize));
            }
            let out = std::env::var("RECOMP_OUT").unwrap_or_else(|_| ".".to_string());
            crate::host::script::write_png(&format!("{out}/{name}.png"), width as u32, height as u32, &rows);
            println!("launcher saved as {name}.png");
            (sdl.destroy_surface)(rgb);
        }
        if !read.is_null() {
            (sdl.destroy_surface)(read);
        }
    }
    sdl.close();
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
        Row::Mod(index) => game.mods[index].values.get(settings.mods[index])?.to_string(),
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
            let start = dialog_start(&settings.folder);
            unsafe { (sdl.pick_folder)(folder_picked, std::ptr::null_mut(), sdl.window, start.as_ptr(), false) };
        }
        Row::File => {
            let start = dialog_start(&settings.folder);
            unsafe { (sdl.pick_file)(folder_picked, std::ptr::null_mut(), sdl.window, std::ptr::null(), 0, start.as_ptr(), false) };
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
        Row::Mod(index) => {
            let count = game.mods[index].values.len() as i32;
            settings.mods[index] = (settings.mods[index] as i32 + if step == 0 { 1 } else { step }).rem_euclid(count) as usize;
        }
        Row::Start if step == 0 && ready => return Some(true),
        Row::Quit if step == 0 => return Some(false),
        _ => {}
    }
    None
}

pub fn show(game: &Game, mut settings: Settings) -> Outcome {
    let gave_folder = std::env::args().skip(1).any(|argument| !argument.starts_with("--"));
    load(game, &mut settings, gave_folder);
    let _ = THEME.set(Theme::parse(game.theme));
    let Some(sdl) = Sdl::open(game) else { return Outcome::Unavailable(settings) };
    let mut advanced = false;
    let mut all = rows(game, advanced);
    let start_row = all.iter().position(|row| *row == Row::Start).unwrap_or(0);
    let mut status = folder_status(game, &settings.folder);
    // Files there twice, which walks the folder, and the mods' checks of the
    // choices, which are quick and follow every change.
    let mut warning = folder_warning(&settings.folder);
    let mut checked = loader::check_choices(game, &settings);
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
                        *PICKED.lock().unwrap() = Some(dropped);
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
            let done = change(&sdl, game, all[selected], step, &mut settings, status.1);
            checked = loader::check_choices(game, &settings);
            if let Some(done) = done {
                result = Some(done);
                break;
            }
        }
        if let Some(picked) = PICKED.lock().unwrap().take() {
            settings.folder = game_place(&picked);
            status = folder_status(game, &settings.folder);
            warning = folder_warning(&settings.folder);
            checked = loader::check_choices(game, &settings);
            // The folder's row is on the first page.
            advanced = false;
            all = rows(game, advanced);
            selected = if status.1 { start_row } else { 0 };
        }
        // A disc image is unpacked before the game starts, and the folder it went to is what's
        // remembered. If that fails the launcher stays, with the reason where the folder's
        // state is shown.
        if result == Some(true) && disc::is_image(std::path::Path::new(&settings.folder)) {
            match unpack_image(&sdl, game, &settings.folder) {
                Ok(folder) => settings.folder = folder,
                Err(problem) => {
                    status = (problem, false);
                    selected = 0;
                    result = None;
                }
            }
        }
        if let Some(done) = result {
            break done;
        }
        // A mod's check knows the game, so it says more than files there twice.
        draw(&sdl, game, &settings, &status, &checked.clone().or_else(|| warning.clone()), selected, advanced);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_fits_the_screen() {
        // 1024x768 with a taskbar: 640x480.
        assert_eq!(window_scale(Some((1024, 728))), 2);
        // With no taskbar, 960x720 and its title bar just fit.
        assert_eq!(window_scale(Some((1024, 768))), 3);
        assert_eq!(window_scale(Some((1920, 1040))), 4);
        assert_eq!(window_scale(Some((3840, 2100))), 4);
        assert_eq!(window_scale(Some((800, 560))), 2);
        assert_eq!(window_scale(Some((640, 400))), 1);
        assert_eq!(window_scale(None), FALLBACK_WINDOW_SCALE);
        assert_eq!(window_scale(Some((0, 0))), FALLBACK_WINDOW_SCALE);
    }

    const FRAME_RATE: loader::ModRow = loader::ModRow { file: "frame_limiter", label: "Frame rate", values: &["Original", "30", "60"], default: 1, advanced: false };
    const ON_OFF: loader::ModRow = loader::ModRow { file: "give_up", label: "Ask", values: &["Off", "On"], default: 1, advanced: false };

    #[test]
    fn mod_values_by_name_or_number() {
        assert_eq!(FRAME_RATE.find("60"), Some(2));
        assert_eq!(FRAME_RATE.find("original"), Some(0));
        // A settings.ini from before choice rows: 1 was "on".
        assert_eq!(FRAME_RATE.find("1"), Some(1));
        assert_eq!(FRAME_RATE.find("3"), None);
        assert_eq!(ON_OFF.find("0"), Some(0));
        assert_eq!(ON_OFF.find("on"), Some(1));
    }

    #[test]
    fn mod_values_from_the_command_line() {
        let arguments = |list: &[&str]| list.iter().map(|argument| argument.to_string()).collect::<Vec<_>>();
        assert_eq!(loader::mod_from_arguments(&FRAME_RATE, &arguments(&["--mod=frame_limiter:60"])), Some(2));
        assert_eq!(loader::mod_from_arguments(&FRAME_RATE, &arguments(&["--mod-off=frame_limiter"])), Some(0));
        assert_eq!(loader::mod_from_arguments(&FRAME_RATE, &arguments(&["--mod=give_up:off"])), None);
        assert_eq!(loader::mod_from_arguments(&ON_OFF, &arguments(&["--mod=give_up:off"])), Some(0));
        assert_eq!(loader::mod_from_arguments(&ON_OFF, &arguments(&["--play"])), None);
    }

    #[test]
    fn header_is_never_cut_off() {
        assert_eq!(header_size("INCUBATION", "0.6-beta"), 1.0);
        // 45 characters: too many for full size, so 3/4.
        assert_eq!(header_size("DRAKAN: ORDER OF THE FLAME", "0.1-alpha"), SMALL);
        // The longest line the recompiler lets through still fits at 3/4.
        let longest = "X".repeat(52 - " - RUST - ".len());
        let size = header_size(&longest, "");
        assert!(52.0 * CHARACTER * size <= HEADER_ROOM);
    }
}
