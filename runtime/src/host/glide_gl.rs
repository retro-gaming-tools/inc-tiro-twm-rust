//! Glide drawn by the graphics card, through OpenGL 3.3.
//!
//! It does what `glide_raster` does on the processor, with the same rules, so
//! either can stand in for the other: positions are already on the screen, so
//! nothing is projected, and everything is interpolated straight across the
//! screen. The fragment shader divides texture coordinates by `oow`, runs the
//! card's combine units, alpha test, chroma key and fog table, and writes
//! `oow` as the depth, where nearer is larger.
//!
//! Buffers keep the game's top row first (see `gl`). 2D that the game writes
//! to the frame buffer is uploaded as a 640x480 texture and drawn over the
//! buffer, leaving out the pixels the game didn't write.

use super::gl::{self, Cover, Gl};
use super::glide_raster::{Pipeline, Snapshot, Texture, Vertex};
use super::{GAME_HEIGHT, GAME_WIDTH};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Arc;

/// x, y, oow, then red, green, blue, alpha, then sow, tow.
const FLOATS: usize = 9;
/// The largest area kept from under the pointer, in game pixels.
const POINTER_AREA: (i32, i32) = (48, 64);

const VERTEX: &str = "#version 330 core
in vec3 a_position;
in vec4 a_color;
in vec2 a_texture;
uniform vec2 u_texture_scale;
out vec4 v_color;
out vec3 v_texture;
void main() {
    gl_Position = vec4(a_position.x / 320.0 - 1.0, a_position.y / 240.0 - 1.0, 0.0, 1.0);
    v_color = a_color;
    v_texture = vec3(a_texture * u_texture_scale, a_position.z);
}
";

const FRAGMENT: &str = "#version 330 core
uniform sampler2D u_texture;
uniform int u_textured;
// Each combine unit: its mode, factor, local source and other source.
uniform ivec4 u_color;
uniform ivec4 u_alpha;
uniform int u_color_invert;
uniform int u_alpha_invert;
uniform vec4 u_constant;
uniform int u_chroma;
uniform vec3 u_chroma_value;
uniform int u_alpha_test;
uniform float u_alpha_reference;
uniform int u_fog_mode;
uniform vec3 u_fog_color;
uniform float u_fog_table[64];
in vec4 v_color;
in vec3 v_texture;
out vec4 o_color;

vec4 combine(ivec4 unit, int invert, vec4 local, vec4 other, float local_alpha, float other_alpha, float texture_alpha) {
    vec4 scale;
    int pick = unit.y;
    if (pick == 0) scale = vec4(0.0);
    else if (pick == 1) scale = local;
    else if (pick == 2) scale = vec4(other_alpha);
    else if (pick == 3) scale = vec4(local_alpha);
    else if (pick == 4) scale = vec4(texture_alpha);
    else if (pick == 9) scale = 255.0 - local;
    else if (pick == 10) scale = vec4(255.0 - other_alpha);
    else if (pick == 11) scale = vec4(255.0 - local_alpha);
    else if (pick == 12) scale = vec4(255.0 - texture_alpha);
    else scale = vec4(255.0);
    scale /= 255.0;
    vec4 value;
    int mode = unit.x;
    if (mode == 0) value = vec4(0.0);
    else if (mode == 1) value = local;
    else if (mode == 2) value = vec4(local_alpha);
    else if (mode == 3) value = other * scale;
    else if (mode == 4) value = other * scale + local;
    else if (mode == 5) value = other * scale + local_alpha;
    else if (mode == 6) value = (other - local) * scale;
    else if (mode == 7) value = (other - local) * scale + local;
    else if (mode == 8) value = (other - local) * scale + local_alpha;
    else if (mode == 9) value = -local * scale;
    else if (mode == 16) value = -local * scale + local;
    else if (mode == 17) value = -local * scale + local_alpha;
    else value = local;
    value = clamp(value, 0.0, 255.0);
    return invert != 0 ? 255.0 - value : value;
}

