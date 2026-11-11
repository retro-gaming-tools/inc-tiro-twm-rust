//! DirectDraw, as COM objects in guest memory backed by plain buffers: 16 bits a
//! pixel, or 8 bits with a palette when the game sets such a display mode.
//! Everything is software here: a frame is whatever the primary surface holds.

use crate::cpu::{Cpu, GuestFn};
use std::collections::HashMap;

const DD_OK: u32 = 0;
const DDERR_GENERIC: u32 = 0x8000_4005;
const SURFACE_DESC_SIZE: u32 = 0x6c;
const PIXEL_FORMAT_SIZE: u32 = 32;
const DESC_PIXEL_FORMAT: u32 = 0x48;
const DESC_CAPS: u32 = 0x68;
const CAPS_PRIMARY: u32 = 0x200;
const DDSD_ALL: u32 = 0x100f;
const BLT_COLORFILL: u32 = 0x400;
const BLT_KEYSRC: u32 = 0x8000;
const FX_FILL_COLOR: u32 = 0x50;
const FAST_SRCCOLORKEY: u32 = 1;

#[derive(Clone, Copy, Default)]
pub struct Surface {
    pub width: u32,
    pub height: u32,
    pub pixels: u32,
    pub back: u32,
    pub key: u16,
    /// Bytes a pixel: 2, or 1 for a surface that holds palette indices.
    pub bytes: u32,
}

impl Surface {
    fn at(&self, x: u32, y: u32) -> u32 {
        self.pixels + (y * self.width + x) * self.bytes
    }
}

pub struct Draw {
    pub surfaces: HashMap<u32, Surface>,
    pub primary: u32,
    pub frames: u64,
    pub last_flip: std::time::Instant,
    /// When the last frame was due to be shown.
    pub last_target: std::time::Instant,
    pub display_vtable: u32,
    pub surface_vtable: u32,
    pub clipper_vtable: u32,
    pub palette_vtable: u32,
    /// The table of the later display interfaces, whose SetDisplayMode takes
    /// two more arguments than the first one's.
    pub display2_vtable: u32,
    /// Bits a pixel of the display mode, which new surfaces take.
    pub depth: u32,
    /// Each palette object's 256 colours, as red, green and blue.
    pub palettes: HashMap<u32, Vec<[u8; 3]>>,
    /// The palette set on the primary surface.
    pub shown_palette: u32,
    /// An 8-bit frame turned into 16-bit pixels for the window.
    pub converted: Vec<u16>,
}

impl Default for Draw {
    fn default() -> Draw {
        Draw {
            surfaces: HashMap::new(),
            primary: 0,
            frames: 0,
            last_flip: std::time::Instant::now(),
            last_target: std::time::Instant::now(),
            display_vtable: 0,
            surface_vtable: 0,
            clipper_vtable: 0,
            palette_vtable: 0,
            display2_vtable: 0,
            depth: 16,
            palettes: HashMap::new(),
            shown_palette: 0,
            converted: Vec::new(),
        }
    }
}

pub type Method = (u32, u32, GuestFn);

pub fn build_vtable(cpu: &mut Cpu, name: &str, slots: u32, methods: &[Method]) -> u32 {
    let table = cpu.allocate(slots * 4);
    for slot in 0..slots {
        let offset = slot * 4;
        let method = methods.iter().find(|(at, _, _)| *at == offset);
        let label = format!("{name} method {offset:#x}");
        let address = cpu.thunk(&label, method.map(|(_, _, function)| *function));
        cpu.w32(table + offset, address);
    }
    table
}

pub fn object(cpu: &mut Cpu, vtable: u32) -> u32 {
    let at = cpu.allocate(8);
    cpu.w32(at, vtable);
    at
}

/// Most methods just succeed. The count includes `this`.
macro_rules! method {
    ($offset:expr, $arguments:expr, |$cpu:ident| $body:block) => {
        ($offset, $arguments, (|$cpu: &mut Cpu| { let result: u32 = $body; $cpu.finish(result, $arguments); }) as GuestFn)
    };
}

