use super::*;

pub(crate) unsafe extern "C" fn gl_clear_depthf(depth: GLclampf) {
    with_gl(|s| s.gl.clear_depth(depth));
}

pub(crate) unsafe extern "C" fn gl_blend_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    with_gl(|s| s.gl.blend_color(r, g, b, a));
}

pub(crate) unsafe extern "C" fn gl_blend_equation(mode: GLenum) {
    with_gl(|s| s.gl.blend_equation(mode));
}

pub(crate) unsafe extern "C" fn gl_blend_equation_separate(mode_rgb: GLenum, mode_alpha: GLenum) {
    with_gl(|s| s.gl.blend_equation_separate(mode_rgb, mode_alpha));
}

pub(crate) unsafe extern "C" fn gl_blend_func(sfactor: GLenum, dfactor: GLenum) {
    with_gl(|s| s.gl.blend_func(sfactor, dfactor));
}

pub(crate) unsafe extern "C" fn gl_blend_func_separate(
    src_rgb: GLenum,
    dst_rgb: GLenum,
    src_alpha: GLenum,
    dst_alpha: GLenum,
) {
    with_gl(|s| {
        s.gl.blend_func_separate(src_rgb, dst_rgb, src_alpha, dst_alpha)
    });
}

pub(crate) unsafe extern "C" fn gl_clear(mask: GLbitfield) {
    with_gl(|s| s.gl.clear(mask));
}

pub(crate) unsafe extern "C" fn gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    with_gl(|s| s.gl.clear_color(r, g, b, a));
}

pub(crate) unsafe extern "C" fn gl_clear_stencil(value: GLint) {
    with_gl(|s| s.gl.clear_stencil(value));
}

pub(crate) unsafe extern "C" fn gl_color_mask(
    r: GLboolean,
    g: GLboolean,
    b: GLboolean,
    a: GLboolean,
) {
    with_gl(|s| s.gl.color_mask(r != 0, g != 0, b != 0, a != 0));
}

pub(crate) unsafe extern "C" fn gl_cull_face(mode: GLenum) {
    with_gl(|s| s.gl.cull_face(mode));
}

pub(crate) unsafe extern "C" fn gl_depth_mask(flag: GLboolean) {
    with_gl(|s| s.gl.depth_mask(flag != 0));
}

pub(crate) unsafe extern "C" fn gl_disable(cap: GLenum) {
    with_gl(|s| s.gl.disable(cap));
}

pub(crate) unsafe extern "C" fn gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    with_gl(|s| s.gl.draw_arrays(mode, first, count));
}

pub(crate) unsafe extern "C" fn gl_draw_elements(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
) {
    with_gl(|s| {
        s.gl.draw_elements_with_i32(mode, count, type_, indices as i32)
    });
}

pub(crate) unsafe extern "C" fn gl_enable(cap: GLenum) {
    with_gl(|s| s.gl.enable(cap));
}

pub(crate) unsafe extern "C" fn gl_finish() {
    with_gl(|s| s.gl.finish());
}

pub(crate) unsafe extern "C" fn gl_flush() {
    with_gl(|s| s.gl.flush());
}

pub(crate) unsafe extern "C" fn gl_front_face(mode: GLenum) {
    with_gl(|s| s.gl.front_face(mode));
}

