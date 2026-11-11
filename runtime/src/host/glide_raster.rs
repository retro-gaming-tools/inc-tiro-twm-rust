//! The drawing half of Glide: textures, state and triangles. Colours are worked
//! on as 0.0 to 255.0, as the card's combine units define them.

use crate::cpu::Cpu;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

const VERTEX_SIZE: u32 = 60;
const LODS: usize = 9;

pub struct Level {
    pub width: usize,
    pub height: usize,
    /// 0xAARRGGBB texels.
    pub texels: Vec<u32>,
}

pub struct Texture {
    pub levels: Vec<Level>,
    /// How texture coordinates, which run to 256, map to the largest level.
    pub s_scale: f32,
    pub t_scale: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Combine {
    pub function: u32,
    pub factor: u32,
    pub local: u32,
    pub other: u32,
    pub invert: bool,
}

/// What the card holds between draws.
#[derive(Default)]
pub struct State {
    pub managed: Vec<Managed>,
    pub textures: HashMap<u32, Arc<Texture>>,
    pub source: Option<u32>,
    pub palette: Palette,
    pub pipeline: Pipeline,
}

pub struct Palette(pub [u32; 256]);

impl Default for Palette {
    fn default() -> Palette {
        Palette([0; 256])
    }
}

impl Deref for State {
    type Target = Pipeline;
    fn deref(&self) -> &Pipeline {
        &self.pipeline
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
}

/// The settings a triangle is drawn with.
#[derive(Clone, PartialEq)]
pub struct Pipeline {
    pub color: Combine,
    pub alpha: Combine,
    pub blend: [u32; 2],
    pub constant: u32,
    pub cull: u32,
    pub depth_mode: u32,
    pub depth_function: u32,
    pub depth_write: bool,
    pub color_write: bool,
    pub alpha_test: u32,
    pub alpha_reference: f32,
    pub chroma: bool,
    pub chroma_value: u32,
    pub fog_mode: u32,
    pub fog_color: u32,
    pub fog_table: [u8; 64],
    pub filter: bool,
    pub clamp: [bool; 2],
    pub clip: [i32; 4],
}

impl Default for Pipeline {
    fn default() -> Pipeline {
        Pipeline {
            color: Combine { function: 1, factor: 0, local: 0, other: 0, invert: false },
            alpha: Combine { function: 1, factor: 0, local: 0, other: 0, invert: false },
            blend: [4, 0],
            constant: 0xffff_ffff,
            cull: 0,
            depth_mode: 0,
            depth_function: 1,
            depth_write: false,
            color_write: true,
            alpha_test: 7,
            alpha_reference: 0.0,
            chroma: false,
            chroma_value: 0,
            fog_mode: 0,
            fog_color: 0,
            fog_table: [0; 64],
            filter: true,
            clamp: [false, false],
            clip: [0, 0, 640, 480],
        }
    }
}

/// A texture's sizes: a large-LOD dimension is 256 >> lod, and the aspect ratio
/// shortens one side.
fn level_size(lod: u32, aspect: u32) -> (usize, usize) {
    const SQUARE: u32 = 3;
    let side = (256usize >> lod).max(1);
    if aspect <= SQUARE {
        (side, (side >> (SQUARE - aspect)).max(1))
    } else {
        ((side >> (aspect - SQUARE)).max(1), side)
    }
}

fn bytes_per_texel(format: u32) -> usize {
    const FIRST_16_BIT: u32 = 8;
    if format >= FIRST_16_BIT { 2 } else { 1 }
}

/// The card memory a texture takes, for the engine's own allocator.
pub fn memory_required(small: u32, large: u32, aspect: u32, format: u32) -> u32 {
    let total: usize = (large..=small).map(|lod| {
        let (width, height) = level_size(lod, aspect);
        width * height * bytes_per_texel(format)
    }).sum();
    ((total + 7) & !7) as u32
}

fn decode(value: u32, format: u32, palette: &[u32; 256]) -> u32 {
    let scale = |v: u32, bits: u32| v * 255 / ((1 << bits) - 1);
    match format {
        0 => 0xff00_0000 | (scale(value >> 5, 3) << 16) | (scale((value >> 2) & 7, 3) << 8) | scale(value & 3, 2),
        2 => value << 24,
        3 => 0xff00_0000 | (value * 0x0001_0101),
        4 => (scale(value >> 4, 4) << 24) | (scale(value & 15, 4) * 0x0001_0101),
        5 => 0xff00_0000 | palette[value as usize],
        8 => ((value >> 8) << 24) | (scale((value >> 5) & 7, 3) << 16) | (scale((value >> 2) & 7, 3) << 8) | scale(value & 3, 2),
        10 => 0xff00_0000 | (scale(value >> 11, 5) << 16) | (scale((value >> 5) & 63, 6) << 8) | scale(value & 31, 5),
        11 => (if value & 0x8000 != 0 { 0xff00_0000 } else { 0 }) | (scale((value >> 10) & 31, 5) << 16) | (scale((value >> 5) & 31, 5) << 8) | scale(value & 31, 5),
        12 => (scale(value >> 12, 4) << 24) | (scale((value >> 8) & 15, 4) << 16) | (scale((value >> 4) & 15, 4) << 8) | scale(value & 15, 4),
        13 => ((value >> 8) << 24) | ((value & 255) * 0x0001_0101),
        14 => ((value >> 8) << 24) | palette[(value & 255) as usize],
        _ => 0xffff_00ff,
    }
}

/// A texture the `gu` calls manage: allocated first, and filled later.
#[derive(Clone, Copy)]
pub struct Managed {
    pub small: u32,
    pub large: u32,
    pub aspect: u32,
    pub format: u32,
    pub clamp: [bool; 2],
    pub filter: bool,
}

/// Reads a GrTexInfo: small LOD, large LOD, aspect ratio, format and data.
pub fn download(cpu: &Cpu, state: &mut State, start: u32, info: u32) {
    let (small, large, aspect, format, at) = (cpu.r32(info), cpu.r32(info + 4), cpu.r32(info + 8), cpu.r32(info + 12), cpu.r32(info + 16));
    download_levels(cpu, state, start, (small, large, aspect, format), at);
}

/// Decodes a whole mipmap chain into a texture.
pub fn download_levels(cpu: &Cpu, state: &mut State, start: u32, (small, large, aspect, format): (u32, u32, u32, u32), mut at: u32) {
    let (top_width, top_height) = level_size(large, aspect);
    let longest = top_width.max(top_height) as f32;
    let mut levels = Vec::with_capacity(LODS);
    let width_16 = bytes_per_texel(format) == 2;
    for lod in large..=small {
        let (width, height) = level_size(lod, aspect);
        let mut texels = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            let value = if width_16 { cpu.r16(at) as u32 } else { cpu.r8(at) as u32 };
            at += if width_16 { 2 } else { 1 };
            texels.push(decode(value, format, &state.palette.0));
        }
        levels.push(Level { width, height, texels });
    }
    let texture = Texture { levels, s_scale: top_width as f32 / longest, t_scale: top_height as f32 / longest };
    state.textures.insert(start, Arc::new(texture));
}

pub fn download_palette(cpu: &Cpu, state: &mut State, table: u32) {
    for index in 0..256 {
        state.palette.0[index as usize] = cpu.r32(table + index * 4) & 0xff_ffff;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub oow: f32,
    pub sow: f32,
    pub tow: f32,
}

fn float(cpu: &Cpu, at: u32) -> f32 {
    f32::from_bits(cpu.r32(at))
}

/// Games add this to positions to snap them to the card's fixed-point grid, and
/// the card only reads the low bits.
const SNAP: f32 = (3 << 18) as f32;

fn unsnap(value: f32) -> f32 {
    if value >= SNAP / 2.0 { value - SNAP } else { value }
}

pub fn read_vertex(cpu: &Cpu, at: u32) -> Vertex {
    Vertex {
        x: unsnap(float(cpu, at)),
        y: unsnap(float(cpu, at + 4)),
        r: float(cpu, at + 12),
        g: float(cpu, at + 16),
        b: float(cpu, at + 20),
        a: float(cpu, at + 28),
        oow: float(cpu, at + 32),
        sow: float(cpu, at + 36),
        tow: float(cpu, at + 40),
    }
}

pub fn read_list(cpu: &Cpu, count: u32, at: u32) -> Vec<Vertex> {
    (0..count).map(|index| read_vertex(cpu, at + index * VERTEX_SIZE)).collect()
}

fn channels(color: u32) -> [f32; 4] {
    [((color >> 16) & 255) as f32, ((color >> 8) & 255) as f32, (color & 255) as f32, (color >> 24) as f32]
}

/// Mixes two 0xAARRGGBB colours, with `amount` from 0 to 256 of the second.
#[inline(always)]
fn mix(a: u32, b: u32, amount: u32) -> u32 {
    let keep = 256 - amount;
    let red_blue = (((a & 0xff00ff) * keep + (b & 0xff00ff) * amount) >> 8) & 0xff00ff;
    let alpha_green = ((a >> 8) & 0xff00ff) * keep + ((b >> 8) & 0xff00ff) * amount;
    red_blue | (alpha_green & 0xff00ff00)
}

/// The texture's colour at a point, as 0xAARRGGBB.
///
/// Coordinates become fixed point with 8 bits of fraction: one conversion
/// gives both the texel and how far into it the point is, which floor and a
/// subtraction did before, and was a third of the time spent drawing.
/// Truncation rounds negative coordinates up by under 1/256 of a texel.
#[inline(always)]
fn sample(level: &Level, s: f32, t: f32, filter: bool, clamp: [bool; 2]) -> u32 {
    const ONE: f32 = 256.0;
    let (width, height) = (level.width as i32, level.height as i32);
    // Texture sides are powers of two.
    let place = |value: i32, size: i32, clamped: bool| if clamped { value.clamp(0, size - 1) } else { value & (size - 1) };
    let texels = &level.texels;
    if !filter {
        let (u, v) = ((s * ONE) as i32 >> 8, (t * ONE) as i32 >> 8);
        return texels[(place(v, height, clamp[1]) * width + place(u, width, clamp[0])) as usize];
    }
    let (s, t) = (((s - 0.5) * ONE) as i32, ((t - 0.5) * ONE) as i32);
    let (u, v) = (s >> 8, t >> 8);
    let (fu, fv) = ((s & 255) as u32, (t & 255) as u32);
    let (left, right) = (place(u, width, clamp[0]), place(u + 1, width, clamp[0]));
    let (top, bottom) = (place(v, height, clamp[1]) * width, place(v + 1, height, clamp[1]) * width);
    let texel = |row: i32, column: i32| texels[(row + column) as usize];
    mix(mix(texel(top, left), texel(top, right), fu), mix(texel(bottom, left), texel(bottom, right), fu), fv)
}

/// One channel of a combine unit, as the card's documentation defines it.
#[inline(always)]
fn combine(unit: &Combine, local: f32, other: f32, local_alpha: f32, other_alpha: f32, texture_alpha: f32) -> f32 {
    let factor = match unit.factor {
        0 => 0.0,
        1 => local,
        2 => other_alpha,
        3 => local_alpha,
        4 => texture_alpha,
        8 => 255.0,
        9 => 255.0 - local,
        10 => 255.0 - other_alpha,
        11 => 255.0 - local_alpha,
        12 => 255.0 - texture_alpha,
        _ => 255.0,
    } / 255.0;
    let value = match unit.function {
        0 => 0.0,
        1 => local,
        2 => local_alpha,
        3 => other * factor,
        4 => other * factor + local,
        5 => other * factor + local_alpha,
        6 => (other - local) * factor,
        7 => (other - local) * factor + local,
        8 => (other - local) * factor + local_alpha,
        9 => -local * factor,
        16 => -local * factor + local,
        17 => -local * factor + local_alpha,
        _ => local,
    };
    let value = value.clamp(0.0, 255.0);
    if unit.invert { 255.0 - value } else { value }
}

/// A blend factor. 2, 6 and 15 mean different things on each side: the source
/// side reads the destination's colour, and the other side the source's.
fn blend_factor(factor: u32, source: [f32; 4], destination: [f32; 3], channel: usize, for_destination: bool) -> f32 {
    let opposite = if for_destination { source[channel] } else { destination[channel] } / 255.0;
    match factor {
        0 => 0.0,
        1 => source[3] / 255.0,
        2 => opposite,
        3 => 1.0,
        4 => 1.0,
        5 => 1.0 - source[3] / 255.0,
        6 => 1.0 - opposite,
        7 => 0.0,
        15 if for_destination => source[channel] / 255.0,
        _ => 1.0,
    }
}

/// Glide's fog table holds an entry for each of these depths.
fn fog_table_depth(index: usize) -> f32 {
    2f32.powi(3 + (index >> 2) as i32) / (8 - (index & 3)) as f32
}

/// What guFogGenerateLinear writes: no fog at `near`, full fog at `far`.
pub fn linear_fog(table: &mut [u8; 64], near: f32, far: f32) {
    for (index, entry) in table.iter_mut().enumerate() {
        let amount = (fog_table_depth(index) - near) / (far - near);
        *entry = (amount.clamp(0.0, 1.0) * 255.0) as u8;
    }
}

#[inline(always)]
fn compare(function: u32, value: f32, reference: f32) -> bool {
    const LESS: u32 = 1;
    // Nearly every depth test; a branch the processor predicts, not a table.
    if function == LESS {
        return value < reference;
    }
    match function {
        0 => false,
        1 => value < reference,
        2 => value == reference,
        3 => value <= reference,
        4 => value > reference,
        5 => value != reference,
        6 => value >= reference,
        _ => true,
    }
}

/// The fog table's blend for a depth, between its entries.
fn fog_amount(table: &[u8; 64], w: f32) -> f32 {
    // Each power of two holds four entries, at 8/8, 8/7, 8/6 and 8/5 of it.
    let bits = w.max(1.0).to_bits();
    let power = (bits >> 23) as usize - 127;
    let fraction = f32::from_bits((bits & 0x7f_ffff) | (127 << 23));
    let step = [8.0 / 7.0, 8.0 / 6.0, 8.0 / 5.0].iter().filter(|entry| fraction >= **entry).count();
    let index = (power * 4 + step).min(62);
    let (near, far) = (fog_table_depth(index), fog_table_depth(index + 1));
    let fraction = ((w - near) / (far - near)).clamp(0.0, 1.0);
    table[index] as f32 + (table[index + 1] as f32 - table[index] as f32) * fraction
}

/// What a queued triangle needs from the card.
pub struct Snapshot {
    pub pipeline: Pipeline,
    pub texture: Option<Arc<Texture>>,
}

/// Triangles waiting to be drawn, in order. Drawing them together lets every
/// core work on its own rows of the screen.
#[derive(Default)]
pub struct Batch {
    snapshots: Vec<Snapshot>,
    triangles: Vec<(u32, [Vertex; 3])>,
}

impl Batch {
    pub fn push(&mut self, state: &State, vertices: [Vertex; 3]) {
        let texture = state.source.and_then(|start| state.textures.get(&start));
        let unchanged = self.snapshots.last().is_some_and(|last| {
            let same_texture = match (&last.texture, texture) {
                (Some(old), Some(new)) => Arc::ptr_eq(old, new),
                (None, None) => true,
                _ => false,
            };
            same_texture && last.pipeline == state.pipeline
        });
        if !unchanged {
            self.snapshots.push(Snapshot { pipeline: state.pipeline.clone(), texture: texture.cloned() });
        }
        self.triangles.push((self.snapshots.len() as u32 - 1, vertices));
    }

    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    /// Hands the queued triangles to another renderer, each with the number
    /// of its snapshot, and empties the queue.
    pub fn take(&mut self) -> (Vec<Snapshot>, Vec<(u32, [Vertex; 3])>) {
        (std::mem::take(&mut self.snapshots), std::mem::take(&mut self.triangles))
    }

    fn draw_band(&self, target: &mut Target) {
        for (snapshot, vertices) in &self.triangles {
            triangle(&self.snapshots[*snapshot as usize], target, vertices);
        }
    }

    /// Draws everything queued. The bands are smaller than a thread's share,
    /// because most of a frame's work falls in a few of them.
    pub fn draw(&mut self, screen: Screen) {
        const BANDS_PER_THREAD: usize = 4;
        const WORTH_THREADS: usize = 64;
        let Screen { color, depth, width, height, scale, pool } = screen;
        let threads = if self.triangles.len() < WORTH_THREADS { 1 } else { pool.threads() };
        let rows = (height / (threads * BANDS_PER_THREAD)).max(1);
        let bands = color.chunks_mut(width * rows).zip(depth.chunks_mut(width * rows)).enumerate();
        let bands: Vec<Target> = bands.map(|(band, (color, depth))| Target { color, depth, width, first_row: band * rows, rows: rows.min(height - band * rows), scale: scale as f32 }).collect();
        let bands = Mutex::new(bands);
        let work = || loop {
            // On its own line, or the lock would be held while the band draws.
            let band = bands.lock().unwrap().pop();
            let Some(mut band) = band else { break };
            self.draw_band(&mut band);
        };
        if threads == 1 {
            work();
        } else {
            pool.run(&work);
        }
        drop(bands);
        self.snapshots.clear();
        self.triangles.clear();
    }
}

/// The buffers to draw into.
pub struct Screen<'a> {
    pub color: &'a mut [u32],
    pub depth: &'a mut [f32],
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub pool: &'a super::pool::Pool,
}

/// Some rows of the screen.
struct Target<'a> {
    color: &'a mut [u32],
    depth: &'a mut [f32],
    width: usize,
    first_row: usize,
    rows: usize,
    scale: f32,
}