vec4 local_of(int source) {
    return source == 1 ? u_constant : v_color;
}

vec4 other_of(int source, vec4 texel) {
    return source == 1 ? texel : (source == 2 ? u_constant : v_color);
}

bool passes(int test, float value, float reference) {
    if (test == 0) return false;
    if (test == 1) return value < reference;
    if (test == 2) return value == reference;
    if (test == 3) return value <= reference;
    if (test == 4) return value > reference;
    if (test == 5) return value != reference;
    if (test == 6) return value >= reference;
    return true;
}

// The table holds four entries for each power of two, at 8/8, 8/7, 8/6 and
// 8/5 of it.
float table_depth(int index) {
    return exp2(float(3 + (index >> 2))) / float(8 - (index & 3));
}

float fog_amount(float w) {
    w = max(w, 1.0);
    float power = floor(log2(w));
    float part = w / exp2(power);
    int step = int(part >= 8.0 / 7.0) + int(part >= 8.0 / 6.0) + int(part >= 8.0 / 5.0);
    int index = clamp(int(power) * 4 + step, 0, 62);
    float near = table_depth(index);
    float far = table_depth(index + 1);
    float between = clamp((w - near) / (far - near), 0.0, 1.0);
    return mix(u_fog_table[index], u_fog_table[index + 1], between);
}

void main() {
    float oow = v_texture.z;
    float w = 1.0 / oow;
    vec4 texel = vec4(255.0);
    if (u_textured != 0) {
        texel = texture(u_texture, v_texture.xy * w) * 255.0;
    }
    if (u_chroma != 0 && all(equal(floor(texel.rgb + 0.5), u_chroma_value))) {
        discard;
    }
    float alpha_local = local_of(u_alpha.z).a;
    float alpha_other = other_of(u_alpha.w, texel).a;
    float alpha = combine(u_alpha, u_alpha_invert, vec4(alpha_local), vec4(alpha_other), alpha_local, alpha_other, texel.a).a;
    if (!passes(u_alpha_test, alpha, u_alpha_reference)) {
        discard;
    }
    vec4 color_local = local_of(u_color.z);
    vec4 color_other = other_of(u_color.w, texel);
    vec3 color = combine(u_color, u_color_invert, color_local, color_other, color_local.a, color_other.a, texel.a).rgb;
    float fog = 0.0;
    int fog_mode = u_fog_mode & 3;
    if (fog_mode == 2) fog = fog_amount(w) / 255.0;
    else if (fog_mode == 1) fog = v_color.a / 255.0;
    if (fog > 0.0) {
        color += (u_fog_color - color) * fog;
    }
    o_color = vec4(color, alpha) / 255.0;
    gl_FragDepth = clamp(oow, 0.0, 1.0);
}
";

struct Uniforms {
    texture_scale: i32,
    textured: i32,
    color: i32,
    alpha: i32,
    color_invert: i32,
    alpha_invert: i32,
    constant: i32,
    chroma: i32,
    chroma_value: i32,
    alpha_test: i32,
    alpha_reference: i32,
    fog_mode: i32,
    fog_color: i32,
    fog_table: i32,
}

/// A texture that can be drawn to.
struct Target {
    framebuffer: u32,
    texture: u32,
}

impl Target {
    fn new(gl: &Gl, width: i32, height: i32, filter: i32, depth: Option<u32>) -> Result<Target, String> {
        let texture = gl.texture(filter);
        let mut framebuffer = 0;
        unsafe {
            (gl.tex_image)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            (gl.gen_framebuffers)(1, &mut framebuffer);
            (gl.bind_framebuffer)(gl::FRAMEBUFFER, framebuffer);
            (gl.framebuffer_texture)(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);
            if let Some(depth) = depth {
                (gl.framebuffer_texture)(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth, 0);
            }
            let status = (gl.check_framebuffer)(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                return Err(format!("a {width}x{height} buffer can't be drawn to (status {status:#x})"));
            }
            (gl.clear_color)(0.0, 0.0, 0.0, 1.0);
            (gl.color_mask)(1, 1, 1, 1);
            (gl.disable)(gl::SCISSOR_TEST);
            (gl.clear)(gl::COLOR_BUFFER_BIT);
        }
        Ok(Target { framebuffer, texture })
    }
}

