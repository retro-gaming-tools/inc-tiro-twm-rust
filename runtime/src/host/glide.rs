//! Glide 2, the 3dfx API, drawn by the graphics card through OpenGL, or by a
//! software rasterizer where there's no window or no usable OpenGL.
//!
//! The game addresses a 640x480 card. Triangles are scaled up and drawn at
//! `scale` times that, so 3D is sharper than the card could manage, while 2D
//! from the linear frame buffer is scaled up by repeating pixels. A function the
//! game never calls stays a stub, sized from its decorated name.
//!
//! This file holds what both renderers share: the calls, the state, and the
//! rules for the frame buffer. `glide_gl` draws on the card and `glide_raster`
//! on the processor.

use super::glide_gl::Gpu;
use super::glide_raster::{self as raster, Batch, Combine, Managed, Screen, State, Vertex};
use crate::cpu::{Cpu, GuestFn};
use std::sync::Mutex;

use super::{GAME_HEIGHT as HEIGHT, GAME_WIDTH as WIDTH};
/// Marks 2D pixels the game didn't write, so 3D shows through them.
const UNWRITTEN: u16 = 0xf81f;
const DEFAULT_SCALE: usize = 3;
const TEXTURE_MEMORY: u32 = 4 << 20;

pub struct Glide {
    pub scale: usize,
    pub width: usize,
    pub height: usize,
    /// The graphics card's renderer. With it, the buffers below stay empty.
    gpu: Option<Gpu>,
    pub buffers: [Vec<u32>; 2],
    pub back: usize,
    pub depth: Vec<f32>,
    pub lfb: u32,
    pub frames: u64,
    /// Counts swaps and writes to the front buffer: each is a new picture to show.
    pub updates: u64,
    /// True once this frame has drawn triangles or cleared the buffer.
    drew_3d: bool,
    /// While frames draw no 3D: the last 3D frame, without its pointer. Each
    /// such frame starts from it, as a 3D frame starts from a fresh render.
    /// Incubation's load and save window draws only itself and the pointer
    /// each frame, so without this the pointer left a trail outside it.
    scene: Option<Vec<u32>>,
    /// The same for the graphics card, which keeps the scene itself.
    scene_on_card: bool,
    /// True once this frame has started from `scene`.
    restored: bool,
    /// The pixels around the pointer just before the latest 2D write of a 3D
    /// frame, which come before the pointer is drawn: x, y, width, pixels.
    under_pointer: (usize, usize, usize, Vec<u32>),
    /// The same area on the graphics card, which keeps the pixels itself: x,
    /// y, width and height.
    under_pointer_on_card: Option<[i32; 4]>,
    pub state: State,
    batch: Batch,
    pool: super::pool::Pool,
    /// For the script's `fps` step: seconds drawing, seconds on 2D, and 2D locks.
    pub spent: [f64; 3],
    last_swap: std::time::Instant,
    /// The longest frame since the `fps` step last read it, in seconds.
    pub longest_frame: f64,
}

