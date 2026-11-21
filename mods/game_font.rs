//! The game's own fonts for the runtime's question box (`host::ask`, used by
//! `give_up.rs`), read from the player's `Libs/font_0.LIB` and `font_1.LIB`,
//! so it looks like the game's own windows rather than SDL's debug font.
//! Nothing here replaces a game function.
//!
//! A `.LIB` shape library: a 20-byte header (count at 4, used size at 8,
//! table at 12, pixels at 16), then 36 bytes a shape (width at 4, height at
//! 8, pixel offset at 16, size at 24, anchor x and y at 28 and 32), then the
//! pixels, one byte each. In the fonts, shape `n` is character `32 + n` in
//! the game's Latin-1 text, and the pixels are inks: 1 the body, 2 the shade.
//! Spaces are 4 pixels in these two fonts, and every character is followed
//! by 1 pixel. See `docs/formats.md` and `docs/original-engine.md` ("Text")
//! in the port repo.
//!
//! Colours are the game's own 5-6-5 values: text 0xCE59 with shade 0x632C
//! (`GUI_SetText` colour 1 and 3), button captions 0xAD34 with 0x4227, and
//! 0xFFDE for the lit one (`GUI_SetButton`).
//!
//! The window is the load and save window's picture, shape 0 of
//! `Libs/loadsave.LIB` (247 by 267) with `Graphics/loadsave.pal`: the title
//! goes in the dark arch under the skull (y 28 to 67), and the text and the
//! answers in the dark inside (about x 20 to 227, y 80 to 238), as the game
//! puts "Speichern" and its save slots there. A `.pal` is an 8-bit BMP whose
//! colour table is the palette (blue, green, red, unused), from byte 54 to
//! the pixel offset at byte 10. Index 0 is see-through.

use recomp_runtime::host::bitmap_font::{BitmapFont, DialogFonts, DialogFrame, Glyph};
use recomp_runtime::{host, Cpu};

const FIRST_CHARACTER: usize = 32;
const SPACE: i32 = 4;
const SPACING: i32 = 1;
const HEADER: usize = 20;
const ENTRY: usize = 36;

/// Gives the question box the game's fonts, once. If they can't be read, it
/// keeps its own.
pub fn give_to_dialog(cpu: &mut Cpu) {
    if host::has_dialog_fonts(cpu) {
        return;
    }
    let file = |name: &str| std::fs::read(host::game_path(cpu, name)).ok();
    let read = |name: &str, body: u16, shade: u16| -> Option<BitmapFont> { font(&file(&format!("Libs\\{name}"))?, colour(body), colour(shade)) };
    // The title is in the regular font, as the game's "Speichern" is.
    let (Some(title), Some(text), Some(button)) = (read("font_0.LIB", 0xce59, 0x632c), read("font_0.LIB", 0xce59, 0x632c), read("font_1.LIB", 0xad34, 0x4227)) else {
        eprintln!("The game's fonts couldn't be read; questions use the runtime's own.");
        return;
    };
    // Without the window the question keeps the runtime's panel, in the game's fonts.
    let frame = file("Libs\\loadsave.LIB").zip(file("Graphics\\loadsave.pal")).and_then(|(library, palette)| window(&library, &palette));
    host::set_dialog_fonts(cpu, DialogFonts { title, text, button, selected: colour(0xffde), frame });
}

const WINDOW_TITLE_CENTRE: i32 = 47;
const WINDOW_INSIDE: [i32; 4] = [20, 80, 207, 158];

/// The load and save window from `loadsave.LIB` and its palette.
fn window(library: &[u8], palette: &[u8]) -> Option<DialogFrame> {
    let colours_end = u32_at(palette, 10)? as usize;
    let colours: Vec<u32> = palette.get(54..colours_end)?.chunks_exact(4).map(|bgr| 0xff00_0000 | u32::from(bgr[2]) << 16 | u32::from(bgr[1]) << 8 | u32::from(bgr[0])).collect();
    let (width, height, inks) = shape(library, 0)?;
    let pixels = inks.iter().map(|&index| if index == 0 { 0 } else { colours.get(index as usize).copied().unwrap_or(0) }).collect();
    Some(DialogFrame { width, height, pixels, title_centre: WINDOW_TITLE_CENTRE, inside: WINDOW_INSIDE })
}

/// One shape of a `.LIB`: its width, height and pixels.
fn shape(bytes: &[u8], index: usize) -> Option<(usize, usize, &[u8])> {
    let count = u32_at(bytes, 4)? as usize;
    let pixels = u32_at(bytes, 16)? as usize;
    if index >= count || u32_at(bytes, 12)? as usize != HEADER || pixels != HEADER + count * ENTRY {
        return None;
    }
    let entry = HEADER + index * ENTRY;
    let (width, height) = (u32_at(bytes, entry + 4)? as usize, u32_at(bytes, entry + 8)? as usize);
    let (offset, size) = (u32_at(bytes, entry + 16)? as usize, u32_at(bytes, entry + 24)? as usize);
    if size != width * height || width > 640 || height > 480 {
        return None;
    }
    Some((width, height, bytes.get(pixels + offset..pixels + offset + size)?))
}

/// A 5-6-5 colour as RGB.
fn colour(value: u16) -> [u8; 3] {
    let scale = |part: u16, max: u16| (u32::from(part) * 255 / u32::from(max)) as u8;
    [scale(value >> 11, 31), scale((value >> 5) & 63, 63), scale(value & 31, 31)]
}

fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// Reads a font from a `.LIB` file, or `None` if it isn't one.
fn font(bytes: &[u8], body: [u8; 3], shade: [u8; 3]) -> Option<BitmapFont> {
    let count = u32_at(bytes, 4)? as usize;
    let used = u32_at(bytes, 8)? as usize;
    let pixels = u32_at(bytes, 16)? as usize;
    if u32_at(bytes, 12)? as usize != HEADER || pixels != HEADER + count * ENTRY || used > bytes.len() || used < pixels || count > 256 - FIRST_CHARACTER {
        return None;
    }
    let mut glyphs: Vec<Option<Glyph>> = (0..256).map(|_| None).collect();
    for index in 0..count {
        let entry = HEADER + index * ENTRY;
        let (width, height) = (u32_at(bytes, entry + 4)? as usize, u32_at(bytes, entry + 8)? as usize);
        let (offset, size) = (u32_at(bytes, entry + 16)? as usize, u32_at(bytes, entry + 24)? as usize);
        let (anchor_x, anchor_y) = (u32_at(bytes, entry + 28)? as i32, u32_at(bytes, entry + 32)? as i32);
        if size != width * height || width > 256 || height > 256 {
            return None;
        }
        let inks = bytes.get(pixels + offset..pixels + offset + size)?.to_vec();
        if width > 0 {
            glyphs[FIRST_CHARACTER + index] = Some(Glyph { width, height, anchor_x, anchor_y, inks });
        }
    }
    Some(BitmapFont { glyphs, space: SPACE, spacing: SPACING, body, shade })
}