pub struct Gpu {
    gl: &'static Gl,
    width: i32,
    height: i32,
    scale: i32,
    /// The two buffers the game swaps, which share one depth buffer.
    buffers: [Target; 2],
    /// The last 3D frame without its pointer (see `Glide::scene`).
    scene: Target,
    /// What was under the pointer before it was drawn.
    pointer: Target,
    /// A buffer at the game's own size, for reading pixels back.
    small: Target,
    /// The game's 2D, as it wrote it to the frame buffer.
    flat: u32,
    cover: Cover,
    program: u32,
    uniforms: Uniforms,
    array: u32,
    buffer: u32,
    /// Textures on the card, by the address of the decoded texture, which is
    /// kept here so that the address stays its own.
    textures: HashMap<usize, (Arc<Texture>, u32)>,
    vertices: Vec<f32>,
}

fn rgb(color: u32) -> [f32; 3] {
    [((color >> 16) & 255) as f32, ((color >> 8) & 255) as f32, (color & 255) as f32]
}

/// Glide's depth tests compare `w`, and the depth buffer holds `1/w`, so
/// less and greater change places.
fn depth_function(function: u32) -> u32 {
    const LESS: u32 = 0x201;
    const EQUAL: u32 = 0x202;
    const LEQUAL: u32 = 0x203;
    const GREATER: u32 = 0x204;
    const NOTEQUAL: u32 = 0x205;
    const GEQUAL: u32 = 0x206;
    match function {
        0 => gl::NEVER,
        1 => GREATER,
        2 => EQUAL,
        3 => GEQUAL,
        4 => LESS,
        5 => NOTEQUAL,
        6 => LEQUAL,
        _ => gl::ALWAYS,
    }
}

/// Glide's blend factors, as `glide_raster::blend_factor` reads them: there
/// is no destination alpha, which counts as 1.
fn blend_factor(factor: u32, for_destination: bool) -> u32 {
    match factor {
        0 => gl::ZERO,
        1 => gl::SRC_ALPHA,
        2 => if for_destination { gl::SRC_COLOR } else { gl::DST_COLOR },
        5 => gl::ONE_MINUS_SRC_ALPHA,
        6 => if for_destination { gl::ONE_MINUS_SRC_COLOR } else { gl::ONE_MINUS_DST_COLOR },
        7 => gl::ZERO,
        15 if for_destination => gl::SRC_COLOR,
        _ => gl::ONE,
    }
}