impl Glide {
    fn new(cpu: &mut Cpu) -> Glide {
        // The loader sets this from the launcher, `--scale=N`, or the player's own.
        let scale = std::env::var("RECOMP_SCALE").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_SCALE).clamp(1, 6);
        let (width, height) = (WIDTH * scale, HEIGHT * scale);
        let lfb = cpu.allocate((WIDTH * HEIGHT * 2) as u32);
        let mut state = State::default();
        state.clip = [0, 0, WIDTH as i32, HEIGHT as i32];
        // The window has OpenGL unless the player chose the processor or the
        // machine has none. If the card can't be set up, the processor draws.
        let gpu = cpu.host.video.as_ref().and_then(|video| video.gl()).and_then(|gl| match Gpu::new(gl, scale) {
            Ok(gpu) => Some(gpu),
            Err(problem) => {
                eprintln!("Glide is drawn by the processor: {problem}");
                None
            }
        });
        println!("Glide at {width}x{height}, drawn by the {}", if gpu.is_some() { "graphics card" } else { "processor" });
        let pixels = if gpu.is_some() { 0 } else { width * height };
        Glide {
            scale,
            width,
            height,
            gpu,
            buffers: [vec![0; pixels], vec![0; pixels]],
            back: 0,
            depth: vec![0.0; pixels],
            lfb,
            frames: 0,
            updates: 0,
            drew_3d: false,
            scene: None,
            scene_on_card: false,
            restored: false,
            under_pointer: (0, 0, 0, Vec::new()),
            under_pointer_on_card: None,
            state,
            batch: Batch::default(),
            pool: super::pool::Pool::new(
                std::env::var("RECOMP_THREADS").ok().and_then(|s| s.parse().ok())
                    .or(std::thread::available_parallelism().ok().map(usize::from))
                    .unwrap_or(1),
            ),
            spent: [0.0; 3],
            last_swap: std::time::Instant::now(),
            longest_frame: 0.0,
        }
    }

    fn draw(&mut self, vertices: [Vertex; 3]) {
        self.batch.push(&self.state, vertices);
    }

    /// Draws the queued triangles. This comes before anything that touches the
    /// buffers another way.
    fn flush(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        self.drew_3d = true;
        let started = std::time::Instant::now();
        if let Some(gpu) = self.gpu.as_mut() {
            let (snapshots, triangles) = self.batch.take();
            gpu.draw(self.back, &snapshots, &triangles);
        } else {
            self.batch.draw(Screen {
                color: &mut self.buffers[self.back],
                depth: &mut self.depth,
                width: self.width,
                height: self.height,
                scale: self.scale,
                pool: &self.pool,
            });
        }
        self.spent[0] += started.elapsed().as_secs_f64();
    }

    /// A fan, as Glide draws polygons.
    fn draw_fan(&mut self, vertices: &[Vertex]) {
        for pair in 1..vertices.len().saturating_sub(1) {
            self.draw([vertices[0], vertices[pair], vertices[pair + 1]]);
        }
    }
}

/// Runs something on the Glide state, if the card is open.
fn with_state(cpu: &mut Cpu, change: impl FnOnce(&mut State)) -> u32 {
    if let Some(glide) = cpu.host.glide.as_mut() {
        change(&mut glide.state);
    }
    0
}

fn combine_arguments(cpu: &Cpu) -> Combine {
    Combine { function: cpu.arg(0), factor: cpu.arg(1), local: cpu.arg(2), other: cpu.arg(3), invert: cpu.arg(4) != 0 }
}

fn draw_triangle(cpu: &mut Cpu) -> u32 {
    let vertices = [0, 1, 2].map(|index| raster::read_vertex(cpu, cpu.arg(index)));
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.draw(vertices);
    }
    0
}

fn draw_vertex_list(cpu: &mut Cpu) -> u32 {
    let vertices = raster::read_list(cpu, cpu.arg(0), cpu.arg(1));
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.draw_fan(&vertices);
    }
    0
}

fn draw_indexed(cpu: &mut Cpu) -> u32 {
    const VERTEX_SIZE: u32 = 60;
    let (count, indices, list) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
    let vertices: Vec<Vertex> = (0..count).map(|at| raster::read_vertex(cpu, list + cpu.r32(indices + at * 4) * VERTEX_SIZE)).collect();
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.draw_fan(&vertices);
    }
    0
}

fn texture_download(cpu: &mut Cpu) -> u32 {
    let (start, info) = (cpu.arg(1), cpu.arg(3));
    if let Some(mut glide) = cpu.host.glide.take() {
        raster::download(cpu, &mut glide.state, start, info);
        cpu.host.glide = Some(glide);
    }
    0
}

/// Keeps `gu` handles apart from the card addresses the `gr` calls use.
const MANAGED_KEY: u32 = 0x8000_0000;

