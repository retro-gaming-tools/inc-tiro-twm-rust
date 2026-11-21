//! A game's own bitmap font, for text the runtime draws over the game, such
//! as `host::ask`'s question. A mod reads it from the player's game files and
//! hands it over with `host::set_dialog_fonts`, so no game data is in the
//! runtime. Without one, the runtime draws with SDL's debug font.
//!
//! Glyph pixels are inks: 0 is empty, 1 the body and 2 the shade, the
//! antialiased edge, as in Incubation's fonts. Text is Latin-1, one byte a
//! character; anything outside it becomes `?`.

/// One character: its size, the point that sits on the pen at the baseline,
/// and its inks, row by row.
pub struct Glyph {
    pub width: usize,
    pub height: usize,
    pub anchor_x: i32,
    pub anchor_y: i32,
    pub inks: Vec<u8>,
}

pub struct BitmapFont {
    /// By Latin-1 byte; `None` draws nothing and takes no room.
    pub glyphs: Vec<Option<Glyph>>,
    /// A space's width, and the gap after every character, in pixels.
    pub space: i32,
    pub spacing: i32,
    /// The colours of ink 1 and ink 2, as RGB.
    pub body: [u8; 3],
    pub shade: [u8; 3],
}

/// The fonts a mod gives the question box, with the game's colours for each.
pub struct DialogFonts {
    pub title: BitmapFont,
    pub text: BitmapFont,
    /// Button captions; `selected` is the body colour of the chosen one.
    pub button: BitmapFont,
    pub selected: [u8; 3],
    /// A window from the game to draw the question in, instead of the
    /// runtime's plain panel.
    pub frame: Option<DialogFrame>,
}

/// A game's window picture for the question box. The answers become a list
/// under the text, as in the game's own windows, rather than two buttons.
pub struct DialogFrame {
    pub width: usize,
    pub height: usize,
    /// Row by row, as 0xAARRGGBB; alpha 0 is see-through.
    pub pixels: Vec<u32>,
    /// Where the title's middle goes, from the picture's top.
    pub title_centre: i32,
    /// The dark inside that holds the text and the answers: x, y, width and
    /// height in the picture.
    pub inside: [i32; 4],
}

impl BitmapFont {
    /// The text as the font's bytes.
    pub fn bytes(text: &str) -> Vec<u8> {
        text.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect()
    }

    fn glyph(&self, byte: u8) -> Option<&Glyph> {
        self.glyphs.get(byte as usize)?.as_ref()
    }

    /// How wide the text is drawn, with the spacing after each character.
    pub fn width(&self, text: &str) -> i32 {
        Self::bytes(text)
            .iter()
            .map(|&byte| if byte == b' ' { self.space + self.spacing } else { self.glyph(byte).map_or(0, |glyph| glyph.width as i32 + self.spacing) })
            .sum()
    }

    /// How far the tallest glyph reaches above the baseline, and below it.
    pub fn extent(&self) -> (i32, i32) {
        let glyphs = self.glyphs.iter().flatten();
        let above = glyphs.clone().map(|glyph| glyph.anchor_y).max().unwrap_or(0);
        let below = glyphs.map(|glyph| glyph.height as i32 - glyph.anchor_y).max().unwrap_or(0);
        (above, below)
    }

    /// The runs of one ink to fill for the text with its pen at `x` and its
    /// baseline at `y`: x, y, length and ink, for drawing as rectangles.
    pub fn runs(&self, text: &str, x: i32, y: i32) -> Vec<(i32, i32, i32, u8)> {
        let mut runs = Vec::new();
        let mut pen = x;
        for byte in Self::bytes(text) {
            if byte == b' ' {
                pen += self.space + self.spacing;
                continue;
            }
            let Some(glyph) = self.glyph(byte) else { continue };
            let (left, top) = (pen - glyph.anchor_x, y - glyph.anchor_y);
            for (row, inks) in glyph.inks.chunks(glyph.width.max(1)).enumerate() {
                let mut column = 0;
                while column < inks.len() {
                    let ink = inks[column];
                    let start = column;
                    while column < inks.len() && inks[column] == ink {
                        column += 1;
                    }
                    if ink == 1 || ink == 2 {
                        runs.push((left + start as i32, top + row as i32, (column - start) as i32, ink));
                    }
                }
            }
            pen += glyph.width as i32 + self.spacing;
        }
        runs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A font with a 2x3 `A`, all body but a shaded corner, anchored on its
    /// bottom row.
    fn font() -> BitmapFont {
        let mut glyphs: Vec<Option<Glyph>> = (0..256).map(|_| None).collect();
        glyphs[b'A' as usize] = Some(Glyph { width: 2, height: 3, anchor_x: 0, anchor_y: 2, inks: vec![1, 2, 1, 1, 0, 1] });
        BitmapFont { glyphs, space: 4, spacing: 1, body: [255; 3], shade: [128; 3] }
    }

    #[test]
    fn measures_glyphs_spaces_and_spacing() {
        let font = font();
        assert_eq!(font.width("A"), 3);
        assert_eq!(font.width("A A"), 3 + 5 + 3);
        // Missing glyphs take no room, characters outside Latin-1 neither.
        assert_eq!(font.width("Ab€"), 3);
        assert_eq!(font.extent(), (2, 1));
    }

    #[test]
    fn draws_runs_of_each_ink_from_the_anchor() {
        let runs = font().runs("A", 10, 20);
        assert_eq!(runs, vec![(10, 18, 1, 1), (11, 18, 1, 2), (10, 19, 2, 1), (11, 20, 1, 1)]);
    }

    #[test]
    fn latin1_keeps_umlauts() {
        assert_eq!(BitmapFont::bytes("ßÄ€"), vec![0xdf, 0xc4, b'?']);
    }
}
