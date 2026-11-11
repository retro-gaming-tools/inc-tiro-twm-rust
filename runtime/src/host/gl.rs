//! OpenGL 3.3, bound by hand like SDL3: the functions the runtime calls, the
//! shader helpers, and the drawing that puts a finished picture in the window.
//!
//! Pictures are kept with the game's top row first, which OpenGL takes for the
//! bottom. Nothing is turned over until a picture is drawn to the window, so
//! reading pixels back gives rows in the game's order.

use std::ffi::{c_char, c_void, CStr};

pub const TEXTURE_2D: u32 = 0x0de1;
pub const RGB: u32 = 0x1907;
pub const RGBA: u32 = 0x1908;
pub const BGRA: u32 = 0x80e1;
pub const RGB8: u32 = 0x8051;
pub const RGBA8: u32 = 0x8058;
pub const DEPTH_COMPONENT: u32 = 0x1902;
pub const DEPTH_COMPONENT32F: u32 = 0x8cac;
pub const UNSIGNED_BYTE: u32 = 0x1401;
pub const UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
pub const FLOAT: u32 = 0x1406;
pub const NEAREST: i32 = 0x2600;
pub const LINEAR: i32 = 0x2601;
pub const NEAREST_MIPMAP_NEAREST: i32 = 0x2700;
pub const LINEAR_MIPMAP_LINEAR: i32 = 0x2703;
pub const TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const TEXTURE_WRAP_S: u32 = 0x2802;
pub const TEXTURE_WRAP_T: u32 = 0x2803;
pub const TEXTURE_MAX_LEVEL: u32 = 0x813d;
pub const TEXTURE_LOD_BIAS: u32 = 0x8501;
pub const REPEAT: i32 = 0x2901;
pub const CLAMP_TO_EDGE: i32 = 0x812f;
pub const TEXTURE0: u32 = 0x84c0;
pub const FRAMEBUFFER: u32 = 0x8d40;
pub const READ_FRAMEBUFFER: u32 = 0x8ca8;
pub const DRAW_FRAMEBUFFER: u32 = 0x8ca9;
pub const COLOR_ATTACHMENT0: u32 = 0x8ce0;
pub const DEPTH_ATTACHMENT: u32 = 0x8d00;
pub const FRAMEBUFFER_COMPLETE: u32 = 0x8cd5;
pub const COLOR_BUFFER_BIT: u32 = 0x4000;
pub const DEPTH_BUFFER_BIT: u32 = 0x100;
pub const BLEND: u32 = 0x0be2;
pub const DEPTH_TEST: u32 = 0x0b71;
pub const SCISSOR_TEST: u32 = 0x0c11;
pub const ZERO: u32 = 0;
pub const ONE: u32 = 1;
pub const SRC_COLOR: u32 = 0x300;
pub const ONE_MINUS_SRC_COLOR: u32 = 0x301;
pub const SRC_ALPHA: u32 = 0x302;
pub const ONE_MINUS_SRC_ALPHA: u32 = 0x303;
pub const DST_COLOR: u32 = 0x306;
pub const ONE_MINUS_DST_COLOR: u32 = 0x307;
pub const NEVER: u32 = 0x200;
pub const ALWAYS: u32 = 0x207;
pub const TRIANGLES: u32 = 4;
pub const ARRAY_BUFFER: u32 = 0x8892;
pub const STREAM_DRAW: u32 = 0x88e0;
pub const FRAGMENT_SHADER: u32 = 0x8b30;
pub const VERTEX_SHADER: u32 = 0x8b31;
pub const COMPILE_STATUS: u32 = 0x8b81;
pub const LINK_STATUS: u32 = 0x8b82;
pub const UNPACK_ALIGNMENT: u32 = 0x0cf5;
pub const PACK_ALIGNMENT: u32 = 0x0d05;
pub const RENDERER: u32 = 0x1f01;
pub const VERSION: u32 = 0x1f02;

macro_rules! functions {
    ($($field:ident $name:literal ($($argument:ty),*) $(-> $result:ty)?;)*) => {
        /// The OpenGL functions the runtime calls, from the window's context.
        pub struct Gl {
            $(pub $field: unsafe extern "system" fn($($argument),*) $(-> $result)?,)*
        }

        impl Gl {
            /// None if the context lacks any of them, which means it's older than 3.3.
            pub unsafe fn load(address: unsafe extern "C" fn(*const c_char) -> *mut c_void) -> Option<Gl> {
                Some(Gl {
                    $($field: {
                        let found = address(concat!($name, "\0").as_ptr() as *const c_char);
                        if found.is_null() {
                            eprintln!("this OpenGL has no {}", $name);
                            return None;
                        }
                        std::mem::transmute::<*mut c_void, unsafe extern "system" fn($($argument),*) $(-> $result)?>(found)
                    },)*
                })
            }
        }
    };
}