fn managed_allocate(cpu: &mut Cpu) -> u32 {
    let managed = Managed {
        small: cpu.arg(6),
        large: cpu.arg(7),
        aspect: cpu.arg(8),
        format: cpu.arg(4),
        clamp: [cpu.arg(9) != 0, cpu.arg(10) != 0],
        filter: cpu.arg(11) != 0 || cpu.arg(12) != 0,
    };
    let Some(glide) = cpu.host.glide.as_mut() else { return 0 };
    glide.state.managed.push(managed);
    glide.state.managed.len() as u32 - 1
}

fn managed_download(cpu: &mut Cpu) -> u32 {
    let (handle, source) = (cpu.arg(0), cpu.arg(1));
    if let Some(mut glide) = cpu.host.glide.take() {
        if let Some(texture) = glide.state.managed.get(handle as usize).copied() {
            let sizes = (texture.small, texture.large, texture.aspect, texture.format);
            raster::download_levels(cpu, &mut glide.state, MANAGED_KEY | handle, sizes, source);
        }
        cpu.host.glide = Some(glide);
    }
    0
}

fn managed_source(cpu: &mut Cpu) -> u32 {
    let handle = cpu.arg(0);
    with_state(cpu, |state| {
        if let Some(texture) = state.managed.get(handle as usize).copied() {
            state.source = Some(MANAGED_KEY | handle);
            state.clamp = texture.clamp;
            state.filter = texture.filter;
        }
    })
}

fn palette_download(cpu: &mut Cpu) -> u32 {
    const TABLE_PALETTE: u32 = 2;
    let (kind, table) = (cpu.arg(1), cpu.arg(2));
    if kind == TABLE_PALETTE {
        if let Some(mut glide) = cpu.host.glide.take() {
            raster::download_palette(cpu, &mut glide.state, table);
            cpu.host.glide = Some(glide);
        }
    }
    0
}

fn clip_window(cpu: &mut Cpu) -> u32 {
    let clip = [0, 1, 2, 3].map(|index| cpu.arg(index) as i32);
    let Some(glide) = cpu.host.glide.as_mut() else { return 0 };
    glide.state.clip = [clip[0].max(0), clip[1].max(0), clip[2].clamp(0, WIDTH as i32), clip[3].clamp(0, HEIGHT as i32)];
    0
}

/// The older one-argument way of choosing a colour combine.
fn color_combine_preset(cpu: &mut Cpu) -> u32 {
    let (function, factor, local, other) = match cpu.arg(0) {
        0 => (0, 0, 0, 0),    // Zero.
        1 => (1, 0, 1, 0),    // The constant colour.
        2 => (1, 0, 0, 0),    // The iterated colour.
        4 => (3, 8, 0, 1),    // The texture.
        5 => (3, 1, 1, 1),    // Texture times constant.
        6 => (3, 1, 0, 1),    // Texture times iterated.
        _ => (3, 8, 0, 1),
    };
    with_state(cpu, |state| state.color = Combine { function, factor, local, other, invert: false })
}

fn generate_fog(cpu: &mut Cpu) -> u32 {
    let (at, near, far) = (cpu.arg(0), f32::from_bits(cpu.arg(1)), f32::from_bits(cpu.arg(2)));
    let mut table = [0u8; 64];
    raster::linear_fog(&mut table, near, far);
    for (index, value) in table.iter().enumerate() {
        cpu.w8(at + index as u32, *value);
    }
    0
}

fn fog_table(cpu: &mut Cpu) -> u32 {
    let at = cpu.arg(0);
    let mut table = [0u8; 64];
    for (index, entry) in table.iter_mut().enumerate() {
        *entry = cpu.r8(at + index as u32);
    }
    with_state(cpu, |state| state.fog_table = table)
}