fn query_interface(cpu: &mut Cpu) -> u32 {
    let (this, out) = (cpu.arg(0), cpu.arg(2));
    cpu.w32(out, this);
    DD_OK
}

fn common() -> [Method; 3] {
    [
        method!(0x00, 3, |cpu| { query_interface(cpu) }),
        method!(0x04, 1, |_cpu| { 1 }),
        method!(0x08, 1, |_cpu| { 0 }),
    ]
}

fn write_pixel_format(cpu: &mut Cpu, at: u32, bytes: u32) {
    const RGB: u32 = 0x40;
    const PALETTE_INDEXED_8: u32 = 0x20;
    cpu.fill(at, PIXEL_FORMAT_SIZE, 0);
    cpu.w32(at, PIXEL_FORMAT_SIZE);
    if bytes == 1 {
        cpu.w32(at + 4, RGB | PALETTE_INDEXED_8);
        cpu.w32(at + 12, 8);
        return;
    }
    cpu.w32(at + 4, RGB);
    cpu.w32(at + 12, 16);
    cpu.w32(at + 16, 0xf800);
    cpu.w32(at + 20, 0x07e0);
    cpu.w32(at + 24, 0x001f);
}

fn read_pixel(cpu: &Cpu, surface: &Surface, x: u32, y: u32) -> u16 {
    if surface.bytes == 1 { cpu.r8(surface.at(x, y)) as u16 } else { cpu.r16(surface.at(x, y)) }
}

fn write_pixel(cpu: &mut Cpu, surface: &Surface, x: u32, y: u32, value: u16) {
    if surface.bytes == 1 { cpu.w8(surface.at(x, y), value as u8) } else { cpu.w16(surface.at(x, y), value) }
}

fn write_description(cpu: &mut Cpu, at: u32, surface: &Surface) {
    cpu.fill(at, SURFACE_DESC_SIZE, 0);
    cpu.w32(at, SURFACE_DESC_SIZE);
    cpu.w32(at + 4, DDSD_ALL);
    cpu.w32(at + 8, surface.height);
    cpu.w32(at + 12, surface.width);
    cpu.w32(at + 16, surface.width * surface.bytes);
    cpu.w32(at + 0x24, surface.pixels);
    write_pixel_format(cpu, at + DESC_PIXEL_FORMAT, surface.bytes);
}

fn new_surface(cpu: &mut Cpu, width: u32, height: u32) -> u32 {
    let vtable = cpu.host.draw.surface_vtable;
    let at = object(cpu, vtable);
    let bytes = cpu.host.draw.depth / 8;
    let size = width * height * bytes;
    let pixels = cpu.allocate(size);
    cpu.fill(pixels, size, 0);
    cpu.host.draw.surfaces.insert(at, Surface { width, height, pixels, back: 0, key: 0, bytes });
    at
}

fn create_surface(cpu: &mut Cpu) -> u32 {
    const FLAG_BACK_BUFFERS: u32 = 0x20;
    let (desc, out) = (cpu.arg(1), cpu.arg(2));
    let flags = cpu.r32(desc + 4);
    let caps = cpu.r32(desc + DESC_CAPS);
    let primary = caps & CAPS_PRIMARY != 0;
    let (mut width, mut height) = (cpu.r32(desc + 12), cpu.r32(desc + 8));
    if primary {
        (width, height) = cpu.host.display;
    }
    let surface = new_surface(cpu, width, height);
    if primary {
        cpu.host.draw.primary = surface;
        if flags & FLAG_BACK_BUFFERS != 0 {
            let back = new_surface(cpu, width, height);
            cpu.host.draw.surfaces.get_mut(&surface).unwrap().back = back;
        }
    }
    cpu.w32(out, surface);
    DD_OK
}

fn surface_of(cpu: &Cpu, at: u32) -> Surface {
    *cpu.host.draw.surfaces.get(&at).unwrap_or_else(|| panic!("no surface at {at:#x}"))
}