impl Gpu {
    /// Sets up the buffers and the shader for a picture `scale` times the game's.
    pub fn new(gl: &'static Gl, scale: usize) -> Result<Gpu, String> {
        let scale = scale as i32;
        let (width, height) = (GAME_WIDTH as i32 * scale, GAME_HEIGHT as i32 * scale);
        let program = gl.program(VERTEX, FRAGMENT, &[c"a_position", c"a_color", c"a_texture"])?;
        let uniform = |name| gl.uniform(program, name);
        let uniforms = Uniforms {
            texture_scale: uniform(c"u_texture_scale"),
            textured: uniform(c"u_textured"),
            color: uniform(c"u_color"),
            alpha: uniform(c"u_alpha"),
            color_invert: uniform(c"u_color_invert"),
            alpha_invert: uniform(c"u_alpha_invert"),
            constant: uniform(c"u_constant"),
            chroma: uniform(c"u_chroma"),
            chroma_value: uniform(c"u_chroma_value"),
            alpha_test: uniform(c"u_alpha_test"),
            alpha_reference: uniform(c"u_alpha_reference"),
            fog_mode: uniform(c"u_fog_mode"),
            fog_color: uniform(c"u_fog_color"),
            fog_table: uniform(c"u_fog_table"),
        };
        let depth = gl.texture(gl::NEAREST);
        let flat = gl.texture(gl::NEAREST);
        let (mut array, mut buffer) = (0, 0);
        unsafe {
            (gl.bind_texture)(gl::TEXTURE_2D, depth);
            (gl.tex_image)(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT32F as i32, width, height, 0, gl::DEPTH_COMPONENT, gl::FLOAT, std::ptr::null());
            (gl.bind_texture)(gl::TEXTURE_2D, flat);
            (gl.tex_image)(gl::TEXTURE_2D, 0, gl::RGB8 as i32, GAME_WIDTH as i32, GAME_HEIGHT as i32, 0, gl::RGB, gl::UNSIGNED_SHORT_5_6_5, std::ptr::null());
            (gl.pixel_store)(gl::UNPACK_ALIGNMENT, 1);
            (gl.pixel_store)(gl::PACK_ALIGNMENT, 1);
            (gl.gen_vertex_arrays)(1, &mut array);
            (gl.bind_vertex_array)(array);
            (gl.gen_buffers)(1, &mut buffer);
            (gl.bind_buffer)(gl::ARRAY_BUFFER, buffer);
            let stride = (FLOATS * 4) as i32;
            for (location, (count, first)) in [(3, 0), (4, 3), (2, 7)].into_iter().enumerate() {
                (gl.enable_attribute)(location as u32);
                (gl.attribute_pointer)(location as u32, count, gl::FLOAT, 0, stride, (first * 4) as *const c_void);
            }
            (gl.use_program)(program);
            (gl.uniform_1i)(gl.uniform(program, c"u_texture"), 0);
        }
        let buffers = [Target::new(gl, width, height, gl::LINEAR, Some(depth))?, Target::new(gl, width, height, gl::LINEAR, Some(depth))?];
        let gpu = Gpu {
            gl,
            width,
            height,
            scale,
            buffers,
            scene: Target::new(gl, width, height, gl::NEAREST, None)?,
            pointer: Target::new(gl, POINTER_AREA.0 * scale, POINTER_AREA.1 * scale, gl::NEAREST, None)?,
            small: Target::new(gl, GAME_WIDTH as i32, GAME_HEIGHT as i32, gl::NEAREST, None)?,
            flat,
            cover: Cover::new(gl)?,
            program,
            uniforms,
            array,
            buffer,
            textures: HashMap::new(),
            vertices: Vec::new(),
        };
        let error = unsafe { (gl.get_error)() };
        if error != 0 {
            return Err(format!("OpenGL error {error:#x} while setting up"));
        }
        Ok(gpu)
    }

    /// The buffer's picture, for the window.
    pub fn texture(&self, buffer: usize) -> u32 {
        self.buffers[buffer].texture
    }

    /// Draws to all of a buffer.
    fn aim(&self, framebuffer: u32, width: i32, height: i32) {
        unsafe {
            (self.gl.bind_framebuffer)(gl::FRAMEBUFFER, framebuffer);
            (self.gl.viewport)(0, 0, width, height);
        }
    }

    /// Copies an area between buffers: x, y, width and height in each.
    fn copy(&self, from: u32, source: [i32; 4], to: u32, target: [i32; 4]) {
        let gl = self.gl;
        unsafe {
            (gl.bind_framebuffer)(gl::READ_FRAMEBUFFER, from);
            (gl.bind_framebuffer)(gl::DRAW_FRAMEBUFFER, to);
            (gl.disable)(gl::SCISSOR_TEST);
            (gl.color_mask)(1, 1, 1, 1);
            let [sx, sy, sw, sh] = source;
            let [tx, ty, tw, th] = target;
            (gl.blit_framebuffer)(sx, sy, sx + sw, sy + sh, tx, ty, tx + tw, ty + th, gl::COLOR_BUFFER_BIT, gl::NEAREST as u32);
        }
    }

    fn whole(&self) -> [i32; 4] {
        [0, 0, self.width, self.height]
    }