/// Narrows pixels `first..last` to those where `weight + n * step` isn't negative.
fn narrow(weight: f64, step: f64, first: &mut i32, last: &mut i32) {
    if step > 0.0 {
        *first = (*first).max((-weight / step).ceil() as i32);
    } else if step < 0.0 {
        *last = (*last).min((weight / -step).floor() as i32 + 1);
    } else if weight < 0.0 {
        *last = *first;
    }
}

/// Every interpolated value, mixed from the corners by these weights.
fn weighted(a: &Vertex, b: &Vertex, c: &Vertex, weights: [f32; 3]) -> [f32; 7] {
    let mix = |pick: fn(&Vertex) -> f32| weights[0] * pick(a) + weights[1] * pick(b) + weights[2] * pick(c);
    [mix(|v| v.oow), mix(|v| v.r), mix(|v| v.g), mix(|v| v.b), mix(|v| v.a), mix(|v| v.sow), mix(|v| v.tow)]
}

/// Draws the part of a triangle in these rows. Everything but texture
/// coordinates is linear on the screen; those are divided by `oow` per pixel.
fn triangle(snapshot: &Snapshot, target: &mut Target, vertices: &[Vertex; 3]) {
    let [a, b, c] = vertices;
    let state = &snapshot.pipeline;
    let scale = target.scale;
    let (band_top, band_bottom) = (target.first_row as f32, (target.first_row + target.rows) as f32);
    if (a.y.min(b.y).min(c.y) * scale) >= band_bottom || (a.y.max(b.y).max(c.y) * scale) < band_top {
        return;
    }
    let wide = scale as f64;
    let (ax, ay, bx, by, cx, cy) = (a.x as f64 * wide, a.y as f64 * wide, b.x as f64 * wide, b.y as f64 * wide, c.x as f64 * wide, c.y as f64 * wide);
    let area = (bx - ax) * (cy - ay) - (cx - ax) * (by - ay);
    if area == 0.0 || !area.is_finite() {
        return;
    }
    const CULL_NEGATIVE: u32 = 1;
    const CULL_POSITIVE: u32 = 2;
    if (state.cull == CULL_NEGATIVE && area < 0.0) || (state.cull == CULL_POSITIVE && area > 0.0) {
        return;
    }
    let clip = state.clip.map(|value| value as f64 * wide);
    let left = ax.min(bx).min(cx).floor().max(clip[0]).max(0.0) as i32;
    let right = ax.max(bx).max(cx).ceil().min(clip[2]).min(target.width as f64) as i32;
    let top = ay.min(by).min(cy).floor().max(clip[1]).max(band_top as f64) as i32;
    let bottom = ay.max(by).max(cy).ceil().min(clip[3]).min(band_bottom as f64) as i32;
    if left >= right || top >= bottom {
        return;
    }

    let inverse = 1.0 / area;
    // How each corner's weight changes per pixel.
    let (ab_x, ab_y) = ((by - cy) * inverse, (cx - bx) * inverse);
    let (bb_x, bb_y) = ((cy - ay) * inverse, (ax - cx) * inverse);
    let (cb_x, cb_y) = ((ay - by) * inverse, (bx - ax) * inverse);
    let start_x = left as f64 + 0.5;
    let start_y = top as f64 + 0.5;
    let row_a = ((bx - start_x) * (cy - start_y) - (cx - start_x) * (by - start_y)) * inverse;
    let row_b = ((cx - start_x) * (ay - start_y) - (ax - start_x) * (cy - start_y)) * inverse;
    let row_c = ((ax - start_x) * (by - start_y) - (bx - start_x) * (ay - start_y)) * inverse;
    let across = weighted(a, b, c, [ab_x as f32, bb_x as f32, cb_x as f32]);

    let texture = snapshot.texture.as_deref();
    let level = texture.map(|texture| {
        // A mip level from how many texels a pixel covers across the triangle.
        let (ua, va, ub, vb) = (a.sow / a.oow, a.tow / a.oow, b.sow / b.oow, b.tow / b.oow);
        let (uc, vc) = (c.sow / c.oow, c.tow / c.oow);
        let texel_area = ((ub - ua) * (vc - va) - (uc - ua) * (vb - va)).abs() * texture.s_scale * texture.t_scale;
        let ratio = (texel_area / area.abs() as f32).max(1.0).log2() * 0.5;
        &texture.levels[(ratio as usize).min(texture.levels.len() - 1)]
    });
    let sizes = texture.zip(level).map(|(texture, level)| {
        let top = &texture.levels[0];
        let size = top.width.max(top.height) as f32 / 256.0;
        (size * level.width as f32 / top.width as f32, size * level.height as f32 / top.height as f32)
    });
    let constant = channels(state.constant);
    let fog_color = channels(state.fog_color);
    let plain_write = state.blend == [4, 0];
    // Most of a frame is plain texture, which needs none of the steps below.
    const ALWAYS: u32 = 7;
    let color = state.color;
    let shows_texture = color.function == 3 && color.factor == 8 && color.other == 1 && !color.invert;
    let plain_texture = sizes.is_some() && shows_texture && plain_write && !state.chroma && state.alpha_test == ALWAYS && state.fog_mode & 3 == 0;

    for y in top..bottom {
        let rows_down = (y - top) as f64;
        let (wa, wb, wc) = (row_a + rows_down * ab_y, row_b + rows_down * bb_y, row_c + rows_down * cb_y);
        let (mut first, mut last) = (0, right - left);
        narrow(wa, ab_x, &mut first, &mut last);
        narrow(wb, bb_x, &mut first, &mut last);
        narrow(wc, cb_x, &mut first, &mut last);
        if first >= last {
            continue;
        }
        let along = first as f64;
        let start = [wa + along * ab_x, wb + along * bb_x, wc + along * cb_x].map(|weight| weight.max(0.0) as f32);
        let base = weighted(a, b, c, start);
        let row = (y as usize - target.first_row) * target.width + left as usize;
        for pixel in 0..last - first {
            let n = pixel as f32;
            let index = row + (first + pixel) as usize;
            let oow = base[0] + across[0] * n;
            let w = 1.0 / oow;
            const DEPTH_DISABLED: u32 = 0;
            if state.depth_mode != DEPTH_DISABLED && !compare(state.depth_function, w, target.depth[index]) {
                continue;
            }
            let mut texel_bits = 0xffff_ffff;
            if let (Some(level), Some((s_size, t_size))) = (level, sizes) {
                let s = (base[5] + across[5] * n) * w * s_size;
                let t = (base[6] + across[6] * n) * w * t_size;
                texel_bits = sample(level, s, t, state.filter, state.clamp);
            }
            if plain_texture {
                if state.color_write {
                    target.color[index] = texel_bits & 0xff_ffff;
                }
                if state.depth_write {
                    target.depth[index] = w;
                }
                continue;
            }
            let iterated = [base[1] + across[1] * n, base[2] + across[2] * n, base[3] + across[3] * n, base[4] + across[4] * n];
            let texel = channels(texel_bits);
            if state.chroma {
                let value = ((texel[0] as u32) << 16) | ((texel[1] as u32) << 8) | texel[2] as u32;
                if value == state.chroma_value & 0xff_ffff {
                    continue;
                }
            }
            let select = |source: u32, channel: usize, local: bool| -> f32 {
                if local {
                    if source == 1 { constant[channel] } else { iterated[channel] }
                } else {
                    match source {
                        1 => texel[channel],
                        2 => constant[channel],
                        _ => iterated[channel],
                    }
                }
            };
            let local_alpha = select(state.alpha.local, 3, true);
            let other_alpha = select(state.alpha.other, 3, false);
            let alpha = combine(&state.alpha, local_alpha, other_alpha, local_alpha, other_alpha, texel[3]);
            if !compare(state.alpha_test, alpha, state.alpha_reference) {
                continue;
            }
            let mut color = [0.0, 0.0, 0.0, alpha];
            let (color_local_alpha, color_other_alpha) = (select(state.color.local, 3, true), select(state.color.other, 3, false));
            for channel in 0..3 {
                let local = select(state.color.local, channel, true);
                let other = select(state.color.other, channel, false);
                color[channel] = combine(&state.color, local, other, color_local_alpha, color_other_alpha, texel[3]);
            }
            const FOG_TABLE: u32 = 2;
            const FOG_ITERATED: u32 = 1;
            let fog = match state.fog_mode & 3 {
                FOG_TABLE => fog_amount(&state.fog_table, w) / 255.0,
                FOG_ITERATED => iterated[3] / 255.0,
                _ => 0.0,
            };
            if fog > 0.0 {
                for channel in 0..3 {
                    color[channel] += (fog_color[channel] - color[channel]) * fog;
                }
            }
            if state.color_write {
                let mut result = 0u32;
                if plain_write {
                    for value in &color[..3] {
                        result = (result << 8) | value.clamp(0.0, 255.0) as u32;
                    }
                } else {
                    let current = target.color[index];
                    let destination = [((current >> 16) & 255) as f32, ((current >> 8) & 255) as f32, (current & 255) as f32];
                    for channel in 0..3 {
                        let value = color[channel] * blend_factor(state.blend[0], color, destination, channel, false)
                            + destination[channel] * blend_factor(state.blend[1], color, destination, channel, true);
                        result = (result << 8) | value.clamp(0.0, 255.0) as u32;
                    }
                }
                target.color[index] = result;
            }
            if state.depth_write {
                target.depth[index] = w;
            }
        }
    }
}