fn expand(pixel: u16) -> u32 {
    let (r, g, b) = ((pixel >> 11) as u32, ((pixel >> 5) & 63) as u32, (pixel & 31) as u32);
    ((r * 255 / 31) << 16) | ((g * 255 / 63) << 8) | (b * 255 / 31)
}

fn reduce(pixel: u32) -> u16 {
    let (r, g, b) = ((pixel >> 19) & 31, (pixel >> 10) & 63, (pixel >> 3) & 31);
    ((r << 11) | (g << 5) | b) as u16
}

const WRITE_ONLY: u32 = 1;
const FRONT_BUFFER: u32 = 0;

/// The game pixels around the pointer's hot spot its image can cover: left,
/// up, right and down.
const POINTER_REACH: [i32; 4] = [16, 16, 32, 48];

impl Glide {
    /// The pointer's area in buffer pixels: x, y, width, height.
    fn pointer_area(&self, pointer: (u32, u32)) -> (usize, usize, usize, usize) {
        let (scale, x, y) = (self.scale as i32, pointer.0 as i32, pointer.1 as i32);
        let left = (x - POINTER_REACH[0]).max(0) * scale;
        let top = (y - POINTER_REACH[1]).max(0) * scale;
        let right = ((x + POINTER_REACH[2]) * scale).min(self.width as i32);
        let bottom = ((y + POINTER_REACH[3]) * scale).min(self.height as i32);
        (left as usize, top as usize, (right - left).max(0) as usize, (bottom - top).max(0) as usize)
    }

    /// Keeps what's around the pointer before a 2D write in a 3D frame. The
    /// last such write draws the pointer, so this ends up holding what's under it.
    fn keep_under_pointer(&mut self, pointer: (u32, u32)) {
        let (x, y, width, height) = self.pointer_area(pointer);
        if let Some(gpu) = self.gpu.as_mut() {
            let area = [x as i32, y as i32, width as i32, height as i32];
            self.under_pointer_on_card = (width > 0 && height > 0).then_some(area);
            if width > 0 && height > 0 {
                gpu.keep_under_pointer(self.back, area);
            }
            return;
        }
        let buffer = &self.buffers[self.back];
        let mut pixels = Vec::with_capacity(width * height);
        for row in y..y + height {
            pixels.extend_from_slice(&buffer[row * self.width + x..row * self.width + x + width]);
        }
        self.under_pointer = (x, y, width, pixels);
    }

    /// Starts a frame that draws no 3D from the last 3D frame, without its
    /// pointer, as a fresh render would.
    fn start_from_scene(&mut self) {
        if self.restored {
            return;
        }
        self.restored = true;
        if let Some(gpu) = self.gpu.as_mut() {
            if !self.scene_on_card {
                gpu.keep_scene(1 - self.back, self.under_pointer_on_card);
                self.scene_on_card = true;
            }
            gpu.restore_scene(self.back);
            return;
        }
        if self.scene.is_none() {
            // The front buffer still shows the last 3D frame.
            let mut scene = self.buffers[1 - self.back].clone();
            let (x, y, width, ref pixels) = self.under_pointer;
            if width > 0 {
                for (row, line) in pixels.chunks(width).enumerate() {
                    let at = (y + row) * self.width + x;
                    scene[at..at + width].copy_from_slice(line);
                }
            }
            self.scene = Some(scene);
        }
        let back = self.back;
        if let Some(scene) = &self.scene {
            self.buffers[back].copy_from_slice(scene);
        }
    }

    /// The buffer a frame buffer lock names. Modal screens, such as Incubation's
    /// load and save window, draw on the front buffer and never swap.
    fn locked(&self, buffer: u32) -> usize {
        if buffer == FRONT_BUFFER { 1 - self.back } else { self.back }
    }
}

/// The linear frame buffer in guest memory, which `allocate` aligns.
fn lfb_pixels(cpu: &Cpu, glide: &Glide) -> &'static mut [u16] {
    unsafe { std::slice::from_raw_parts_mut(cpu.memory.add(glide.lfb as usize) as *mut u16, WIDTH * HEIGHT) }
}