    /// The card's copy of a decoded texture, made the first time it's drawn.
    fn upload(&mut self, texture: &Arc<Texture>) -> u32 {
        let key = Arc::as_ptr(texture) as usize;
        if let Some((_, name)) = self.textures.get(&key) {
            return *name;
        }
        let gl = self.gl;
        let name = gl.texture(gl::LINEAR);
        unsafe {
            for (level, data) in texture.levels.iter().enumerate() {
                // 0xAARRGGBB in memory is blue first.
                (gl.tex_image)(gl::TEXTURE_2D, level as i32, gl::RGBA8 as i32, data.width as i32, data.height as i32, 0, gl::BGRA, gl::UNSIGNED_BYTE, data.texels.as_ptr() as *const c_void);
            }
            (gl.tex_parameter)(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, texture.levels.len() as i32 - 1);
            // The rasterizer takes the sharper of the two levels a pixel
            // falls between. Half a level towards it looks the same from
            // here, with no line where one level gives way to the next.
            (gl.tex_parameter_float)(gl::TEXTURE_2D, gl::TEXTURE_LOD_BIAS, -0.5);
        }
        self.textures.insert(key, (Arc::clone(texture), name));
        name
    }

    /// Drops the card's copies of textures the game has replaced. Call once a frame.
    pub fn collect(&mut self) {
        let gl = self.gl;
        self.textures.retain(|_, (texture, name)| {
            let used = Arc::strong_count(texture) > 1;
            if !used {
                unsafe { (gl.delete_textures)(1, name) };
            }
            used
        });
    }

    /// Sets the card up for triangles drawn with these settings.
    fn apply(&mut self, pipeline: &Pipeline, texture: Option<&Arc<Texture>>) {
        const DEPTH_DISABLED: u32 = 0;
        let name = texture.map(|texture| self.upload(texture));
        let (gl, uniforms, scale) = (self.gl, &self.uniforms, self.scale);
        unsafe {
            let [left, top, right, bottom] = pipeline.clip.map(|edge| edge.max(0) * scale);
            (gl.enable)(gl::SCISSOR_TEST);
            (gl.scissor)(left, top, (right - left).max(0), (bottom - top).max(0));
            let write = pipeline.color_write as u8;
            (gl.color_mask)(write, write, write, write);
            // The depth is written with testing off too, which OpenGL only
            // does while the test is on.
            if pipeline.depth_mode != DEPTH_DISABLED || pipeline.depth_write {
                (gl.enable)(gl::DEPTH_TEST);
                (gl.depth_func)(if pipeline.depth_mode != DEPTH_DISABLED { depth_function(pipeline.depth_function) } else { gl::ALWAYS });
            } else {
                (gl.disable)(gl::DEPTH_TEST);
            }
            (gl.depth_mask)(pipeline.depth_write as u8);
            if pipeline.blend == [4, 0] {
                (gl.disable)(gl::BLEND);
            } else {
                (gl.enable)(gl::BLEND);
                (gl.blend_func)(blend_factor(pipeline.blend[0], false), blend_factor(pipeline.blend[1], true));
            }
            let unit = |at: i32, invert: i32, unit: &super::glide_raster::Combine| {
                (gl.uniform_4i)(at, unit.function as i32, unit.factor as i32, unit.local as i32, unit.other as i32);
                (gl.uniform_1i)(invert, unit.invert as i32);
            };
            unit(uniforms.color, uniforms.color_invert, &pipeline.color);
            unit(uniforms.alpha, uniforms.alpha_invert, &pipeline.alpha);
            let [red, green, blue] = rgb(pipeline.constant);
            (gl.uniform_4f)(uniforms.constant, red, green, blue, (pipeline.constant >> 24) as f32);
            (gl.uniform_1i)(uniforms.chroma, pipeline.chroma as i32);
            let [red, green, blue] = rgb(pipeline.chroma_value);
            (gl.uniform_3f)(uniforms.chroma_value, red, green, blue);
            (gl.uniform_1i)(uniforms.alpha_test, pipeline.alpha_test as i32);
            (gl.uniform_1f)(uniforms.alpha_reference, pipeline.alpha_reference);
            (gl.uniform_1i)(uniforms.fog_mode, pipeline.fog_mode as i32);
            let [red, green, blue] = rgb(pipeline.fog_color);
            (gl.uniform_3f)(uniforms.fog_color, red, green, blue);
            if pipeline.fog_mode & 3 == 2 {
                let table = pipeline.fog_table.map(|entry| entry as f32);
                (gl.uniform_1fv)(uniforms.fog_table, table.len() as i32, table.as_ptr());
            }
            (gl.uniform_1i)(uniforms.textured, name.is_some() as i32);
            if let (Some(name), Some(texture)) = (name, texture) {
                (gl.bind_texture)(gl::TEXTURE_2D, name);
                let (near, far) = if pipeline.filter { (gl::LINEAR, gl::LINEAR_MIPMAP_LINEAR) } else { (gl::NEAREST, gl::NEAREST_MIPMAP_NEAREST) };
                (gl.tex_parameter)(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, near);
                (gl.tex_parameter)(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, far);
                let wrap = |clamp: bool| if clamp { gl::CLAMP_TO_EDGE } else { gl::REPEAT };
                (gl.tex_parameter)(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap(pipeline.clamp[0]));
                (gl.tex_parameter)(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap(pipeline.clamp[1]));
                // Coordinates run to 256 along the texture's longer side.
                (gl.uniform_2f)(uniforms.texture_scale, 1.0 / (256.0 * texture.s_scale), 1.0 / (256.0 * texture.t_scale));
            }
        }
    }