functions! {
    viewport "glViewport" (i32, i32, i32, i32);
    scissor "glScissor" (i32, i32, i32, i32);
    enable "glEnable" (u32);
    disable "glDisable" (u32);
    clear_color "glClearColor" (f32, f32, f32, f32);
    clear_depth "glClearDepth" (f64);
    clear "glClear" (u32);
    color_mask "glColorMask" (u8, u8, u8, u8);
    depth_mask "glDepthMask" (u8);
    depth_func "glDepthFunc" (u32);
    blend_func "glBlendFunc" (u32, u32);
    gen_textures "glGenTextures" (i32, *mut u32);
    delete_textures "glDeleteTextures" (i32, *const u32);
    bind_texture "glBindTexture" (u32, u32);
    active_texture "glActiveTexture" (u32);
    tex_image "glTexImage2D" (u32, i32, i32, i32, i32, i32, u32, u32, *const c_void);
    tex_sub_image "glTexSubImage2D" (u32, i32, i32, i32, i32, i32, u32, u32, *const c_void);
    tex_parameter "glTexParameteri" (u32, u32, i32);
    tex_parameter_float "glTexParameterf" (u32, u32, f32);
    pixel_store "glPixelStorei" (u32, i32);
    read_pixels "glReadPixels" (i32, i32, i32, i32, u32, u32, *mut c_void);
    gen_framebuffers "glGenFramebuffers" (i32, *mut u32);
    bind_framebuffer "glBindFramebuffer" (u32, u32);
    framebuffer_texture "glFramebufferTexture2D" (u32, u32, u32, u32, i32);
    check_framebuffer "glCheckFramebufferStatus" (u32) -> u32;
    blit_framebuffer "glBlitFramebuffer" (i32, i32, i32, i32, i32, i32, i32, i32, u32, u32);
    create_shader "glCreateShader" (u32) -> u32;
    shader_source "glShaderSource" (u32, i32, *const *const c_char, *const i32);
    compile_shader "glCompileShader" (u32);
    get_shader "glGetShaderiv" (u32, u32, *mut i32);
    shader_log "glGetShaderInfoLog" (u32, i32, *mut i32, *mut c_char);
    create_program "glCreateProgram" () -> u32;
    attach_shader "glAttachShader" (u32, u32);
    bind_attribute "glBindAttribLocation" (u32, u32, *const c_char);
    link_program "glLinkProgram" (u32);
    get_program "glGetProgramiv" (u32, u32, *mut i32);
    program_log "glGetProgramInfoLog" (u32, i32, *mut i32, *mut c_char);
    use_program "glUseProgram" (u32);
    uniform_location "glGetUniformLocation" (u32, *const c_char) -> i32;
    uniform_1i "glUniform1i" (i32, i32);
    uniform_1f "glUniform1f" (i32, f32);
    uniform_2f "glUniform2f" (i32, f32, f32);
    uniform_3f "glUniform3f" (i32, f32, f32, f32);
    uniform_4f "glUniform4f" (i32, f32, f32, f32, f32);
    uniform_4i "glUniform4i" (i32, i32, i32, i32, i32);
    uniform_1fv "glUniform1fv" (i32, i32, *const f32);
    gen_vertex_arrays "glGenVertexArrays" (i32, *mut u32);
    bind_vertex_array "glBindVertexArray" (u32);
    gen_buffers "glGenBuffers" (i32, *mut u32);
    bind_buffer "glBindBuffer" (u32, u32);
    buffer_data "glBufferData" (u32, isize, *const c_void, u32);
    enable_attribute "glEnableVertexAttribArray" (u32);
    attribute_pointer "glVertexAttribPointer" (u32, i32, u32, u8, i32, *const c_void);
    draw_arrays "glDrawArrays" (u32, i32, i32);
    get_string "glGetString" (u32) -> *const u8;
    get_error "glGetError" () -> u32;
}

