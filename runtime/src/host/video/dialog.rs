//! A yes or no question drawn over the game, for mods. It runs its own loop:
//! the last frame shown, dimmed, under a panel with the question and two
//! buttons, until the player answers. The game is stopped meanwhile, since the
//! mod that asks is waiting for the answer.
//!
//! The no button, on the right, is selected when it opens, so a stray press
//! keeps the game as it was. Left and right, Tab, the d-pad or the left stick
//! choose; Enter, Space or A answers with the selected button; Escape, B or
//! Start means no; the mouse selects by hovering and answers with a click.
//!
//! Everything answers on release, and only for a press made in the dialog, so
//! no half of a click or key reaches the game. Releases of keys and buttons
//! held when it opened, such as the Escape that asked, go to the game after
//! it closes, and so do the sticks' and triggers' last positions, so nothing
//! stays held there.
//!
//! A mod can give it the game's own fonts and colours with
//! `host::set_dialog_fonts`; otherwise it uses SDL's debug font.

use super::*;
use crate::host::bitmap_font::{BitmapFont, DialogFonts, DialogFrame};

const PANEL_WIDTH: f32 = 460.0;
const PADDING: f32 = 20.0;
/// SDL's debug font is 8 by 8 pixels.
const GLYPH: f32 = 8.0;
const TITLE_SCALE: f32 = 2.0;
const LINE: f32 = 12.0;
const BUTTON_WIDTH: f32 = 160.0;
const BUTTON_HEIGHT: f32 = 32.0;
const BUTTON_GAP: f32 = 40.0;
const STICK_PRESS: f32 = 0.5;
const STICK_RELEASE: f32 = 0.3;
const YES: usize = 0;
const NO: usize = 1;

const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_TAB: u32 = 9;
const KEY_SPACE: u32 = 32;
const KEY_RIGHT: u32 = 0x4000_004f;
const KEY_LEFT: u32 = 0x4000_0050;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const KEY_KEYPAD_ENTER: u32 = 0x4000_0058;
const MOUSE_LEFT: u8 = 1;
const BUTTON_SOUTH: u8 = gamepad::BUTTON_SOUTH;
const BUTTON_EAST: u8 = gamepad::BUTTON_EAST;
const BUTTON_START: u8 = gamepad::BUTTON_START;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;
const BUTTON_DPAD_LEFT: u8 = 13;
const BUTTON_DPAD_RIGHT: u8 = 14;
const AXES: usize = 6;

/// Where the panel's parts go, in game pixels. Text positions are the top
/// left of the text.
struct Layout {
    panel: [f32; 4],
    title: (f32, f32, String),
    lines: Vec<(f32, f32, String)>,
    buttons: [[f32; 4]; 2],
    /// In a game's window picture, with the answers as a list.
    framed: bool,
}

const PIXELFORMAT_ARGB8888: u32 = 0x1636_2004;
const TEXTUREACCESS_STATIC: c_int = 0;
/// Room inside a game window's dark inside, and between its text and list.
const FRAME_INSET: f32 = 6.0;
const FRAME_GAP: f32 = 10.0;
/// A list answer's height beyond its font's.
const ANSWER_ROOM: f32 = 8.0;

/// A font the dialog draws with: SDL's debug font at a scale, or the game's
/// own, when a mod gave it (`host::set_dialog_fonts`).
#[derive(Clone, Copy)]
enum Face<'a> {
    Debug(f32),
    Bitmap(&'a BitmapFont),
}

impl Face<'_> {
    fn width(&self, text: &str) -> f32 {
        match self {
            Face::Debug(scale) => printable(text).as_bytes().len() as f32 * GLYPH * scale,
            Face::Bitmap(font) => font.width(text) as f32,
        }
    }

    fn height(&self) -> f32 {
        match self {
            Face::Debug(scale) => GLYPH * scale,
            Face::Bitmap(font) => {
                let (above, below) = font.extent();
                (above + below) as f32
            }
        }
    }

    /// From one line's top to the next.
    fn line(&self) -> f32 {
        match self {
            Face::Debug(_) => LINE,
            Face::Bitmap(_) => self.height() + 3.0,
        }
    }
}