fn lfb_lock(cpu: &mut Cpu) -> u32 {
    const INFO_POINTER: u32 = 4;
    const INFO_STRIDE: u32 = 8;
    const INFO_WRITE_MODE: u32 = 12;
    const INFO_ORIGIN: u32 = 16;
    let (kind, buffer, info) = (cpu.arg(0), cpu.arg(1), cpu.arg(5));
    if cpu.trace_filter.as_deref() == Some("lfb") {
        let glide = cpu.host.glide.as_ref();
        println!(
            "lfb lock kind {kind} buffer {buffer} mode {} origin {} frame {} 3d {} queued {}",
            cpu.arg(2),
            cpu.arg(3),
            glide.map_or(0, |g| g.frames),
            glide.is_some_and(|g| g.drew_3d),
            glide.is_some_and(|g| !g.batch.is_empty())
        );
    }
    let mut glide = cpu.host.glide.take().expect("Glide isn't open");
    glide.flush();
    let started = std::time::Instant::now();
    if kind & 1 == WRITE_ONLY && buffer != FRONT_BUFFER && !glide.drew_3d {
        glide.start_from_scene();
    }
    let locked = glide.locked(buffer);
    let pixels = lfb_pixels(cpu, &glide);
    if kind & 1 == WRITE_ONLY {
        pixels.fill(UNWRITTEN);
    } else if let Some(gpu) = glide.gpu.as_mut() {
        gpu.read_flat(locked, pixels);
    } else {
        let source = &glide.buffers[locked];
        for (y, row) in pixels.chunks_mut(WIDTH).enumerate() {
            let line = &source[y * glide.scale * glide.width..];
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = reduce(line[x * glide.scale]);
            }
        }
    }
    cpu.w32(info + INFO_POINTER, glide.lfb);
    cpu.w32(info + INFO_STRIDE, WIDTH as u32 * 2);
    cpu.w32(info + INFO_WRITE_MODE, 0);
    cpu.w32(info + INFO_ORIGIN, 0);
    glide.spent[1] += started.elapsed().as_secs_f64();
    glide.spent[2] += 1.0;
    cpu.host.glide = Some(glide);
    1
}

fn lfb_unlock(cpu: &mut Cpu) -> u32 {
    let (kind, target) = (cpu.arg(0), cpu.arg(1));
    if kind & 1 != WRITE_ONLY {
        return 1;
    }
    let mut glide = cpu.host.glide.take().expect("Glide isn't open");
    glide.flush();
    let started = std::time::Instant::now();
    let (scale, width, locked) = (glide.scale, glide.width, glide.locked(target));
    if target == FRONT_BUFFER {
        glide.updates += 1;
    } else if glide.drew_3d {
        glide.keep_under_pointer(cpu.host.pointer);
    }
    let pixels = lfb_pixels(cpu, &glide);
    if cpu.trace_filter.as_deref() == Some("lfb") {
        let written: Vec<usize> = (0..pixels.len()).filter(|&at| pixels[at] != UNWRITTEN).collect();
        let (xs, ys) = (written.iter().map(|at| at % WIDTH), written.iter().map(|at| at / WIDTH));
        let (left, right) = (xs.clone().min().unwrap_or(0), xs.max().unwrap_or(0));
        let (top, bottom) = (ys.clone().min().unwrap_or(0), ys.max().unwrap_or(0));
        println!("lfb unlock buffer {target}: {} pixels written, x {left}-{right}, y {top}-{bottom}", written.len());
    }
    if let Some(gpu) = glide.gpu.as_mut() {
        gpu.write_flat(locked, pixels);
        glide.spent[1] += started.elapsed().as_secs_f64();
        cpu.host.glide = Some(glide);
        return 1;
    }
    // Each game row covers `scale` rows of the buffer. Runs of written pixels
    // are widened once, then copied to each of those rows, and the rows are
    // shared out among the threads, since this runs at the full resolution.
    let rows_per_job = 8;
    let jobs = Mutex::new(glide.buffers[locked].chunks_mut(rows_per_job * scale * width).zip(pixels.chunks(rows_per_job * WIDTH)).collect::<Vec<_>>());
    let work = || {
        let mut wide = Vec::with_capacity(WIDTH * scale);
        loop {
            let job = jobs.lock().unwrap().pop();
            let Some((buffer, pixels)) = job else { break };
            for (lines, row) in buffer.chunks_mut(scale * width).zip(pixels.chunks(WIDTH)) {
                let mut x = 0;
                while x < row.len() {
                    if row[x] == UNWRITTEN {
                        x += 1;
                        continue;
                    }
                    let start = x;
                    while x < row.len() && row[x] != UNWRITTEN {
                        x += 1;
                    }
                    wide.clear();
                    for &value in &row[start..x] {
                        let colour = expand(value);
                        wide.extend(std::iter::repeat_n(colour, scale));
                    }
                    for line in lines.chunks_mut(width) {
                        line[start * scale..x * scale].copy_from_slice(&wide);
                    }
                }
            }
        }
    };
    glide.pool.run(&work);
    glide.spent[1] += started.elapsed().as_secs_f64();
    cpu.host.glide = Some(glide);
    1
}