fn flip(cpu: &mut Cpu) -> u32 {
    let this = cpu.arg(0);
    let front = surface_of(cpu, this);
    if front.back != 0 {
        let back = surface_of(cpu, front.back);
        cpu.host.draw.surfaces.get_mut(&this).unwrap().pixels = back.pixels;
        cpu.host.draw.surfaces.get_mut(&front.back).unwrap().pixels = front.pixels;
    }
    cpu.host.draw.frames += 1;
    wait_for_refresh(cpu);
    super::video::pump(cpu);
    DD_OK
}

/// A real Flip waits for the display, and game timing depends on that.
pub fn wait_for_refresh(cpu: &mut Cpu) {
    use std::time::{Duration, Instant};
    const STALE: Duration = Duration::from_millis(250);
    let (refresh, vsync, shown) = match cpu.host.video.as_ref() {
        Some(video) => (video.refresh(), video.vsync(), video.last_present()),
        None => (super::video::DEFAULT_REFRESH, false, cpu.host.draw.last_flip),
    };
    let period = cpu.host.frame_period.filter(|(_, set)| set.elapsed() < STALE).map(|(period, _)| period);
    let interval = period.unwrap_or(refresh);
    cpu.host.paced_since_limiter = true;
    let target = if vsync {
        // Presenting waits for the next refresh, so aiming half a refresh
        // early rounds the period to a whole number of refreshes, and every
        // frame stays on screen as long as the one before. The second bound
        // keeps the pace if presenting stops waiting, as some drivers do for a
        // hidden window.
        (shown + interval.saturating_sub(refresh / 2)).max(cpu.host.draw.last_target + interval.mul_f32(0.9))
    } else {
        cpu.host.draw.last_flip + interval
    };
    let now = Instant::now();
    if target > now {
        super::sleep(cpu, target - now);
    }
    cpu.host.draw.last_target = target;
    let finished = Instant::now();
    let frame = finished - cpu.host.draw.last_flip;
    cpu.host.draw.last_flip = finished;
    if cpu.trace_filter.as_deref() == Some("frames") {
        println!("frame took {:.1} ms", frame.as_secs_f64() * 1000.0);
    }
    // Kept for the script's `fps` step; a run with no script never reads them.
    const KEPT: usize = 1 << 16;
    if cpu.host.shown_frames.len() < KEPT && !cpu.host.script.steps.is_empty() {
        cpu.host.shown_frames.push(frame.as_secs_f32() * 1000.0);
    }
    if let Some(video) = cpu.host.video.as_mut() {
        video.record_frame(frame.as_secs_f32() * 1000.0);
    }
}

fn rectangle(cpu: &Cpu, at: u32, surface: &Surface) -> (u32, u32, u32, u32) {
    if at == 0 {
        (0, 0, surface.width, surface.height)
    } else {
        (cpu.r32(at), cpu.r32(at + 4), cpu.r32(at + 8), cpu.r32(at + 12))
    }
}

fn copy_pixels(cpu: &mut Cpu, target: &Surface, (x, y): (u32, u32), source: &Surface, (left, top, right, bottom): (u32, u32, u32, u32), key: Option<u16>) {
    for row in 0..bottom.saturating_sub(top) {
        if y + row >= target.height || top + row >= source.height {
            break;
        }
        for column in 0..right.saturating_sub(left) {
            if x + column >= target.width || left + column >= source.width {
                break;
            }
            let value = read_pixel(cpu, source, left + column, top + row);
            if key != Some(value) {
                write_pixel(cpu, target, x + column, y + row, value);
            }
        }
    }
}

fn blt(cpu: &mut Cpu) -> u32 {
    let (this, target_rect, source_at, source_rect, flags, fx) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
    let target = surface_of(cpu, this);
    let (left, top, right, bottom) = rectangle(cpu, target_rect, &target);
    if flags & BLT_COLORFILL != 0 {
        let color = cpu.r32(fx + FX_FILL_COLOR) as u16;
        for y in top..bottom.min(target.height) {
            for x in left..right.min(target.width) {
                write_pixel(cpu, &target, x, y, color);
            }
        }
        return DD_OK;
    }
    if source_at != 0 {
        let source = surface_of(cpu, source_at);
        let area = rectangle(cpu, source_rect, &source);
        let key = (flags & BLT_KEYSRC != 0).then_some(source.key);
        copy_pixels(cpu, &target, (left, top), &source, area, key);
    }
    DD_OK
}