/// The faces for the title, the text and the buttons.
fn faces(fonts: Option<&DialogFonts>) -> [Face<'_>; 3] {
    match fonts {
        Some(fonts) => [Face::Bitmap(&fonts.title), Face::Bitmap(&fonts.text), Face::Bitmap(&fonts.button)],
        None => [Face::Debug(TITLE_SCALE), Face::Debug(1.0), Face::Debug(1.0)],
    }
}

/// Breaks the text into lines of at most `width` characters, at spaces.
#[cfg(test)]
pub(super) fn wrap(text: &str, width: usize) -> Vec<String> {
    wrap_measured(text, width as f32, |line| line.len() as f32)
}

/// Breaks the text into lines no wider than `width` as `measure` measures
/// them, at spaces. A word wider than a line gets one of its own.
fn wrap_measured(text: &str, width: f32, measure: impl Fn(&str) -> f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && measure(&format!("{line} {word}")) > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// The debug font has only ASCII.
fn printable(text: &str) -> CString {
    CString::new(text.chars().map(|c| if c.is_ascii_graphic() || c == ' ' { c } else { '?' }).collect::<String>()).unwrap()
}

/// The question in a game's window: the title centred on its title place,
/// the text and then the answers, one under the other, centred in its inside.
fn framed_layout(title: &str, text: &str, fonts: &DialogFonts, frame: &DialogFrame) -> Layout {
    let [title_face, text_face, button_face] = faces(Some(fonts));
    let left = ((WIDTH as f32 - frame.width as f32) / 2.0).floor();
    let top = ((HEIGHT as f32 - frame.height as f32) / 2.0).floor();
    let [inside_x, inside_y, inside_width, inside_height] = frame.inside.map(|value| value as f32);
    let wrapped = wrap_measured(text, inside_width - 2.0 * FRAME_INSET, |line| text_face.width(line));
    let line = text_face.line();
    let answer = button_face.height() + ANSWER_ROOM;
    let block = wrapped.len() as f32 * line + FRAME_GAP + 2.0 * answer;
    let centre = left + inside_x + inside_width / 2.0;
    let mut y = top + inside_y + ((inside_height - block) / 2.0).max(0.0).floor();
    let mut lines = Vec::new();
    for text in wrapped {
        lines.push(((centre - text_face.width(&text) / 2.0).floor(), y, text));
        y += line;
    }
    y += FRAME_GAP;
    let answer_rect = |y: f32| [left + inside_x + FRAME_INSET, y, inside_width - 2.0 * FRAME_INSET, answer];
    Layout {
        panel: [left, top, frame.width as f32, frame.height as f32],
        title: ((centre - title_face.width(title) / 2.0).floor(), top + frame.title_centre as f32 - (title_face.height() / 2.0).floor(), title.to_string()),
        lines,
        buttons: [answer_rect(y), answer_rect(y + answer)],
        framed: true,
    }
}

fn layout(title: &str, text: &str, fonts: Option<&DialogFonts>) -> Layout {
    if let Some(fonts) = fonts {
        if let Some(frame) = &fonts.frame {
            return framed_layout(title, text, fonts, frame);
        }
    }
    let [title_face, text_face, _] = faces(fonts);
    let wrapped = wrap_measured(text, PANEL_WIDTH - 2.0 * PADDING, |line| text_face.width(line));
    let title_height = title_face.height();
    let line = text_face.line();
    let height = PADDING + title_height + PADDING + wrapped.len() as f32 * line + PADDING + BUTTON_HEIGHT + PADDING;
    let left = (WIDTH as f32 - PANEL_WIDTH) / 2.0;
    let top = ((HEIGHT as f32 - height) / 2.0).floor();
    let centre = WIDTH as f32 / 2.0;
    let title_width = title_face.width(title);
    let text_top = top + PADDING + title_height + PADDING;
    let lines = wrapped
        .into_iter()
        .enumerate()
        .map(|(row, text)| (left + PADDING, text_top + row as f32 * line, text))
        .collect::<Vec<_>>();
    let button_top = text_top + lines.len() as f32 * line + PADDING;
    let first = centre - BUTTON_GAP / 2.0 - BUTTON_WIDTH;
    Layout {
        panel: [left, top, PANEL_WIDTH, height],
        title: ((centre - title_width / 2.0).floor().max(left + PADDING), top + PADDING, title.to_string()),
        lines,
        buttons: [
            [first, button_top, BUTTON_WIDTH, BUTTON_HEIGHT],
            [centre + BUTTON_GAP / 2.0, button_top, BUTTON_WIDTH, BUTTON_HEIGHT],
        ],
        framed: false,
    }
}

fn inside(rect: &[f32; 4], x: f32, y: f32) -> bool {
    x >= rect[0] && x < rect[0] + rect[2] && y >= rect[1] && y < rect[1] + rect[3]
}

fn button_at(layout: &Layout, x: f32, y: f32) -> Option<usize> {
    layout.buttons.iter().position(|rect| inside(rect, x, y))
}

impl Video {
    /// Draws text with its top left at `x`, `top`, its body in `body`.
    fn draw_text(&self, face: Face, x: f32, top: f32, text: &str, body: [u8; 3]) {
        let r = self.renderer;
        unsafe {
            match face {
                Face::Debug(scale) => {
                    (self.set_color)(r, body[0], body[1], body[2], 255);
                    (self.set_scale)(r, scale, scale);
                    (self.debug_text)(r, x / scale, top / scale, printable(text).as_ptr());
                    (self.set_scale)(r, 1.0, 1.0);
                }
                Face::Bitmap(font) => {
                    let (above, _) = font.extent();
                    for (run_x, run_y, length, ink) in font.runs(text, x as i32, top as i32 + above) {
                        let [red, green, blue] = if ink == 1 { body } else { font.shade };
                        (self.set_color)(r, red, green, blue, 255);
                        let rect = [run_x as f32, run_y as f32, length as f32, 1.0];
                        (self.fill_rect)(r, rect.as_ptr());
                    }
                }
            }
        }
    }

    /// The game's window picture as a texture, for one question. Destroy it
    /// with `destroy_texture` when the question closes.
    fn frame_texture(&self, fonts: Option<&DialogFonts>) -> Option<Pointer> {
        let frame = fonts?.frame.as_ref()?;
        unsafe {
            let texture = (self.create_texture)(self.renderer, PIXELFORMAT_ARGB8888, TEXTUREACCESS_STATIC, frame.width as c_int, frame.height as c_int);
            if texture.is_null() {
                return None;
            }
            (self.update_texture)(texture, std::ptr::null(), frame.pixels.as_ptr() as *const c_void, (frame.width * 4) as c_int);
            (self.set_texture_blend)(texture, BLEND_ALPHA);
            Some(texture)
        }
    }

    fn draw_dialog(&self, layout: &Layout, labels: &[&str; 2], selected: usize, fonts: Option<&DialogFonts>, frame: Option<Pointer>) {
        let [title_face, text_face, button_face] = faces(fonts);
        unsafe {
            let r = self.renderer;
            if self.gl.is_some() {
                // With OpenGL this is drawn to the surface that goes over
                // the last frame.
                self.clear_overlay();
            } else {
                (self.set_color)(r, 0, 0, 0, 255);
                (self.render_clear)(r);
                (self.render_texture)(r, self.texture, std::ptr::null(), std::ptr::null());
            }
            (self.set_blend)(r, BLEND_ALPHA);
            (self.set_color)(r, 0, 0, 0, 150);
            let whole = [0.0, 0.0, WIDTH as f32, HEIGHT as f32];
            (self.fill_rect)(r, whole.as_ptr());
            match frame.filter(|_| layout.framed) {
                Some(texture) => {
                    (self.render_texture)(r, texture, std::ptr::null(), layout.panel.as_ptr() as *const c_void);
                }
                None => {
                    (self.set_color)(r, 20, 24, 32, 235);
                    (self.fill_rect)(r, layout.panel.as_ptr());
                    let [x, y, w, h] = layout.panel;
                    (self.set_color)(r, 150, 160, 180, 255);
                    for (x1, y1, x2, y2) in [(x, y, x + w - 1.0, y), (x, y + h - 1.0, x + w - 1.0, y + h - 1.0), (x, y, x, y + h - 1.0), (x + w - 1.0, y, x + w - 1.0, y + h - 1.0)] {
                        (self.draw_line)(r, x1, y1, x2, y2);
                    }
                }
            }
            // The game's fonts come with its colours; the debug font keeps
            // the runtime's own.
            let (title_colour, text_colour) = match fonts {
                Some(fonts) => (fonts.title.body, fonts.text.body),
                None => ([255, 220, 120], [230, 230, 230]),
            };
            let (title_x, title_y, title) = &layout.title;
            self.draw_text(title_face, *title_x, *title_y, title, title_colour);
            for (x, y, line) in &layout.lines {
                self.draw_text(text_face, *x, *y, line, text_colour);
            }
            for (index, rect) in layout.buttons.iter().enumerate() {
                let chosen = index == selected;
                if let (true, Some(fonts)) = (layout.framed, fonts) {
                    // A list entry, as the game's save slots: only its text,
                    // lit when chosen.
                    let label = labels[index];
                    let colour = if chosen { fonts.selected } else { fonts.button.body };
                    let (text_x, text_y) = (rect[0] + (rect[2] - button_face.width(label)) / 2.0, rect[1] + (rect[3] - button_face.height()) / 2.0);
                    self.draw_text(button_face, text_x.floor(), text_y.floor(), label, colour);
                    continue;
                }
                // With the game's light text, the chosen button is a lighter
                // grey with a frame, rather than the bright yellow of the
                // debug font's dark text.
                let (fill, label_colour) = match (fonts, chosen) {
                    (Some(fonts), true) => ([88, 94, 112], fonts.selected),
                    (Some(fonts), false) => ([44, 48, 60], fonts.button.body),
                    (None, true) => ([230, 180, 70], [20, 20, 20]),
                    (None, false) => ([60, 66, 82], [230, 230, 230]),
                };
                (self.set_color)(r, fill[0], fill[1], fill[2], 255);
                (self.fill_rect)(r, rect.as_ptr());
                if chosen && fonts.is_some() {
                    let [x, y, w, h] = *rect;
                    (self.set_color)(r, 200, 196, 200, 255);
                    for (x1, y1, x2, y2) in [(x, y, x + w - 1.0, y), (x, y + h - 1.0, x + w - 1.0, y + h - 1.0), (x, y, x, y + h - 1.0), (x + w - 1.0, y, x + w - 1.0, y + h - 1.0)] {
                        (self.draw_line)(r, x1, y1, x2, y2);
                    }
                }
                let label = labels[index];
                let (text_x, text_y) = (rect[0] + (rect[2] - button_face.width(label)) / 2.0, rect[1] + (rect[3] - button_face.height()) / 2.0);
                self.draw_text(button_face, text_x.floor(), text_y.floor(), label, label_colour);
            }
            (self.set_blend)(r, BLEND_NONE);
            if self.gl.is_none() {
                (self.render_present)(r);
            }
        }
    }
}

/// What the dialog saw pressed, so it answers only to its own presses and
/// hands the rest to the game.
#[derive(Default)]
struct Held {
    keys: Vec<u32>,
    mouse: u32,
    pad: u32,
    /// The left stick past the threshold, left or right, for one step a push.
    stick: bool,
}

/// Draws the question over the last frame and leaves it there, for scripted runs.
pub(super) fn show_once(cpu: &mut Cpu, title: &str, text: &str, yes: &str, no: &str) {
    let fonts = cpu.host.dialog_fonts.clone();
    let Some(video) = cpu.host.video.as_mut() else { return };
    let frame = video.frame_texture(fonts.as_deref());
    video.draw_dialog(&layout(title, text, fonts.as_deref()), &[yes, no], NO, fonts.as_deref(), frame);
    if let Some(texture) = frame {
        unsafe { (video.destroy_texture)(texture) };
    }
    video.present_gl(Picture::Last, true);
    video.shown_frame = None;
    // The game draws over it at once, so the picture is taken here.
    if let Some((width, height, pixels)) = video.window_picture() {
        let path = format!("{}/question.png", cpu.host.script.output);
        crate::host::script::write_png(&path, width, height, &pixels);
        println!("question saved as question.png");
    }
}

pub(super) fn run(cpu: &mut Cpu, title: &str, text: &str, yes: &str, no: &str) -> bool {
    let fonts = cpu.host.dialog_fonts.clone();
    let mut video = cpu.host.video.take().unwrap();
    let layout = layout(title, text, fonts.as_deref());
    let labels = [yes, no];
    let frame = video.frame_texture(fonts.as_deref());
    unsafe {
        // The game draws its own pointer and hides the system's.
        (video.set_relative)(video.window, false);
        (video.show_cursor)();
    }
    video.relative = false;
    let mut selected = NO;
    let mut held = Held::default();
    // Events for the game once the dialog closes, and the axes' last values.
    let mut later: Vec<Event> = Vec::new();
    let mut axes: [Option<Event>; AXES] = Default::default();
    let mut mouse_on: Option<usize> = None;
    let answer = 'dialog: loop {
        let mut event = Event([0; 128]);
        while unsafe { (video.poll_event)(&mut event) } {
            video.convert(&mut event);
            match event.u32_at(0) {
                EVENT_QUIT => std::process::exit(0),
                EVENT_GAMEPAD_ADDED => unsafe {
                    (video.open_gamepad)(event.u32_at(16));
                },
                EVENT_MOUSE_MOTION => {
                    if let Some(index) = button_at(&layout, event.f32_at(28), event.f32_at(32)) {
                        selected = index;
                    }
                }
                EVENT_MOUSE_BUTTON_DOWN => {
                    let button = event.0[24];
                    held.mouse |= 1 << (button & 31);
                    if button == MOUSE_LEFT {
                        mouse_on = button_at(&layout, event.f32_at(28), event.f32_at(32));
                        if let Some(index) = mouse_on {
                            selected = index;
                        }
                    }
                }
                EVENT_MOUSE_BUTTON_UP => {
                    let button = event.0[24];
                    let bit = 1 << (button & 31);
                    if held.mouse & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    held.mouse &= !bit;
                    if button == MOUSE_LEFT {
                        let on = button_at(&layout, event.f32_at(28), event.f32_at(32));
                        if on.is_some() && on == mouse_on {
                            break 'dialog on == Some(YES);
                        }
                        mouse_on = None;
                    }
                }
                EVENT_KEY_DOWN => {
                    let key = event.u32_at(28);
                    if event.0[37] != 0 {
                        continue; // A repeat.
                    }
                    if key == KEY_F10 || key == KEY_F11 {
                        cpu.host.video = Some(video);
                        handle_event(cpu, &event);
                        video = cpu.host.video.take().unwrap();
                        continue;
                    }
                    if !held.keys.contains(&key) {
                        held.keys.push(key);
                    }
                    match key {
                        KEY_LEFT => selected = YES,
                        KEY_RIGHT => selected = NO,
                        KEY_TAB | KEY_UP | KEY_DOWN => selected = 1 - selected,
                        _ => {}
                    }
                }
                EVENT_KEY_UP => {
                    let key = event.u32_at(28);
                    let Some(at) = held.keys.iter().position(|k| *k == key) else {
                        later.push(Event(event.0));
                        continue;
                    };
                    held.keys.swap_remove(at);
                    match key {
                        KEY_RETURN | KEY_KEYPAD_ENTER | KEY_SPACE => break 'dialog selected == YES,
                        KEY_ESCAPE => break 'dialog false,
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_DOWN => {
                    let button = event.0[20];
                    held.pad |= 1 << (button & 31);
                    match button {
                        BUTTON_DPAD_LEFT => selected = YES,
                        BUTTON_DPAD_RIGHT => selected = NO,
                        BUTTON_DPAD_UP | BUTTON_DPAD_DOWN => selected = 1 - selected,
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_UP => {
                    let button = event.0[20];
                    let bit = 1 << (button & 31);
                    if held.pad & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    held.pad &= !bit;
                    match button {
                        BUTTON_SOUTH => break 'dialog selected == YES,
                        BUTTON_EAST | BUTTON_START => break 'dialog false,
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_AXIS => {
                    let axis = event.0[20] as usize;
                    let value = i16::from_le_bytes([event.0[24], event.0[25]]) as f32 / i16::MAX as f32;
                    if axis == 0 {
                        if !held.stick && value.abs() >= STICK_PRESS {
                            held.stick = true;
                            selected = if value < 0.0 { YES } else { NO };
                        } else if held.stick && value.abs() < STICK_RELEASE {
                            held.stick = false;
                        }
                    }
                    if axis < AXES {
                        axes[axis] = Some(Event(event.0));
                    }
                }
                EVENT_GAMEPAD_REMOVED => later.push(Event(event.0)),
                _ => {}
            }
        }
        video.draw_dialog(&layout, &labels, selected, fonts.as_deref(), frame);
        video.present_gl(Picture::Last, true);
        if !video.vsync {
            std::thread::sleep(REFRESH);
        }
    };
    unsafe {
        (video.hide_cursor)();
        if let Some(texture) = frame {
            (video.destroy_texture)(texture);
        }
    }
    // The game's frame goes back on screen at its next swap or flip.
    video.shown_frame = None;
    cpu.host.video = Some(video);
    for event in later.iter().chain(axes.iter().flatten()) {
        handle_event(cpu, event);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_at_spaces_within_the_width() {
        assert_eq!(wrap("one two three four", 9), ["one two", "three", "four"]);
        assert_eq!(wrap("  spaced   out  ", 20), ["spaced out"]);
        assert!(wrap("", 10).is_empty());
    }

    #[test]
    fn keeps_a_word_longer_than_a_line_whole() {
        assert_eq!(wrap("a verylongword b", 5), ["a", "verylongword", "b"]);
    }

    #[test]
    fn buttons_fit_inside_the_panel_and_the_screen() {
        let text = "Do you really want to give up? All your units will be lost, and so will the mission.";
        let layout = layout("Give up the mission?", text, None);
        let [x, y, w, h] = layout.panel;
        assert!(x >= 0.0 && y >= 0.0 && x + w <= WIDTH as f32 && y + h <= HEIGHT as f32);
        for button in &layout.buttons {
            assert!(button[0] >= x && button[0] + button[2] <= x + w);
            assert!(button[1] + button[3] <= y + h);
        }
        assert_eq!(button_at(&layout, layout.buttons[NO][0] + 1.0, layout.buttons[NO][1] + 1.0), Some(NO));
        assert_eq!(button_at(&layout, x + 1.0, y + 1.0), None);
        assert!(layout.lines.iter().all(|(_, _, line)| line.len() as f32 * GLYPH <= w - 2.0 * PADDING));
    }

    #[test]
    fn a_game_window_holds_the_text_and_the_answers() {
        use crate::host::bitmap_font::Glyph;
        // Every letter a 5 by 9 block, about as wide as Incubation's fonts.
        let font = || BitmapFont {
            glyphs: (0..256).map(|byte| (byte > 32).then(|| Glyph { width: 5, height: 9, anchor_x: 0, anchor_y: 7, inks: vec![1; 45] })).collect(),
            space: 4,
            spacing: 1,
            body: [200; 3],
            shade: [100; 3],
        };
        let frame = DialogFrame { width: 247, height: 267, pixels: vec![0; 247 * 267], title_centre: 47, inside: [20, 80, 207, 158] };
        let fonts = DialogFonts { title: font(), text: font(), button: font(), selected: [255; 3], frame: Some(frame) };
        let text = "Willst du die Mission wirklich aufgeben? Alle deine Einheiten gehen verloren, und die Mission ist verloren.";
        let layout = layout("Aufgeben?", text, Some(&fonts));
        assert!(layout.framed);
        let [x, y, ..] = layout.panel;
        let inside = [x + 20.0, y + 80.0, 207.0, 158.0];
        for (line_x, line_y, line) in &layout.lines {
            assert!(*line_x >= inside[0] && line_x + fonts.text.width(line) as f32 <= inside[0] + inside[2], "{line}");
            assert!(*line_y >= inside[1]);
        }
        for button in &layout.buttons {
            assert!(button[0] >= inside[0] && button[0] + button[2] <= inside[0] + inside[2]);
            assert!(button[1] + button[3] <= inside[1] + inside[3]);
        }
        assert_eq!(button_at(&layout, layout.buttons[YES][0] + 1.0, layout.buttons[YES][1] + 1.0), Some(YES));
    }

    #[test]
    fn replaces_what_the_font_lacks() {
        assert_eq!(printable("Mission aufgeben?").as_bytes(), b"Mission aufgeben?");
        assert_eq!(printable("Größe").as_bytes(), b"Gr??e");
    }
}