fn buffer_clear(cpu: &mut Cpu) -> u32 {
    const FAR: u32 = 0xffff;
    let (color, depth) = (cpu.arg(0), cpu.arg(2) & 0xffff);
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.flush();
        // Only clearing the colour starts a fresh picture. Incubation's load
        // and save window clears just the depth each frame.
        glide.drew_3d |= glide.state.color_write;
        let (back, depth_write, color_write) = (glide.back, glide.state.depth_write, glide.state.color_write);
        if let Some(gpu) = glide.gpu.as_mut() {
            // The card's depth buffer holds 1/w, so the farthest is 0.
            let far = if depth == FAR { 0.0 } else { 1.0 / depth as f32 };
            if depth_write || color_write {
                gpu.clear(back, color_write.then_some(color & 0xff_ffff), depth_write.then_some(far));
            }
            return 0;
        }
        if glide.state.depth_write {
            glide.depth.fill(if depth == FAR { f32::INFINITY } else { depth as f32 });
        }
        if glide.state.color_write {
            let back = glide.back;
            glide.buffers[back].fill(color & 0xff_ffff);
        }
    }
    0
}

fn buffer_swap(cpu: &mut Cpu) -> u32 {
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.flush();
        glide.back = 1 - glide.back;
        if glide.drew_3d {
            glide.scene = None;
            glide.scene_on_card = false;
        }
        if let Some(gpu) = glide.gpu.as_mut() {
            gpu.collect();
        }
        glide.drew_3d = false;
        glide.restored = false;
        glide.frames += 1;
        glide.updates += 1;
        let now = std::time::Instant::now();
        let frame = (now - std::mem::replace(&mut glide.last_swap, now)).as_secs_f64();
        glide.longest_frame = glide.longest_frame.max(frame);
    }
    super::ddraw::wait_for_refresh(cpu);
    super::video::pump(cpu);
    0
}

fn window_open(cpu: &mut Cpu) -> u32 {
    let glide = Glide::new(cpu);
    if let Some(video) = cpu.host.video.as_mut().filter(|video| video.gl().is_none()) {
        video.use_glide_size(glide.width, glide.height);
    }
    cpu.host.glide = Some(glide);
    1
}

/// The picture most recently swapped to the front, as 0xRRGGBB values, when
/// the processor draws.
pub fn front(cpu: &Cpu) -> Option<(usize, usize, &[u32])> {
    let glide = cpu.host.glide.as_ref().filter(|glide| glide.gpu.is_none())?;
    Some((glide.width, glide.height, &glide.buffers[1 - glide.back]))
}