fn blt_fast(cpu: &mut Cpu) -> u32 {
    let (this, x, y, source_at, source_rect, flags) = (cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3), cpu.arg(4), cpu.arg(5));
    let target = surface_of(cpu, this);
    let source = surface_of(cpu, source_at);
    let area = rectangle(cpu, source_rect, &source);
    let key = (flags & FAST_SRCCOLORKEY != 0).then_some(source.key);
    copy_pixels(cpu, &target, (x, y), &source, area, key);
    DD_OK
}

/// Copies PALETTEENTRY values, which are red, green, blue and flags, into a palette.
fn set_palette_entries(cpu: &mut Cpu, palette: u32, start: u32, count: u32, entries: u32) {
    for index in 0..count.min(256u32.saturating_sub(start)) {
        let at = entries + index * 4;
        let color = [cpu.r8(at), cpu.r8(at + 1), cpu.r8(at + 2)];
        if let Some(colors) = cpu.host.draw.palettes.get_mut(&palette) {
            colors[(start + index) as usize] = color;
        }
    }
}

/// The first display interface hands out the later ones through QueryInterface.
/// They are the same object here, behind a table whose SetDisplayMode takes the
/// refresh rate and flags as well.
fn query_display(cpu: &mut Cpu) -> u32 {
    const IID_DIRECT_DRAW: u32 = 0x6c14_db80;
    let (this, id, out) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
    let object_at = if cpu.r32(id) == IID_DIRECT_DRAW {
        this
    } else {
        let vtable = cpu.host.draw.display2_vtable;
        object(cpu, vtable)
    };
    cpu.w32(out, object_at);
    DD_OK
}

fn set_display_mode(cpu: &mut Cpu) -> u32 {
    let (width, height, depth) = (cpu.arg(1), cpu.arg(2), cpu.arg(3));
    if depth != 8 && depth != 16 {
        return DDERR_GENERIC;
    }
    cpu.host.display = (width, height);
    cpu.host.draw.depth = depth;
    DD_OK
}

/// Calls the game back once for each display mode there is: the game's own
/// screen size, in 8 and 16 bits a pixel.
fn enumerate_modes(cpu: &mut Cpu) -> u32 {
    let (context, callback) = (cpu.arg(3), cpu.arg(4));
    let description = cpu.allocate(SURFACE_DESC_SIZE);
    for bytes in [1, 2] {
        let mode = Surface { width: super::GAME_WIDTH as u32, height: super::GAME_HEIGHT as u32, pixels: 0, back: 0, key: 0, bytes };
        write_description(cpu, description, &mode);
        const STOP: u32 = 0;
        if super::call_guest(cpu, callback, &[description, context]) == STOP {
            break;
        }
    }
    cpu.release(description);
    DD_OK
}

/// The frame for the window, as 16-bit pixels: where they are, and the bytes a row.
pub fn frame_for_window(cpu: &mut Cpu) -> Option<(*const u8, u32)> {
    let surface = *cpu.host.draw.surfaces.get(&cpu.host.draw.primary)?;
    if surface.bytes == 2 {
        return Some((unsafe { cpu.memory.add(surface.pixels as usize) } as *const u8, surface.width * 2));
    }
    let colors = shown_colors(cpu);
    let table: Vec<u16> = colors.iter().map(|c| (c[0] as u16 >> 3) << 11 | (c[1] as u16 >> 2) << 5 | c[2] as u16 >> 3).collect();
    let indices = cpu.bytes(surface.pixels, surface.width * surface.height);
    let converted: Vec<u16> = indices.iter().map(|&index| table[index as usize]).collect();
    cpu.host.draw.converted = converted;
    Some((cpu.host.draw.converted.as_ptr() as *const u8, surface.width * 2))
}

/// The palette on screen: the one set on the primary surface, or grey steps
/// before the game sets one.
fn shown_colors(cpu: &Cpu) -> Vec<[u8; 3]> {
    let draw = &cpu.host.draw;
    draw.palettes.get(&draw.shown_palette).cloned().unwrap_or_else(|| (0..=255).map(|v| [v; 3]).collect())
}