    /// Draws queued triangles into a buffer, in order.
    pub fn draw(&mut self, buffer: usize, snapshots: &[Snapshot], triangles: &[(u32, [Vertex; 3])]) {
        const CULL_NEGATIVE: u32 = 1;
        const CULL_POSITIVE: u32 = 2;
        // Runs of triangles that share a snapshot: its number, first vertex and count.
        let mut runs: Vec<(u32, i32, i32)> = Vec::new();
        self.vertices.clear();
        for (snapshot, corners) in triangles {
            let [a, b, c] = corners;
            let area = (b.x - a.x) as f64 * (c.y - a.y) as f64 - (c.x - a.x) as f64 * (b.y - a.y) as f64;
            let cull = snapshots[*snapshot as usize].pipeline.cull;
            if area == 0.0 || !area.is_finite() || (cull == CULL_NEGATIVE && area < 0.0) || (cull == CULL_POSITIVE && area > 0.0) {
                continue;
            }
            let first = (self.vertices.len() / FLOATS) as i32;
            for v in corners {
                self.vertices.extend_from_slice(&[v.x, v.y, v.oow, v.r, v.g, v.b, v.a, v.sow, v.tow]);
            }
            match runs.last_mut() {
                Some((last, _, count)) if *last == *snapshot => *count += 3,
                _ => runs.push((*snapshot, first, 3)),
            }
        }
        if runs.is_empty() {
            return;
        }
        let gl = self.gl;
        self.aim(self.buffers[buffer].framebuffer, self.width, self.height);
        unsafe {
            (gl.use_program)(self.program);
            (gl.bind_vertex_array)(self.array);
            (gl.bind_buffer)(gl::ARRAY_BUFFER, self.buffer);
            (gl.buffer_data)(gl::ARRAY_BUFFER, (self.vertices.len() * 4) as isize, self.vertices.as_ptr() as *const c_void, gl::STREAM_DRAW);
            (gl.active_texture)(gl::TEXTURE0);
        }
        for (snapshot, first, count) in runs {
            let snapshot = &snapshots[snapshot as usize];
            self.apply(&snapshot.pipeline, snapshot.texture.as_ref());
            unsafe { (gl.draw_arrays)(gl::TRIANGLES, first, count) };
        }
    }

