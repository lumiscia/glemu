use super::{EMPTY_C_STRING, with_gl};
use crate::helpers::{
    copy_info_log, get_parameter_int, is_valid_webgl2_get_parameter, pixel_byte_size,
    write_glfloat_params_from_js, write_glint_params_from_js,
};
use crate::types::{
    Buffer, Framebuffer, GL_EXTENSIONS, GL_FALSE, GL_NUM_EXTENSIONS, GL_TRUE, GLbitfield,
    GLboolean, GLchar, GLclampf, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint,
    Program, Renderbuffer, Shader, Texture, UniformLocation, VertexArray,
};
use js_sys::Uint8Array;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use web_sys::WebGl2RenderingContext as Gl;

macro_rules! gen_objects {
    ($fn_name:ident, $create:ident, $map:ident, $handle_ty:ty) => {
        pub(crate) unsafe extern "C" fn $fn_name(n: GLsizei, out: *mut GLuint) {
            with_gl(|s| {
                for i in 0..n as usize {
                    let id = match s.gl.$create() {
                        Some(obj) => {
                            let handle = s.alloc_handle::<$handle_ty>();
                            s.$map.insert(handle, obj);
                            handle.get()
                        }
                        None => 0,
                    };
                    unsafe { *out.add(i) = id };
                }
            });
        }
    };
}

macro_rules! uniform_scalar {
    ($fn_name:ident, $method:ident, $T:ty) => {
        pub(crate) unsafe extern "C" fn $fn_name(location: GLint, v0: $T) {
            with_gl(|s| {
                if let Some(loc) = s
                    .uniform_locations
                    .get(&UniformLocation::from(location as GLuint))
                {
                    s.gl.$method(Some(loc), v0);
                }
            });
        }
    };
}

macro_rules! uniform_vec {
    ($fn_name:ident, $method:ident, $T:ty, $n:expr) => {
        pub(crate) unsafe extern "C" fn $fn_name(
            location: GLint,
            count: GLsizei,
            value: *const $T,
        ) {
            with_gl(|s| {
                if let Some(loc) = s
                    .uniform_locations
                    .get(&UniformLocation::from(location as GLuint))
                {
                    let data = unsafe { std::slice::from_raw_parts(value, count as usize * $n) };
                    s.gl.$method(Some(loc), data);
                }
            });
        }
    };
}

macro_rules! uniform_matrix {
    ($fn_name:ident, $method:ident, $n:expr) => {
        pub(crate) unsafe extern "C" fn $fn_name(
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ) {
            with_gl(|s| {
                if let Some(loc) = s
                    .uniform_locations
                    .get(&UniformLocation::from(location as GLuint))
                {
                    let data =
                        unsafe { std::slice::from_raw_parts(value, count as usize * $n * $n) };
                    s.gl.$method(Some(loc), transpose != 0, data);
                }
            });
        }
    };
}

mod buffer;
mod framebuffer;
mod program;
mod query;
mod state;
mod texture;
mod uniform;
mod vertex_array;

pub(crate) use buffer::*;
pub(crate) use framebuffer::*;
pub(crate) use program::*;
pub(crate) use query::*;
pub(crate) use state::*;
pub(crate) use texture::*;
pub(crate) use uniform::*;
pub(crate) use vertex_array::*;