/// The same picture as a texture, when the graphics card draws.
pub fn front_texture(cpu: &Cpu) -> Option<u32> {
    let glide = cpu.host.glide.as_ref()?;
    Some(glide.gpu.as_ref()?.texture(1 - glide.back))
}

/// The same picture read back from the graphics card, for a screenshot.
pub fn front_from_card(cpu: &Cpu) -> Option<(usize, usize, Vec<u32>)> {
    let glide = cpu.host.glide.as_ref()?;
    Some(glide.gpu.as_ref()?.read(1 - glide.back))
}

macro_rules! stubs {
    ($($bytes:literal)*) => {
        fn stub(bytes: u32) -> Option<GuestFn> {
            match bytes {
                $($bytes => Some((|cpu: &mut Cpu| cpu.finish(0, $bytes / 4)) as GuestFn),)*
                _ => None,
            }
        }
    };
}
stubs!(0 4 8 12 16 20 24 28 32 36 40 44 48 52 56 60 64);

fn query_hardware(cpu: &mut Cpu) -> u32 {
    const VOODOO: u32 = 0;
    let at = cpu.arg(0);
    cpu.w32(at, 1); // num_sst
    cpu.w32(at + 4, VOODOO);
    cpu.w32(at + 8, 4); // fbRam
    cpu.w32(at + 12, 2); // fbiRev
    cpu.w32(at + 16, 1); // nTexelfx
    cpu.w32(at + 20, 0); // sliDetect
    cpu.w32(at + 24, 1); // tmuRev
    cpu.w32(at + 28, 4); // tmuRam
    1
}