fn build_tables(cpu: &mut Cpu) {
    if cpu.host.draw.display_vtable != 0 {
        return;
    }
    let mut display = common().to_vec();
    display.extend([
        method!(0x10, 4, |cpu| {
            let vtable = cpu.host.draw.clipper_vtable;
            let clipper = object(cpu, vtable);
            let out = cpu.arg(2);
            cpu.w32(out, clipper);
            DD_OK
        }),
        method!(0x14, 5, |cpu| {
            let vtable = cpu.host.draw.palette_vtable;
            let palette = object(cpu, vtable);
            let (entries, out) = (cpu.arg(2), cpu.arg(3));
            cpu.host.draw.palettes.insert(palette, vec![[0; 3]; 256]);
            if entries != 0 {
                set_palette_entries(cpu, palette, 0, 256, entries);
            }
            cpu.w32(out, palette);
            DD_OK
        }),
        method!(0x18, 4, |cpu| { create_surface(cpu) }),
        method!(0x20, 5, |cpu| { enumerate_modes(cpu) }),
        method!(0x28, 1, |_cpu| { DD_OK }),
        method!(0x2c, 3, |cpu| {
            // The game's own drawing does everything: no hardware help is claimed
            // beyond plain copies and colour fills.
            const CAPS_BLT: u32 = 0x40;
            const CAPS_BLT_COLOR_FILL: u32 = 0x0400_0000;
            for at in [cpu.arg(1), cpu.arg(2)] {
                if at != 0 {
                    let size = cpu.r32(at);
                    cpu.fill(at + 4, size.saturating_sub(4), 0);
                    cpu.w32(at + 4, CAPS_BLT | CAPS_BLT_COLOR_FILL);
                }
            }
            DD_OK
        }),
        method!(0x4c, 1, |_cpu| { DD_OK }),
        method!(0x50, 3, |_cpu| { DD_OK }),
        method!(0x54, 4, |cpu| { set_display_mode(cpu) }),
        method!(0x58, 3, |_cpu| { DD_OK }),
    ]);
    display[0] = method!(0x00, 3, |cpu| { query_display(cpu) });
    let mut display2 = display.clone();
    let mode = display2.iter().position(|(offset, _, _)| *offset == 0x54).unwrap();
    display2[mode] = method!(0x54, 6, |cpu| { set_display_mode(cpu) });
    let mut surface = common().to_vec();
    surface.extend([
        method!(0x14, 6, |cpu| { blt(cpu) }),
        method!(0x1c, 6, |cpu| { blt_fast(cpu) }),
        method!(0x2c, 3, |cpu| { flip(cpu) }),
        method!(0x30, 3, |cpu| {
            let (this, out) = (cpu.arg(0), cpu.arg(2));
            let back = surface_of(cpu, this).back;
            cpu.w32(out, back);
            if back != 0 { DD_OK } else { DDERR_GENERIC }
        }),
        method!(0x34, 2, |_cpu| { DD_OK }),
        method!(0x48, 2, |_cpu| { DD_OK }),
        method!(0x54, 2, |cpu| {
            let (this, at) = (cpu.arg(0), cpu.arg(1));
            let bytes = surface_of(cpu, this).bytes;
            write_pixel_format(cpu, at, bytes);
            DD_OK
        }),
        method!(0x58, 2, |cpu| {
            let (this, at) = (cpu.arg(0), cpu.arg(1));
            let surface = surface_of(cpu, this);
            write_description(cpu, at, &surface);
            DD_OK
        }),
        method!(0x60, 1, |_cpu| { DD_OK }),
        method!(0x64, 5, |cpu| {
            let (this, at) = (cpu.arg(0), cpu.arg(2));
            let surface = surface_of(cpu, this);
            write_description(cpu, at, &surface);
            DD_OK
        }),
        method!(0x6c, 1, |_cpu| { DD_OK }),
        method!(0x70, 2, |_cpu| { DD_OK }),
        method!(0x74, 3, |cpu| {
            let (this, key) = (cpu.arg(0), cpu.arg(2));
            if key != 0 {
                let value = cpu.r32(key) as u16;
                cpu.host.draw.surfaces.get_mut(&this).unwrap().key = value;
            }
            DD_OK
        }),
        method!(0x7c, 2, |cpu| {
            let palette = cpu.arg(1);
            if palette != 0 {
                cpu.host.draw.shown_palette = palette;
            }
            DD_OK
        }),
        method!(0x80, 2, |_cpu| { DD_OK }),
    ]);
    let mut clipper = common().to_vec();
    clipper.extend([method!(0x20, 3, |_cpu| { DD_OK })]);
    let mut palette = common().to_vec();
    palette.extend([
        method!(0x10, 5, |cpu| {
            let (this, start, count, out) = (cpu.arg(0), cpu.arg(2), cpu.arg(3), cpu.arg(4));
            let colors = cpu.host.draw.palettes.get(&this).cloned().unwrap_or_default();
            for (index, color) in colors.iter().enumerate().skip(start as usize).take(count as usize) {
                let at = out + (index as u32 - start) * 4;
                cpu.w32(at, u32::from_le_bytes([color[0], color[1], color[2], 0]));
            }
            DD_OK
        }),
        method!(0x18, 5, |cpu| {
            let (this, start, count, entries) = (cpu.arg(0), cpu.arg(2), cpu.arg(3), cpu.arg(4));
            set_palette_entries(cpu, this, start, count, entries);
            DD_OK
        }),
    ]);

    cpu.host.draw.display_vtable = build_vtable(cpu, "IDirectDraw", 23, &display);
    cpu.host.draw.display2_vtable = build_vtable(cpu, "IDirectDraw2", 24, &display2);
    cpu.host.draw.surface_vtable = build_vtable(cpu, "IDirectDrawSurface", 36, &surface);
    cpu.host.draw.clipper_vtable = build_vtable(cpu, "IDirectDrawClipper", 9, &clipper);
    cpu.host.draw.palette_vtable = build_vtable(cpu, "IDirectDrawPalette", 7, &palette);
}