pub(crate) unsafe extern "C" fn gl_get_booleanv(pname: GLenum, params: *mut GLboolean) {
    with_gl(|s| {
        unsafe { *params = GL_FALSE };
        if !is_valid_webgl2_get_parameter(pname) {
            return;
        }
        match s.gl.get_parameter(pname) {
            Ok(value) => unsafe {
                *params = if value.as_bool().unwrap_or(false) {
                    GL_TRUE
                } else {
                    GL_FALSE
                };
            },
            Err(_) => {
                let _ = s.gl.get_error();
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_error() -> GLenum {
    with_gl(|s| s.gl.get_error())
}

pub(crate) unsafe extern "C" fn gl_get_integerv(pname: GLenum, params: *mut GLint) {
    with_gl(|s| {
        unsafe { *params = 0 };
        match pname {
            GL_NUM_EXTENSIONS => {
                unsafe { *params = s.cached_strings.extensions.len() as GLint };
                return;
            }
            0x84E2 => {
                if let Some(value) = get_parameter_int(&s.gl, Gl::MAX_TEXTURE_IMAGE_UNITS) {
                    unsafe { *params = value };
                }
                return;
            }
            0x8B4A => {
                if let Some(value) = get_parameter_int(&s.gl, Gl::MAX_VERTEX_UNIFORM_VECTORS) {
                    unsafe { *params = value * 4 };
                }
                return;
            }
            0x8B49 => {
                if let Some(value) = get_parameter_int(&s.gl, Gl::MAX_FRAGMENT_UNIFORM_VECTORS) {
                    unsafe { *params = value * 4 };
                }
                return;
            }
            0x8B4B => {
                if let Some(value) = get_parameter_int(&s.gl, Gl::MAX_VARYING_VECTORS) {
                    unsafe { *params = value * 4 };
                }
                return;
            }
            0x8871 => {
                if let Some(value) = get_parameter_int(&s.gl, Gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS)
                {
                    unsafe { *params = value };
                }
                return;
            }
            0x821B => {
                unsafe { *params = 3 };
                return;
            }
            0x821C => {
                unsafe { *params = 0 };
                return;
            }
            _ => {}
        }

        if !is_valid_webgl2_get_parameter(pname) {
            return;
        }

        match s.gl.get_parameter(pname) {
            Ok(value) => write_glint_params_from_js(pname, params, &value),
            Err(_) => {
                let _ = s.gl.get_error();
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_string(name: GLenum) -> *const u8 {
    with_gl(|s| match name {
        0x1F00 => s.cached_strings.vendor.as_ptr(),
        0x1F01 => s.cached_strings.renderer.as_ptr(),
        0x1F02 => s.cached_strings.version.as_ptr(),
        0x8B8C => s.cached_strings.shading_language_version.as_ptr(),
        GL_EXTENSIONS => EMPTY_C_STRING.as_ptr(),
        _ => std::ptr::null(),
    })
}

pub(crate) unsafe extern "C" fn gl_get_stringi(name: GLenum, index: GLuint) -> *const u8 {
    with_gl(|s| {
        if name == GL_EXTENSIONS {
            s.cached_strings
                .extensions
                .get(index as usize)
                .map_or(std::ptr::null(), Vec::as_ptr)
        } else {
            std::ptr::null()
        }
    })
}

pub(crate) unsafe extern "C" fn gl_line_width(width: GLfloat) {
    with_gl(|s| s.gl.line_width(width));
}

pub(crate) unsafe extern "C" fn gl_read_pixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
) {
    with_gl(|s| {
        let len = pixel_byte_size(width, height, format, type_);
        let slice = unsafe { std::slice::from_raw_parts_mut(pixels.cast::<u8>(), len) };
        let _ =
            s.gl.read_pixels_with_opt_u8_array(x, y, width, height, format, type_, Some(slice));
    });
}

pub(crate) unsafe extern "C" fn gl_scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    with_gl(|s| s.gl.scissor(x, y, width, height));
}

pub(crate) unsafe extern "C" fn gl_stencil_func(func: GLenum, ref_: GLint, mask: GLuint) {
    with_gl(|s| s.gl.stencil_func(func, ref_, mask));
}

pub(crate) unsafe extern "C" fn gl_stencil_func_separate(
    face: GLenum,
    func: GLenum,
    ref_: GLint,
    mask: GLuint,
) {
    with_gl(|s| s.gl.stencil_func_separate(face, func, ref_, mask));
}

pub(crate) unsafe extern "C" fn gl_stencil_mask(mask: GLuint) {
    with_gl(|s| s.gl.stencil_mask(mask));
}

pub(crate) unsafe extern "C" fn gl_stencil_mask_separate(face: GLenum, mask: GLuint) {
    with_gl(|s| s.gl.stencil_mask_separate(face, mask));
}

pub(crate) unsafe extern "C" fn gl_stencil_op(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    with_gl(|s| s.gl.stencil_op(fail, zfail, zpass));
}

pub(crate) unsafe extern "C" fn gl_stencil_op_separate(
    face: GLenum,
    fail: GLenum,
    zfail: GLenum,
    zpass: GLenum,
) {
    with_gl(|s| s.gl.stencil_op_separate(face, fail, zfail, zpass));
}

pub(crate) unsafe extern "C" fn gl_viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    with_gl(|s| s.gl.viewport(x, y, width, height));
}

pub(crate) unsafe extern "C" fn gl_is_enabled(cap: GLenum) -> GLboolean {
    with_gl(|s| {
        if s.gl.is_enabled(cap) {
            GL_TRUE
        } else {
            GL_FALSE
        }
    })
}

pub(crate) unsafe extern "C" fn gl_depth_func(func: GLenum) {
    with_gl(|s| s.gl.depth_func(func));
}

pub(crate) unsafe extern "C" fn gl_depth_rangef(n: GLclampf, f: GLclampf) {
    with_gl(|s| s.gl.depth_range(n, f));
}

pub(crate) unsafe extern "C" fn gl_get_floatv(pname: GLenum, params: *mut GLfloat) {
    with_gl(|s| {
        unsafe { *params = 0.0 };
        if !is_valid_webgl2_get_parameter(pname) {
            return;
        }
        match s.gl.get_parameter(pname) {
            Ok(value) => write_glfloat_params_from_js(pname, params, &value),
            Err(_) => {
                let _ = s.gl.get_error();
            }
        }
    });
}
