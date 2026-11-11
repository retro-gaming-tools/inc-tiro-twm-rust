//! The window, through SDL3, which is loaded when the game starts. Without it
//! the game still runs, with no window, for tests.

use crate::cpu::Cpu;
use crate::host::gamepad;
use crate::host::gl::{self, Cover, Gl};
use crate::sys;
use std::collections::VecDeque;
use std::ffi::{c_char, c_int, c_void, CString};
use std::time::{Duration, Instant};

mod dialog;

const INIT_VIDEO: u32 = 0x20;
const INIT_GAMEPAD: u32 = 0x2000;
const WINDOW_OPENGL: u64 = 0x2;
const WINDOW_RESIZABLE: u64 = 0x20;
const GL_CONTEXT_MAJOR_VERSION: c_int = 17;
const GL_CONTEXT_MINOR_VERSION: c_int = 18;
const GL_CONTEXT_PROFILE_MASK: c_int = 20;
const GL_CONTEXT_PROFILE_CORE: c_int = 1;
/// Where an SDL_Surface keeps its pixels.
const SURFACE_PIXELS: usize = 24;
const LOGICAL_LETTERBOX: c_int = 2;
const PIXELFORMAT_RGB565: u32 = 0x1515_1002;
const PIXELFORMAT_RGBA32: u32 = 0x1676_2004;
const PIXELFORMAT_XRGB8888: u32 = 0x1616_1804;
const ICON_SIZE: c_int = 64;
const TEXTUREACCESS_STREAMING: c_int = 1;
const EVENT_QUIT: u32 = 0x100;
const EVENT_KEY_DOWN: u32 = 0x300;
const EVENT_KEY_UP: u32 = 0x301;
const EVENT_MOUSE_MOTION: u32 = 0x400;
const EVENT_MOUSE_BUTTON_DOWN: u32 = 0x401;
const EVENT_MOUSE_BUTTON_UP: u32 = 0x402;
const EVENT_GAMEPAD_AXIS: u32 = 0x650;
const EVENT_GAMEPAD_BUTTON_DOWN: u32 = 0x651;
const EVENT_GAMEPAD_BUTTON_UP: u32 = 0x652;
const EVENT_GAMEPAD_ADDED: u32 = 0x653;
const EVENT_GAMEPAD_REMOVED: u32 = 0x654;
const WINDOW_SCALE: c_int = 2;
const WIDTH: c_int = crate::host::GAME_WIDTH as c_int;
const HEIGHT: c_int = crate::host::GAME_HEIGHT as c_int;
/// Longer than a frame at 30 a second, so a game that flips is shown only at its flips.
const NOT_FLIPPING: Duration = Duration::from_millis(50);
/// How often a game that draws to the front buffer without flipping is shown.
const REFRESH: Duration = Duration::from_micros(16_667);
/// The refresh assumed when the display doesn't say, or there's no window.
pub const DEFAULT_REFRESH: Duration = REFRESH;
const MK_LBUTTON: u32 = 1;
const MK_RBUTTON: u32 = 2;
const WM_KEYDOWN: u32 = 0x100;
const WM_KEYUP: u32 = 0x101;
const WM_CHAR: u32 = 0x102;
const WM_MOUSEMOVE: u32 = 0x200;
const WM_LBUTTONDOWN: u32 = 0x201;
const WM_LBUTTONUP: u32 = 0x202;
const WM_RBUTTONDOWN: u32 = 0x204;
const WM_RBUTTONUP: u32 = 0x205;
const KEY_F10: u32 = 0x4000_0043;
const KEY_F11: u32 = 0x4000_0044;
const STATS_FRAMES: usize = 120;
const BLEND_NONE: u32 = 0;
const BLEND_ALPHA: u32 = 1;
const MESSAGEBOX_ERROR: u32 = 0x10;

/// An SDL_Event is a union of at most 128 bytes.
#[repr(C, align(8))]
pub(crate) struct Event(pub(crate) [u8; 128]);