pub fn find(name: &str) -> Option<GuestFn> {
    let known = super::table! { name;
        "_grSstQueryHardware@4" 1 |cpu| { query_hardware(cpu) }
        "_grSstQueryBoards@4" 1 |cpu| { let at = cpu.arg(0); cpu.w32(at, 1); 1 }
        "_grSstWinOpen@28" 7 |cpu| { window_open(cpu) }
        "_grSstScreenWidth@0" 0 |_cpu| { WIDTH as u32 }
        "_grSstScreenHeight@0" 0 |_cpu| { HEIGHT as u32 }
        "_grLfbLock@24" 6 |cpu| { lfb_lock(cpu) }
        "_grLfbUnlock@8" 2 |cpu| { lfb_unlock(cpu) }
        "_grBufferClear@12" 3 |cpu| { buffer_clear(cpu) }
        "_grBufferSwap@4" 1 |cpu| { buffer_swap(cpu) }
        "_guColorCombineFunction@4" 1 |cpu| { color_combine_preset(cpu) }
        "_guFogGenerateLinear@12" 3 |cpu| { generate_fog(cpu) }
        "_guTexAllocateMemory@60" 15 |cpu| { managed_allocate(cpu) }
        "_guTexDownloadMipMap@12" 3 |cpu| { managed_download(cpu) }
        "_guTexSource@4" 1 |cpu| { managed_source(cpu) }
        "_guTexMemReset@0" 0 |cpu| { with_state(cpu, |state| state.managed.clear()) }
        "_guTexMemQueryAvail@4" 1 |_cpu| { TEXTURE_MEMORY }
        "_grTexMinAddress@4" 1 |_cpu| { 0 }
        "_grTexMaxAddress@4" 1 |_cpu| { TEXTURE_MEMORY }
        "_grTexTextureMemRequired@8" 2 |cpu| {
            let info = cpu.arg(1);
            raster::memory_required(cpu.r32(info), cpu.r32(info + 4), cpu.r32(info + 8), cpu.r32(info + 12))
        }
        "_grTexCalcMemRequired@16" 4 |cpu| { raster::memory_required(cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3)) }
        "_grTexDownloadMipMap@16" 4 |cpu| { texture_download(cpu) }
        "_grTexDownloadTable@12" 3 |cpu| { palette_download(cpu) }
        "_grTexSource@16" 4 |cpu| { let start = cpu.arg(1); with_state(cpu, |state| state.source = Some(start)) }
        "_grTexFilterMode@12" 3 |cpu| { let filter = cpu.arg(1) != 0 || cpu.arg(2) != 0; with_state(cpu, |state| state.filter = filter) }
        "_grTexClampMode@12" 3 |cpu| { let clamp = [cpu.arg(1) != 0, cpu.arg(2) != 0]; with_state(cpu, |state| state.clamp = clamp) }
        "_grColorCombine@20" 5 |cpu| { let unit = combine_arguments(cpu); with_state(cpu, |state| state.color = unit) }
        "_grAlphaCombine@20" 5 |cpu| { let unit = combine_arguments(cpu); with_state(cpu, |state| state.alpha = unit) }
        "_grAlphaBlendFunction@16" 4 |cpu| { let blend = [cpu.arg(0), cpu.arg(1)]; with_state(cpu, |state| state.blend = blend) }
        "_grAlphaTestFunction@4" 1 |cpu| { let function = cpu.arg(0); with_state(cpu, |state| state.alpha_test = function) }
        "_grAlphaTestReferenceValue@4" 1 |cpu| { let value = (cpu.arg(0) & 255) as f32; with_state(cpu, |state| state.alpha_reference = value) }
        "_grChromakeyMode@4" 1 |cpu| { let on = cpu.arg(0) != 0; with_state(cpu, |state| state.chroma = on) }
        "_grChromakeyValue@4" 1 |cpu| { let value = cpu.arg(0); with_state(cpu, |state| state.chroma_value = value) }
        "_grConstantColorValue@4" 1 |cpu| { let value = cpu.arg(0); with_state(cpu, |state| state.constant = value) }
        "_grCullMode@4" 1 |cpu| { let mode = cpu.arg(0); with_state(cpu, |state| state.cull = mode) }
        "_grDepthBufferMode@4" 1 |cpu| { let mode = cpu.arg(0); with_state(cpu, |state| state.depth_mode = mode) }
        "_grDepthBufferFunction@4" 1 |cpu| { let function = cpu.arg(0); with_state(cpu, |state| state.depth_function = function) }
        "_grDepthMask@4" 1 |cpu| { let on = cpu.arg(0) != 0; with_state(cpu, |state| state.depth_write = on) }
        "_grColorMask@8" 2 |cpu| { let on = cpu.arg(0) != 0; with_state(cpu, |state| state.color_write = on) }
        "_grFogMode@4" 1 |cpu| { let mode = cpu.arg(0); with_state(cpu, |state| state.fog_mode = mode) }
        "_grFogColorValue@4" 1 |cpu| { let value = cpu.arg(0); with_state(cpu, |state| state.fog_color = value) }
        "_grFogTable@4" 1 |cpu| { fog_table(cpu) }
        "_grClipWindow@16" 4 |cpu| { clip_window(cpu) }
        "_grDrawTriangle@12" 3 |cpu| { draw_triangle(cpu) }
        "_grAADrawTriangle@24" 6 |cpu| { draw_triangle(cpu) }
        "_grDrawPolygonVertexList@8" 2 |cpu| { draw_vertex_list(cpu) }
        "_grDrawPlanarPolygonVertexList@8" 2 |cpu| { draw_vertex_list(cpu) }
        "_grAADrawPolygonVertexList@8" 2 |cpu| { draw_vertex_list(cpu) }
        "_grDrawPolygon@12" 3 |cpu| { draw_indexed(cpu) }
        "_grDrawPlanarPolygon@12" 3 |cpu| { draw_indexed(cpu) }
        "_grAADrawPolygon@12" 3 |cpu| { draw_indexed(cpu) }
    };
    known.or_else(|| {
        let (_, bytes) = name.rsplit_once('@')?;
        stub(bytes.parse().ok()?)
    })
}