/// The primary surface's pixels as RGB, for a screenshot or the window.
pub fn current_frame(cpu: &Cpu) -> Option<(u32, u32, Vec<u8>)> {
    let from_card = super::glide::front_from_card(cpu);
    let front = from_card.as_ref().map(|(width, height, pixels)| (*width, *height, &pixels[..]));
    if let Some((width, height, pixels)) = front.or_else(|| super::glide::front(cpu)) {
        let bytes = pixels.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8]).collect();
        return Some((width as u32, height as u32, bytes));
    }
    let surface = cpu.host.draw.surfaces.get(&cpu.host.draw.primary)?;
    if surface.bytes == 1 {
        let colors = shown_colors(cpu);
        let indices = cpu.bytes(surface.pixels, surface.width * surface.height);
        return Some((surface.width, surface.height, indices.iter().flat_map(|&index| colors[index as usize]).collect()));
    }
    let mut pixels = Vec::with_capacity((surface.width * surface.height * 3) as usize);
    for at in 0..surface.width * surface.height {
        let value = cpu.r16(surface.pixels + at * 2) as u32;
        pixels.push(((value >> 11) * 255 / 31) as u8);
        pixels.push((((value >> 5) & 63) * 255 / 63) as u8);
        pixels.push(((value & 31) * 255 / 31) as u8);
    }
    Some((surface.width, surface.height, pixels))
}

pub fn find(name: &str) -> Option<GuestFn> {
    super::table! { name;
        "DirectDrawCreate" 3 |cpu| {
            build_tables(cpu);
            let vtable = cpu.host.draw.display_vtable;
            let at = object(cpu, vtable);
            let out = cpu.arg(1);
            cpu.w32(out, at);
            DD_OK
        }
        "DirectDrawCreateClipper" 3 |cpu| {
            build_tables(cpu);
            let vtable = cpu.host.draw.clipper_vtable;
            let at = object(cpu, vtable);
            let out = cpu.arg(1);
            cpu.w32(out, at);
            DD_OK
        }
    }
}