impl Event {
    pub(crate) fn u32_at(&self, at: usize) -> u32 {
        u32::from_le_bytes(self.0[at..at + 4].try_into().unwrap())
    }
    pub(crate) fn f32_at(&self, at: usize) -> f32 {
        f32::from_bits(self.u32_at(at))
    }
    fn set_f32(&mut self, at: usize, value: f32) {
        self.0[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
}

/// The window drawn with OpenGL, which Glide then draws with too. Pictures
/// reach the window as textures, and the frame counter and the dialog are
/// drawn by SDL into a surface that goes over them.
struct GlScreen {
    gl: &'static Gl,
    cover: Cover,
    /// For pictures that come as pixels: DirectDraw's, or the rasterizer's.
    frame: u32,
    /// The surface's pixels on the card.
    overlay: u32,
    /// 640x480, drawn to through `Video::renderer`.
    surface: Pointer,
    /// The texture last shown, which the dialog dims.
    shown: u32,
    /// Whether the surface went over it.
    overlay_shown: bool,
    swap: unsafe extern "C" fn(Pointer) -> bool,
    size: unsafe extern "C" fn(Pointer, *mut c_int, *mut c_int) -> bool,
    size_in_pixels: unsafe extern "C" fn(Pointer, *mut c_int, *mut c_int) -> bool,
    flush: unsafe extern "C" fn(Pointer) -> bool,
}

/// What to put in the window.
enum Picture {
    /// A texture, with the game's top row first.
    Texture(u32),
    /// Pixels: their size, OpenGL format and type.
    Pixels(*const c_void, c_int, c_int, u32, u32),
    /// What was shown last.
    Last,
}

impl GlScreen {
    /// Makes the window's OpenGL context and what drawing to it needs. None,
    /// with the reason printed, if this machine's OpenGL won't do. Software
    /// OpenGL is turned down unless `forced`, since the rasterizer is faster.
    unsafe fn open(library: Pointer, window: Pointer, forced: bool) -> Option<(GlScreen, Pointer, bool)> {
        let create_context: unsafe extern "C" fn(Pointer) -> Pointer = symbol(library, "SDL_GL_CreateContext");
        let destroy_context: unsafe extern "C" fn(Pointer) -> bool = symbol(library, "SDL_GL_DestroyContext");
        let address: unsafe extern "C" fn(*const c_char) -> Pointer = symbol(library, "SDL_GL_GetProcAddress");
        let set_interval: unsafe extern "C" fn(c_int) -> bool = symbol(library, "SDL_GL_SetSwapInterval");
        let create_surface: unsafe extern "C" fn(c_int, c_int, u32) -> Pointer = symbol(library, "SDL_CreateSurface");
        let software_renderer: unsafe extern "C" fn(Pointer) -> Pointer = symbol(library, "SDL_CreateSoftwareRenderer");
        let context = create_context(window);
        if context.is_null() {
            println!("no OpenGL 3.3 here, so the processor draws");
            return None;
        }
        let made = (|| {
            let gl = Gl::load(address)?;
            let (renderer, version) = (gl.string(gl::RENDERER), gl.string(gl::VERSION));
            let lower = renderer.to_lowercase();
            if !forced && ["llvmpipe", "softpipe", "swrast", "software"].iter().any(|name| lower.contains(name)) {
                println!("OpenGL here is software ({renderer}), so the processor draws");
                return None;
            }
            println!("OpenGL {version} on {renderer}");
            let cover = match Cover::new(&gl) {
                Ok(cover) => cover,
                Err(problem) => {
                    println!("OpenGL can't be used, so the processor draws: {problem}");
                    return None;
                }
            };
            let surface = create_surface(WIDTH, HEIGHT, PIXELFORMAT_RGBA32);
            let renderer = if surface.is_null() { surface } else { software_renderer(surface) };
            if renderer.is_null() {
                return None;
            }
            let (frame, overlay) = (gl.texture(gl::LINEAR), gl.texture(gl::NEAREST));
            let screen = GlScreen {
                gl: Box::leak(Box::new(gl)),
                cover,
                frame,
                overlay,
                surface,
                shown: 0,
                overlay_shown: false,
                swap: symbol(library, "SDL_GL_SwapWindow"),
                size: symbol(library, "SDL_GetWindowSize"),
                size_in_pixels: symbol(library, "SDL_GetWindowSizeInPixels"),
                flush: symbol(library, "SDL_FlushRenderer"),
            };
            Some((screen, renderer))
        })();
        let Some((screen, renderer)) = made else {
            destroy_context(context);
            return None;
        };
        let vsync = if std::env::var_os("RECOMP_NO_VSYNC").is_none() {
            set_interval(1)
        } else {
            set_interval(0);
            false
        };
        Some((screen, renderer, vsync))
    }

    /// Where the picture is in a window of this size: x, y, width and height.
    fn area(&self, window: Pointer, in_pixels: bool) -> (c_int, c_int, c_int, c_int) {
        let (mut width, mut height) = (0, 0);
        unsafe { (if in_pixels { self.size_in_pixels } else { self.size })(window, &mut width, &mut height) };
        gl::letterbox((width, height), (WIDTH, HEIGHT))
    }
}

type Pointer = *mut c_void;

pub(crate) unsafe fn symbol<T: Copy>(library: Pointer, name: &str) -> T {
    let found = sys::symbol(library, name);
    assert!(!found.is_null(), "this SDL3 has no {name}");
    std::mem::transmute_copy(&found)
}

pub struct Video {
    window: Pointer,
    /// Draws to the window, or with OpenGL to the surface that goes over the picture.
    renderer: Pointer,
    texture: Pointer,
    gl: Option<GlScreen>,
    last_present: Instant,
    /// The last frame shown: whether it was Glide's, and its swap or flip count.
    shown_frame: Option<(bool, u64)>,
    stats: bool,
    frame_times: VecDeque<f32>,
    present_time: f32,
    fullscreen: bool,
    /// Whether presenting waits for the display's refresh.
    vsync: bool,
    display_for_window: unsafe extern "C" fn(Pointer) -> u32,
    display_mode: unsafe extern "C" fn(u32) -> *const DisplayMode,
    /// Mouse buttons held, as SDL numbers them, one bit each.
    buttons: u32,
    /// True while the game holds the cursor in place during a drag, so the
    /// pointer follows the mouse's movement rather than its position.
    relative: bool,
    /// Movement below a game pixel, kept for the next event.
    remainder: (f32, f32),
    set_relative: unsafe extern "C" fn(Pointer, bool) -> bool,
    to_window: unsafe extern "C" fn(Pointer, f32, f32, *mut f32, *mut f32) -> bool,
    warp_mouse: unsafe extern "C" fn(Pointer, f32, f32),
    create_texture: unsafe extern "C" fn(Pointer, u32, c_int, c_int, c_int) -> Pointer,
    update_texture: unsafe extern "C" fn(Pointer, *const c_void, *const c_void, c_int) -> bool,
    render_clear: unsafe extern "C" fn(Pointer) -> bool,
    render_texture: unsafe extern "C" fn(Pointer, Pointer, *const c_void, *const c_void) -> bool,
    render_present: unsafe extern "C" fn(Pointer) -> bool,
    poll_event: unsafe extern "C" fn(*mut Event) -> bool,
    convert_coordinates: unsafe extern "C" fn(Pointer, *mut Event) -> bool,
    set_fullscreen: unsafe extern "C" fn(Pointer, bool) -> bool,
    open_gamepad: unsafe extern "C" fn(u32) -> Pointer,
    set_blend: unsafe extern "C" fn(Pointer, u32) -> bool,
    set_color: unsafe extern "C" fn(Pointer, u8, u8, u8, u8) -> bool,
    fill_rect: unsafe extern "C" fn(Pointer, *const f32) -> bool,
    draw_line: unsafe extern "C" fn(Pointer, f32, f32, f32, f32) -> bool,
    debug_text: unsafe extern "C" fn(Pointer, f32, f32, *const c_char) -> bool,
    set_scale: unsafe extern "C" fn(Pointer, f32, f32) -> bool,
    show_cursor: unsafe extern "C" fn() -> bool,
    hide_cursor: unsafe extern "C" fn() -> bool,
}

impl Video {
    /// Opens the window, or returns None if SDL3 isn't there or has no display.
    pub fn open(title: &str, icon: &'static [u8]) -> Option<Video> {
        if std::env::var_os("RECOMP_HEADLESS").is_some() {
            return None;
        }
        unsafe {
            let library = sys::open_sdl();
            if library.is_null() {
                return None;
            }
            let init: unsafe extern "C" fn(u32) -> bool = symbol(library, "SDL_Init");
            let create_window: unsafe extern "C" fn(*const c_char, c_int, c_int, u64) -> Pointer = symbol(library, "SDL_CreateWindow");
            let create_renderer: unsafe extern "C" fn(Pointer, *const c_char) -> Pointer = symbol(library, "SDL_CreateRenderer");
            let set_logical: unsafe extern "C" fn(Pointer, c_int, c_int, c_int) -> bool = symbol(library, "SDL_SetRenderLogicalPresentation");
            let create_texture: unsafe extern "C" fn(Pointer, u32, c_int, c_int, c_int) -> Pointer = symbol(library, "SDL_CreateTexture");
            let hide_cursor: unsafe extern "C" fn() -> bool = symbol(library, "SDL_HideCursor");
            if !init(INIT_VIDEO) {
                return None;
            }
            init(INIT_GAMEPAD); // The game runs without one.
            let name = CString::new(title).unwrap();
            // The window is drawn with OpenGL, so that Glide can draw on the
            // graphics card, unless the player chose the processor with
            // `RECOMP_GLIDE=cpu` or OpenGL 3.3 isn't to be had. `gpu` takes
            // software OpenGL too, for tests.
            let choice = std::env::var("RECOMP_GLIDE").unwrap_or_default();
            let mut window = std::ptr::null_mut();
            let mut screen = None;
            if choice != "cpu" {
                let set_attribute: unsafe extern "C" fn(c_int, c_int) -> bool = symbol(library, "SDL_GL_SetAttribute");
                let destroy_window: unsafe extern "C" fn(Pointer) = symbol(library, "SDL_DestroyWindow");
                set_attribute(GL_CONTEXT_MAJOR_VERSION, 3);
                set_attribute(GL_CONTEXT_MINOR_VERSION, 3);
                set_attribute(GL_CONTEXT_PROFILE_MASK, GL_CONTEXT_PROFILE_CORE);
                window = create_window(name.as_ptr(), WIDTH * WINDOW_SCALE, HEIGHT * WINDOW_SCALE, WINDOW_RESIZABLE | WINDOW_OPENGL);
                if !window.is_null() {
                    screen = GlScreen::open(library, window, choice == "gpu");
                    if screen.is_none() {
                        destroy_window(window);
                        window = std::ptr::null_mut();
                    }
                }
            }
            if window.is_null() {
                window = create_window(name.as_ptr(), WIDTH * WINDOW_SCALE, HEIGHT * WINDOW_SCALE, WINDOW_RESIZABLE);
            }
            if window.is_null() {
                return None;
            }
            if icon.len() == ICON_SIZE as usize * ICON_SIZE as usize * 4 {
                let create_surface: unsafe extern "C" fn(c_int, c_int, u32, *const u8, c_int) -> Pointer = symbol(library, "SDL_CreateSurfaceFrom");
                let set_icon: unsafe extern "C" fn(Pointer, Pointer) -> bool = symbol(library, "SDL_SetWindowIcon");
                let surface = create_surface(ICON_SIZE, ICON_SIZE, PIXELFORMAT_RGBA32, icon.as_ptr(), ICON_SIZE * 4);
                if !surface.is_null() {
                    set_icon(window, surface);
                }
            }
            // Frames are shown at a refresh, so none is torn, and each stays
            // on screen for whole refreshes. `ddraw::wait_for_refresh` paces them.
            let (gl, renderer, texture, vsync) = match screen {
                Some((screen, renderer, vsync)) => (Some(screen), renderer, std::ptr::null_mut(), vsync),
                None => {
                    let renderer = create_renderer(window, std::ptr::null());
                    if renderer.is_null() {
                        return None;
                    }
                    let set_vsync: unsafe extern "C" fn(Pointer, c_int) -> bool = symbol(library, "SDL_SetRenderVSync");
                    let vsync = std::env::var_os("RECOMP_NO_VSYNC").is_none() && set_vsync(renderer, 1);
                    set_logical(renderer, WIDTH, HEIGHT, LOGICAL_LETTERBOX);
                    let texture = create_texture(renderer, PIXELFORMAT_RGB565, TEXTUREACCESS_STREAMING, WIDTH, HEIGHT);
                    (None, renderer, texture, vsync)
                }
            };
            hide_cursor();
            let set_fullscreen: unsafe extern "C" fn(Pointer, bool) -> bool = symbol(library, "SDL_SetWindowFullscreen");
            // The launcher, or --fullscreen, can ask for it.
            let fullscreen = std::env::var_os("RECOMP_FULLSCREEN").is_some_and(|value| value == "1");
            if fullscreen {
                set_fullscreen(window, true);
            }
            Some(Video {
                window,
                renderer,
                texture,
                gl,
                last_present: Instant::now(),
                shown_frame: None,
                stats: false,
                frame_times: VecDeque::with_capacity(STATS_FRAMES),
                present_time: 0.0,
                fullscreen,
                vsync,
                display_for_window: symbol(library, "SDL_GetDisplayForWindow"),
                display_mode: symbol(library, "SDL_GetCurrentDisplayMode"),
                buttons: 0,
                relative: false,
                remainder: (0.0, 0.0),
                set_relative: symbol(library, "SDL_SetWindowRelativeMouseMode"),
                to_window: symbol(library, "SDL_RenderCoordinatesToWindow"),
                warp_mouse: symbol(library, "SDL_WarpMouseInWindow"),
                create_texture,
                update_texture: symbol(library, "SDL_UpdateTexture"),
                render_clear: symbol(library, "SDL_RenderClear"),
                render_texture: symbol(library, "SDL_RenderTexture"),
                render_present: symbol(library, "SDL_RenderPresent"),
                poll_event: symbol(library, "SDL_PollEvent"),
                convert_coordinates: symbol(library, "SDL_ConvertEventToRenderCoordinates"),
                set_fullscreen,
                open_gamepad: symbol(library, "SDL_OpenGamepad"),
                set_blend: symbol(library, "SDL_SetRenderDrawBlendMode"),
                set_color: symbol(library, "SDL_SetRenderDrawColor"),
                fill_rect: symbol(library, "SDL_RenderFillRect"),
                draw_line: symbol(library, "SDL_RenderLine"),
                debug_text: symbol(library, "SDL_RenderDebugText"),
                set_scale: symbol(library, "SDL_SetRenderScale"),
                show_cursor: symbol(library, "SDL_ShowCursor"),
                hide_cursor: symbol(library, "SDL_HideCursor"),
            })
        }
    }
}

/// Frame rate and a graph of recent frames, over the picture: each column is a
/// frame, as tall as it took. The lines mark 60 and 30 frames a second.
/// The start of SDL_DisplayMode, up to the refresh rate.
#[repr(C)]
pub struct DisplayMode {
    display: u32,
    format: u32,
    width: c_int,
    height: c_int,
    pixel_density: f32,
    refresh_rate: f32,
}

impl Video {
    /// How long the display's refresh takes, where the window is now.
    pub fn refresh(&self) -> Duration {
        const SLOWEST: f32 = 24.0;
        let mode = unsafe { (self.display_mode)((self.display_for_window)(self.window)) };
        match unsafe { mode.as_ref() } {
            Some(mode) if mode.refresh_rate >= SLOWEST => Duration::from_secs_f32(1.0 / mode.refresh_rate),
            _ => DEFAULT_REFRESH,
        }
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    /// OpenGL, when the window is drawn with it.
    pub fn gl(&self) -> Option<&'static Gl> {
        self.gl.as_ref().map(|screen| screen.gl)
    }

    /// Turns a mouse event's window position into the game's.
    fn convert(&self, event: &mut Event) {
        let Some(screen) = self.gl.as_ref() else {
            unsafe { (self.convert_coordinates)(self.renderer, event) };
            return;
        };
        let kind = event.u32_at(0);
        if !matches!(kind, EVENT_MOUSE_MOTION | EVENT_MOUSE_BUTTON_DOWN | EVENT_MOUSE_BUTTON_UP) {
            return;
        }
        let (x, y, width, height) = screen.area(self.window, false);
        let (across, down) = (WIDTH as f32 / width as f32, HEIGHT as f32 / height as f32);
        event.set_f32(28, (event.f32_at(28) - x as f32) * across);
        event.set_f32(32, (event.f32_at(32) - y as f32) * down);
        if kind == EVENT_MOUSE_MOTION {
            event.set_f32(36, event.f32_at(36) * across);
            event.set_f32(40, event.f32_at(40) * down);
        }
    }

    /// Where a game position is in the window.
    fn game_to_window(&self, x: f32, y: f32) -> (f32, f32) {
        let Some(screen) = self.gl.as_ref() else {
            let (mut window_x, mut window_y) = (0.0, 0.0);
            unsafe { (self.to_window)(self.renderer, x, y, &mut window_x, &mut window_y) };
            return (window_x, window_y);
        };
        let (left, top, width, height) = screen.area(self.window, false);
        (left as f32 + x * width as f32 / WIDTH as f32, top as f32 + y * height as f32 / HEIGHT as f32)
    }

    /// Empties the surface that goes over the picture, for new drawing.
    fn clear_overlay(&self) {
        unsafe {
            (self.set_color)(self.renderer, 0, 0, 0, 0);
            (self.render_clear)(self.renderer);
        }
    }

    /// Puts a picture in the window with OpenGL, as large as fits, and over
    /// it what was drawn to the surface since `clear_overlay`.
    fn present_gl(&mut self, picture: Picture, overlay: bool) {
        self.draw_gl(picture, overlay);
        if let Some(screen) = self.gl.as_ref() {
            unsafe { (screen.swap)(self.window) };
        }
    }

    /// What the window shows, as RGB with the top row first, for a script's
    /// `window` step. Only with OpenGL: it's drawn again and read back.
    pub fn window_picture(&mut self) -> Option<(u32, u32, Vec<u8>)> {
        let overlay = self.gl.as_ref()?.overlay_shown;
        self.draw_gl(Picture::Last, overlay);
        let screen = self.gl.as_ref()?;
        let (mut width, mut height) = (0, 0);
        let mut pixels;
        unsafe {
            (screen.size_in_pixels)(self.window, &mut width, &mut height);
            pixels = vec![0u8; (width * height * 3) as usize];
            (screen.gl.pixel_store)(gl::PACK_ALIGNMENT, 1);
            (screen.gl.read_pixels)(0, 0, width, height, gl::RGB, gl::UNSIGNED_BYTE, pixels.as_mut_ptr() as *mut c_void);
        }
        // The window's rows come bottom first.
        let rows: Vec<u8> = pixels.chunks((width * 3) as usize).rev().flatten().copied().collect();
        Some((width as u32, height as u32, rows))
    }

    fn draw_gl(&mut self, picture: Picture, overlay: bool) {
        let Some(screen) = self.gl.as_mut() else { return };
        let gl = screen.gl;
        screen.overlay_shown = overlay;
        unsafe {
            match picture {
                Picture::Texture(texture) => screen.shown = texture,
                Picture::Pixels(pixels, width, height, format, kind) => {
                    (gl.bind_texture)(gl::TEXTURE_2D, screen.frame);
                    (gl.tex_image)(gl::TEXTURE_2D, 0, gl::RGB8 as i32, width, height, 0, format, kind, pixels);
                    screen.shown = screen.frame;
                }
                Picture::Last => {}
            }
            let (mut width, mut height) = (0, 0);
            (screen.size_in_pixels)(self.window, &mut width, &mut height);
            (gl.bind_framebuffer)(gl::FRAMEBUFFER, 0);
            (gl.viewport)(0, 0, width, height);
            (gl.disable)(gl::SCISSOR_TEST);
            (gl.disable)(gl::BLEND);
            (gl.color_mask)(1, 1, 1, 1);
            (gl.clear_color)(0.0, 0.0, 0.0, 1.0);
            (gl.clear)(gl::COLOR_BUFFER_BIT);
            let (x, y, width, height) = screen.area(self.window, true);
            (gl.viewport)(x, y, width, height);
            if screen.shown != 0 {
                screen.cover.draw(gl, screen.shown, true, false);
            }
            if overlay {
                (screen.flush)(self.renderer);
                let pixels = *((screen.surface as *const u8).add(SURFACE_PIXELS) as *const *const c_void);
                (gl.bind_texture)(gl::TEXTURE_2D, screen.overlay);
                (gl.tex_image)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, WIDTH, HEIGHT, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixels);
                // SDL blended onto nothing, so the colours are already
                // multiplied by their alpha.
                (gl.enable)(gl::BLEND);
                (gl.blend_func)(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
                screen.cover.draw(gl, screen.overlay, true, false);
                (gl.disable)(gl::BLEND);
            }
        }
    }

    /// When the last frame was put on screen.
    pub fn last_present(&self) -> Instant {
        self.last_present
    }

    pub fn record_frame(&mut self, milliseconds: f32) {
        if self.frame_times.len() == STATS_FRAMES {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(milliseconds);
    }

    fn draw_stats(&self) {
        const LEFT: f32 = 4.0;
        const TOP: f32 = 4.0;
        const GRAPH_TOP: f32 = 24.0;
        const SECOND_LINE: f32 = 11.0;
        const GRAPH_HEIGHT: f32 = 50.0; // A pixel is a millisecond.
        const THIRTY_FPS: f32 = 1000.0 / 30.0;
        const GOOD: f32 = THIRTY_FPS + 3.0; // Room for the timer's jitter.
        const SLOW: f32 = 50.0;
        let width = STATS_FRAMES as f32;
        let bottom = TOP + GRAPH_TOP + GRAPH_HEIGHT;
        let count = self.frame_times.len().max(1) as f32;
        let average = self.frame_times.iter().sum::<f32>() / count;
        let longest = self.frame_times.iter().fold(0.0f32, |a, b| a.max(*b));
        let text = format!("{:.0} FPS {:.1} ms max {:.0}", 1000.0 / average.max(0.1), average, longest);
        let text = CString::new(text).unwrap();
        let shown = CString::new(format!("display {:.1} ms", self.present_time)).unwrap();
        unsafe {
            (self.set_blend)(self.renderer, BLEND_ALPHA);
            (self.set_color)(self.renderer, 0, 0, 0, 225);
            let panel = [LEFT - 2.0, TOP - 2.0, width + 56.0, GRAPH_TOP + GRAPH_HEIGHT + 4.0];
            (self.fill_rect)(self.renderer, panel.as_ptr());
            for (column, milliseconds) in self.frame_times.iter().enumerate() {
                let (red, green) = if *milliseconds <= GOOD { (60, 220) } else if *milliseconds <= SLOW { (240, 200) } else { (240, 50) };
                (self.set_color)(self.renderer, red, green, 60, 255);
                let x = LEFT + column as f32;
                (self.draw_line)(self.renderer, x, bottom, x, bottom - milliseconds.min(GRAPH_HEIGHT));
            }
            (self.set_color)(self.renderer, 255, 255, 255, 110);
            for target in [1000.0 / 60.0, THIRTY_FPS] {
                (self.draw_line)(self.renderer, LEFT, bottom - target, LEFT + width, bottom - target);
            }
            (self.set_color)(self.renderer, 255, 255, 255, 255);
            (self.debug_text)(self.renderer, LEFT, TOP, text.as_ptr());
            (self.debug_text)(self.renderer, LEFT, TOP + SECOND_LINE, shown.as_ptr());
            (self.set_blend)(self.renderer, BLEND_NONE);
        }
    }
}

/// The game's `SetCursorPos`. Games that scroll or turn on a drag put the cursor
/// back each frame and read how far it went. Only the game's pointer moves, so
/// the next mouse event would report the real cursor, still far away, and the
/// view would jump. While a button is held, the pointer follows the mouse's
/// movement instead.
pub fn set_cursor(cpu: &mut Cpu, x: u32, y: u32) {
    cpu.host.pointer = (x.min(639), y.min(479));
    if let Some(video) = cpu.host.video.as_mut() {
        if video.buttons != 0 && !video.relative {
            video.relative = true;
            video.remainder = (0.0, 0.0);
            unsafe { (video.set_relative)(video.window, true) };
        }
    }
}

/// Shows or hides the frame counter.
pub fn toggle_stats(cpu: &mut Cpu) {
    if let Some(video) = cpu.host.video.as_mut() {
        video.stats = !video.stats;
    }
}

impl Video {
    /// Replaces the 640x480 texture with one for Glide's larger picture.
    pub fn use_glide_size(&mut self, width: usize, height: usize) {
        unsafe {
            let texture = (self.create_texture)(self.renderer, PIXELFORMAT_XRGB8888, TEXTUREACCESS_STREAMING, width as c_int, height as c_int);
            if !texture.is_null() {
                self.texture = texture;
            }
        }
    }
}

/// Shows an error to the player, in a box if SDL3 can, and always on the console.
pub fn show_error(title: &str, text: &str) {
    eprintln!("{text}");
    unsafe {
        let library = sys::open_sdl();
        if library.is_null() {
            return;
        }
        let show: unsafe extern "C" fn(u32, *const c_char, *const c_char, Pointer) -> bool = symbol(library, "SDL_ShowSimpleMessageBox");
        let (title, text) = (CString::new(title).unwrap(), CString::new(text).unwrap());
        show(MESSAGEBOX_ERROR, title.as_ptr(), text.as_ptr(), std::ptr::null_mut());
    }
}

/// Asks the player a yes or no question over the game's window, for mods,
/// with the mouse, keys or a controller. See `dialog`. With no window, as in
/// scripted runs, the answer is the script's, yes unless an `answer:no` step
/// said otherwise.
pub fn ask(cpu: &mut Cpu, title: &str, text: &str, yes: &str, no: &str) -> bool {
    if cpu.host.video.is_none() || cpu.host.script.scripted {
        println!("question: {title} {}", if cpu.host.script.answer { yes } else { no });
        // A scripted run with a display shows the question once, so that a
        // `window` step can photograph it, and doesn't wait for an answer.
        if cpu.host.video.is_some() {
            dialog::show_once(cpu, title, text, yes, no);
        }
        return cpu.host.script.answer;
    }
    dialog::run(cpu, title, text, yes, no)
}

/// Windows virtual-key codes for the SDL keys the game cares about.
fn virtual_key(key: u32) -> Option<u32> {
    const SCANCODE_MASK: u32 = 0x4000_0000;
    Some(match key {
        97..=122 => key - 32,
        48..=57 | 13 | 27 | 8 | 9 | 32 => key,
        0x4000_004f => 0x27, // Right.
        0x4000_0050 => 0x25, // Left.
        0x4000_0051 => 0x28, // Down.
        0x4000_0052 => 0x26, // Up.
        0x4000_004a => 0x24, // Home.
        0x4000_004d => 0x23, // End.
        0x4000_004b => 0x21, // Page up.
        0x4000_004e => 0x22, // Page down.
        0x4000_0049 => 0x2d, // Insert.
        127 => 0x2e,         // Delete.
        0x4000_00e0 | 0x4000_00e4 => 0x11, // Ctrl.
        0x4000_00e1 | 0x4000_00e5 => 0x10, // Shift.
        0x4000_00e2 | 0x4000_00e6 => 0x12, // Alt.
        _ if key & SCANCODE_MASK != 0 && (0x3a..=0x45).contains(&(key & 0xff)) => 0x70 + (key & 0xff) - 0x3a,
        _ => return None,
    })
}

pub(crate) fn handle_event(cpu: &mut Cpu, event: &Event) {
    match event.u32_at(0) {
        EVENT_QUIT => std::process::exit(0),
        EVENT_MOUSE_MOTION => {
            let video = cpu.host.video.as_mut().unwrap();
            let (x, y) = if video.relative {
                // The game put the cursor back where the drag began: add the
                // travel to that, in game pixels.
                let (dx, dy) = (event.f32_at(36) + video.remainder.0, event.f32_at(40) + video.remainder.1);
                video.remainder = (dx.fract(), dy.fract());
                let (x, y) = cpu.host.pointer;
                ((x as f32 + dx.trunc()).clamp(0.0, 639.0) as u32, (y as f32 + dy.trunc()).clamp(0.0, 479.0) as u32)
            } else {
                (event.f32_at(28).clamp(0.0, 639.0) as u32, event.f32_at(32).clamp(0.0, 479.0) as u32)
            };
            cpu.host.pointer = (x, y);
            cpu.host.messages.push_back((WM_MOUSEMOVE, 0, x | (y << 16)));
        }
        kind @ (EVENT_MOUSE_BUTTON_DOWN | EVENT_MOUSE_BUTTON_UP) => {
            let down = kind == EVENT_MOUSE_BUTTON_DOWN;
            let (x, y) = cpu.host.pointer;
            let video = cpu.host.video.as_mut().unwrap();
            let bit = 1u32 << (event.0[24] & 31);
            video.buttons = if down { video.buttons | bit } else { video.buttons & !bit };
            if video.buttons == 0 && video.relative {
                // Hand the cursor back where the game's pointer is.
                video.relative = false;
                unsafe { (video.set_relative)(video.window, false) };
                let (window_x, window_y) = video.game_to_window(x as f32 + 0.5, y as f32 + 0.5);
                unsafe { (video.warp_mouse)(video.window, window_x, window_y) };
            }
            let position = x | (y << 16);
            match event.0[24] {
                1 => cpu.host.messages.push_back((if down { WM_LBUTTONDOWN } else { WM_LBUTTONUP }, if down { MK_LBUTTON } else { 0 }, position)),
                3 => cpu.host.messages.push_back((if down { WM_RBUTTONDOWN } else { WM_RBUTTONUP }, if down { MK_RBUTTON } else { 0 }, position)),
                _ => {}
            }
        }
        EVENT_GAMEPAD_ADDED => {
            if let Some(video) = cpu.host.video.as_ref() {
                unsafe { (video.open_gamepad)(event.u32_at(16)) };
            }
        }
        EVENT_GAMEPAD_REMOVED => cpu.host.pad.sticks = [[0.0; 2]; 2],
        EVENT_GAMEPAD_AXIS => {
            let value = i16::from_le_bytes([event.0[24], event.0[25]]);
            gamepad::set_axis(cpu, event.0[20], value as f32 / i16::MAX as f32);
        }
        kind @ (EVENT_GAMEPAD_BUTTON_DOWN | EVENT_GAMEPAD_BUTTON_UP) => {
            gamepad::set_button(cpu, event.0[20], kind == EVENT_GAMEPAD_BUTTON_DOWN);
        }
        kind @ (EVENT_KEY_DOWN | EVENT_KEY_UP) => {
            let key = event.u32_at(28);
            let down = kind == EVENT_KEY_DOWN;
            if key == KEY_F10 {
                if down {
                    toggle_stats(cpu);
                }
                return;
            }
            if key == KEY_F11 {
                if down {
                    if let Some(video) = cpu.host.video.as_mut() {
                        video.fullscreen = !video.fullscreen;
                        unsafe { (video.set_fullscreen)(video.window, video.fullscreen) };
                    }
                }
                return;
            }
            if let Some(code) = virtual_key(key) {
                cpu.host.messages.push_back((if down { WM_KEYDOWN } else { WM_KEYUP }, code, 0));
                // Windows sends a character for Enter, Backspace, Tab and Escape
                // too, and text fields such as a save's name wait for it.
                if down && ((32..127).contains(&key) || matches!(key, 8 | 9 | 13 | 27)) {
                    cpu.host.messages.push_back((WM_CHAR, key, 0));
                }
            }
        }
        _ => {}
    }
}

/// Reads input and shows the current frame, at most once per display refresh.
pub fn pump(cpu: &mut Cpu) {
    let Some(mut video) = cpu.host.video.take() else { return };
    let mut event = Event([0; 128]);
    while unsafe { (video.poll_event)(&mut event) } {
        video.convert(&mut event);
        cpu.host.video = Some(video);
        handle_event(cpu, &event);
        video = cpu.host.video.take().unwrap();
    }
    // Show every new frame: a Glide swap or a DirectDraw Flip. Waiting a full
    // refresh since the last present instead skipped every other Flip, since
    // presents run a little after the flips, so 60 game frames showed as 30.
    // Glide only changes at a swap, so showing it again would only cost time.
    // A screen drawn straight to the front buffer never flips, so DirectDraw is
    // still shown once a refresh when the game hasn't flipped for a while, as
    // Incubation's menus do with the software renderer. Those refreshes are its
    // frames for the counter, since no Flip times them.
    let glide = cpu.host.glide.as_ref().map(|glide| glide.updates);
    let frame = (glide.is_some(), glide.unwrap_or(cpu.host.draw.frames));
    let new_frame = Some(frame) != video.shown_frame;
    let since_present = video.last_present.elapsed();
    let not_flipping = glide.is_none() && cpu.host.draw.last_flip.elapsed() >= NOT_FLIPPING && since_present >= REFRESH;
    if not_flipping && !new_frame {
        video.record_frame(since_present.as_secs_f32() * 1000.0);
    }
    if new_frame || not_flipping {
        video.shown_frame = Some(frame);
        if video.gl.is_some() {
            // With OpenGL the picture is a texture already when the card
            // drew it, and otherwise its pixels become one.
            let from_card = crate::host::glide::front_texture(cpu).map(Picture::Texture);
            let from_processor = crate::host::glide::front(cpu)
                .map(|(width, height, pixels)| Picture::Pixels(pixels.as_ptr() as *const c_void, width as c_int, height as c_int, gl::BGRA, gl::UNSIGNED_BYTE));
            let picture = from_card.or(from_processor).or_else(|| {
                let (pixels, _) = crate::host::ddraw::frame_for_window(cpu)?;
                Some(Picture::Pixels(pixels as *const c_void, WIDTH, HEIGHT, gl::RGB, gl::UNSIGNED_SHORT_5_6_5))
            });
            if let Some(picture) = picture {
                let started = Instant::now();
                if video.stats {
                    video.clear_overlay();
                    video.draw_stats();
                }
                video.present_gl(picture, video.stats);
                video.present_time += (started.elapsed().as_secs_f32() * 1000.0 - video.present_time) * 0.05;
                cpu.host.presented += started.elapsed().as_secs_f64();
                video.last_present = Instant::now();
            }
            cpu.host.video = Some(video);
            return;
        }
        let frame = match crate::host::glide::front(cpu) {
            Some((width, _, pixels)) => Some((pixels.as_ptr() as *const c_void, width as c_int * 4)),
            None => crate::host::ddraw::frame_for_window(cpu).map(|(pixels, pitch)| (pixels as *const c_void, pitch as c_int)),
        };
        if let Some((pixels, pitch)) = frame {
            let started = Instant::now();
            unsafe {
                (video.update_texture)(video.texture, std::ptr::null(), pixels, pitch);
                // The bars beside the picture take the draw colour, which the
                // frame counter leaves white.
                (video.set_color)(video.renderer, 0, 0, 0, 255);
                (video.render_clear)(video.renderer);
                (video.render_texture)(video.renderer, video.texture, std::ptr::null(), std::ptr::null());
                if video.stats {
                    video.draw_stats();
                }
                (video.render_present)(video.renderer);
            }
            // Smoothed, so the number can be read.
            video.present_time += (started.elapsed().as_secs_f32() * 1000.0 - video.present_time) * 0.05;
            cpu.host.presented += started.elapsed().as_secs_f64();
            video.last_present = Instant::now();
        }
    }
    cpu.host.video = Some(video);
}