impl Gl {
    /// A text the driver describes itself with, such as its renderer's name.
    pub fn string(&self, name: u32) -> String {
        let text = unsafe { (self.get_string)(name) };
        if text.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(text as *const c_char) }.to_string_lossy().into_owned()
    }

    fn shader(&self, kind: u32, source: &str) -> Result<u32, String> {
        unsafe {
            let shader = (self.create_shader)(kind);
            let (text, length) = (source.as_ptr() as *const c_char, source.len() as i32);
            (self.shader_source)(shader, 1, &text, &length);
            (self.compile_shader)(shader);
            let mut compiled = 0;
            (self.get_shader)(shader, COMPILE_STATUS, &mut compiled);
            if compiled == 0 {
                let mut log = [0u8; 2048];
                let mut written = 0;
                (self.shader_log)(shader, log.len() as i32, &mut written, log.as_mut_ptr() as *mut c_char);
                return Err(String::from_utf8_lossy(&log[..written.max(0) as usize]).into_owned());
            }
            Ok(shader)
        }
    }

    /// Compiles and links a program. `attributes` are bound from location 0 on.
    pub fn program(&self, vertex: &str, fragment: &str, attributes: &[&CStr]) -> Result<u32, String> {
        unsafe {
            let program = (self.create_program)();
            (self.attach_shader)(program, self.shader(VERTEX_SHADER, vertex)?);
            (self.attach_shader)(program, self.shader(FRAGMENT_SHADER, fragment)?);
            for (location, name) in attributes.iter().enumerate() {
                (self.bind_attribute)(program, location as u32, name.as_ptr());
            }
            (self.link_program)(program);
            let mut linked = 0;
            (self.get_program)(program, LINK_STATUS, &mut linked);
            if linked == 0 {
                let mut log = [0u8; 2048];
                let mut written = 0;
                (self.program_log)(program, log.len() as i32, &mut written, log.as_mut_ptr() as *mut c_char);
                return Err(String::from_utf8_lossy(&log[..written.max(0) as usize]).into_owned());
            }
            Ok(program)
        }
    }

    pub fn uniform(&self, program: u32, name: &CStr) -> i32 {
        unsafe { (self.uniform_location)(program, name.as_ptr()) }
    }

    /// A texture with no mip levels but its first, ready to be filled.
    pub fn texture(&self, filter: i32) -> u32 {
        let mut texture = 0;
        unsafe {
            (self.gen_textures)(1, &mut texture);
            (self.bind_texture)(TEXTURE_2D, texture);
            (self.tex_parameter)(TEXTURE_2D, TEXTURE_MIN_FILTER, filter);
            (self.tex_parameter)(TEXTURE_2D, TEXTURE_MAG_FILTER, filter);
            (self.tex_parameter)(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE);
            (self.tex_parameter)(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE);
            (self.tex_parameter)(TEXTURE_2D, TEXTURE_MAX_LEVEL, 0);
        }
        texture
    }
}

/// One triangle that covers the whole target, from the vertex's number alone.
/// `u_turn` puts the picture's first row at the top, for the window.
pub const COVER_VERTEX: &str = "#version 330 core
uniform int u_turn;
out vec2 v_at;
void main() {
    vec2 corner = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
    v_at = corner;
    if (u_turn != 0) {
        corner.y = 1.0 - corner.y;
    }
    gl_Position = vec4(corner * 2.0 - 1.0, 0.0, 1.0);
}
";

/// Copies a texture. With `u_key`, texels of the marker colour are left out,
/// which is how 2D written to the frame buffer goes over 3D.
pub const COVER_FRAGMENT: &str = "#version 330 core
uniform sampler2D u_picture;
uniform int u_key;
in vec2 v_at;
out vec4 o_color;
void main() {
    vec4 texel = texture(u_picture, v_at);
    if (u_key != 0 && texel.r > 0.99 && texel.g < 0.01 && texel.b > 0.99) {
        discard;
    }
    o_color = texel;
}
";

/// A program that draws one texture over the whole target.
pub struct Cover {
    pub program: u32,
    pub turn: i32,
    pub key: i32,
    pub array: u32,
}

impl Cover {
    pub fn new(gl: &Gl) -> Result<Cover, String> {
        let program = gl.program(COVER_VERTEX, COVER_FRAGMENT, &[])?;
        let mut array = 0;
        unsafe { (gl.gen_vertex_arrays)(1, &mut array) };
        Ok(Cover { program, turn: gl.uniform(program, c"u_turn"), key: gl.uniform(program, c"u_key"), array })
    }

    /// Draws the texture over the viewport. The caller has set the target,
    /// the viewport and blending.
    pub fn draw(&self, gl: &Gl, texture: u32, turn: bool, key: bool) {
        unsafe {
            (gl.use_program)(self.program);
            (gl.uniform_1i)(self.turn, turn as i32);
            (gl.uniform_1i)(self.key, key as i32);
            (gl.bind_vertex_array)(self.array);
            (gl.active_texture)(TEXTURE0);
            (gl.bind_texture)(TEXTURE_2D, texture);
            (gl.disable)(DEPTH_TEST);
            (gl.disable)(SCISSOR_TEST);
            (gl.color_mask)(1, 1, 1, 1);
            (gl.depth_mask)(0);
            (gl.draw_arrays)(TRIANGLES, 0, 3);
        }
    }
}

/// The largest area of this shape that fits in the window, centred: x, y,
/// width and height.
pub fn letterbox(window: (i32, i32), shape: (i32, i32)) -> (i32, i32, i32, i32) {
    let (wide, high) = (window.0.max(1) as i64, window.1.max(1) as i64);
    let (width, height) = if wide * shape.1 as i64 >= high * shape.0 as i64 {
        ((high * shape.0 as i64 / shape.1 as i64) as i32, high as i32)
    } else {
        (wide as i32, (wide * shape.1 as i64 / shape.0 as i64) as i32)
    };
    ((window.0 - width) / 2, (window.1 - height) / 2, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letterbox_keeps_the_shape_and_centres_it() {
        assert_eq!(letterbox((1280, 960), (640, 480)), (0, 0, 1280, 960));
        assert_eq!(letterbox((1920, 1080), (640, 480)), (240, 0, 1440, 1080));
        assert_eq!(letterbox((600, 1000), (640, 480)), (0, 275, 600, 450));
        assert_eq!(letterbox((0, 0), (640, 480)).2, 1);
    }
}