    /// Clears a buffer's colour, as 0xRRGGBB, its depth, as `1/w`, or both.
    pub fn clear(&mut self, buffer: usize, color: Option<u32>, depth: Option<f32>) {
        let gl = self.gl;
        self.aim(self.buffers[buffer].framebuffer, self.width, self.height);
        let mut what = 0;
        unsafe {
            (gl.disable)(gl::SCISSOR_TEST);
            if let Some(color) = color {
                let [red, green, blue] = rgb(color);
                (gl.color_mask)(1, 1, 1, 1);
                (gl.clear_color)(red / 255.0, green / 255.0, blue / 255.0, 1.0);
                what |= gl::COLOR_BUFFER_BIT;
            }
            if let Some(depth) = depth {
                (gl.depth_mask)(1);
                (gl.clear_depth)(depth.clamp(0.0, 1.0) as f64);
                what |= gl::DEPTH_BUFFER_BIT;
            }
            (gl.clear)(what);
        }
    }

    /// Draws the game's 2D over a buffer: 640x480 pixels of 5-6-5 colour, where
    /// the marker colour means the game wrote nothing.
    pub fn write_flat(&mut self, buffer: usize, pixels: &[u16]) {
        let gl = self.gl;
        self.aim(self.buffers[buffer].framebuffer, self.width, self.height);
        unsafe {
            (gl.active_texture)(gl::TEXTURE0);
            (gl.bind_texture)(gl::TEXTURE_2D, self.flat);
            (gl.tex_sub_image)(gl::TEXTURE_2D, 0, 0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32, gl::RGB, gl::UNSIGNED_SHORT_5_6_5, pixels.as_ptr() as *const c_void);
            (gl.disable)(gl::BLEND);
        }
        self.cover.draw(gl, self.flat, false, true);
    }

    /// Reads a buffer back at the game's size, for a frame buffer lock that reads.
    pub fn read_flat(&mut self, buffer: usize, pixels: &mut [u16]) {
        let small = [0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32];
        self.copy(self.buffers[buffer].framebuffer, self.whole(), self.small.framebuffer, small);
        unsafe {
            (self.gl.bind_framebuffer)(gl::READ_FRAMEBUFFER, self.small.framebuffer);
            (self.gl.read_pixels)(0, 0, small[2], small[3], gl::RGB, gl::UNSIGNED_SHORT_5_6_5, pixels.as_mut_ptr() as *mut c_void);
        }
    }

    /// A buffer's pixels as 0xRRGGBB, for a screenshot.
    pub fn read(&self, buffer: usize) -> (usize, usize, Vec<u32>) {
        let mut pixels = vec![0u32; (self.width * self.height) as usize];
        unsafe {
            (self.gl.bind_framebuffer)(gl::READ_FRAMEBUFFER, self.buffers[buffer].framebuffer);
            (self.gl.read_pixels)(0, 0, self.width, self.height, gl::BGRA, gl::UNSIGNED_BYTE, pixels.as_mut_ptr() as *mut c_void);
        }
        (self.width as usize, self.height as usize, pixels)
    }

    /// Keeps an area of a buffer, in its pixels, from before the pointer is drawn there.
    pub fn keep_under_pointer(&mut self, buffer: usize, area: [i32; 4]) {
        let [_, _, width, height] = area;
        self.copy(self.buffers[buffer].framebuffer, area, self.pointer.framebuffer, [0, 0, width, height]);
    }

    /// Makes the scene from a buffer, with the kept area put back over the pointer.
    pub fn keep_scene(&mut self, buffer: usize, under_pointer: Option<[i32; 4]>) {
        self.copy(self.buffers[buffer].framebuffer, self.whole(), self.scene.framebuffer, self.whole());
        if let Some(area) = under_pointer {
            let [_, _, width, height] = area;
            self.copy(self.pointer.framebuffer, [0, 0, width, height], self.scene.framebuffer, area);
        }
    }

    /// Starts a buffer from the scene.
    pub fn restore_scene(&mut self, buffer: usize) {
        self.copy(self.scene.framebuffer, self.whole(), self.buffers[buffer].framebuffer, self.whole());
    }
}
