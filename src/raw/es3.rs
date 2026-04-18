use super::with_gl;
use crate::helpers::{copy_info_log, js_value_to_glint, pixel_byte_size_3d};
use crate::types::{
    Buffer, GL_FALSE, GL_TRUE, GLbitfield, GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr,
    GLsizei, GLsizeiptr, GLuint, Program, Sampler, SyncHandle, TransformFeedback,
};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl;

fn write_glint_array_from_js(params: *mut GLint, value: &wasm_bindgen::JsValue) {
    if params.is_null() {
        return;
    }

    unsafe { *params = 0 };

    if let Some(number) = value.as_f64() {
        unsafe { *params = number as GLint };
        return;
    }

    if let Some(flag) = value.as_bool() {
        unsafe { *params = flag as GLint };
        return;
    }

    if let Some(array) = value.dyn_ref::<js_sys::Int32Array>() {
        for i in 0..array.length() as usize {
            unsafe { *params.add(i) = array.get_index(i as u32) };
        }
        return;
    }

    if let Some(array) = value.dyn_ref::<js_sys::Uint32Array>() {
        for i in 0..array.length() as usize {
            unsafe { *params.add(i) = array.get_index(i as u32) as GLint };
        }
        return;
    }

    if js_sys::Array::is_array(value) {
        let array = js_sys::Array::from(value);
        for i in 0..array.length() as usize {
            unsafe { *params.add(i) = js_value_to_glint(&array.get(i as u32)) };
        }
    }
}

pub(crate) unsafe extern "C" fn gl_draw_arrays_instanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instance_count: GLsizei,
) {
    with_gl(|s| {
        s.gl.draw_arrays_instanced(mode, first, count, instance_count)
    });
}

pub(crate) unsafe extern "C" fn gl_draw_elements_instanced(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instance_count: GLsizei,
) {
    with_gl(|s| {
        s.gl.draw_elements_instanced_with_i32(mode, count, type_, indices as i32, instance_count)
    });
}

pub(crate) unsafe extern "C" fn gl_vertex_attrib_divisor(index: GLuint, divisor: GLuint) {
    with_gl(|s| s.gl.vertex_attrib_divisor(index, divisor));
}

pub(crate) unsafe extern "C" fn gl_draw_range_elements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
) {
    with_gl(|s| {
        s.gl.draw_range_elements_with_i32(mode, start, end, count, type_, indices as i32)
    });
}

pub(crate) unsafe extern "C" fn gl_vertex_attrib_i_pointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
) {
    with_gl(|s| {
        s.gl.vertex_attrib_i_pointer_with_i32(index, size, type_, stride, pointer as i32)
    });
}

pub(crate) unsafe extern "C" fn gl_draw_buffers(n: GLsizei, bufs: *const GLenum) {
    if n <= 0 || bufs.is_null() {
        return;
    }
    let array = js_sys::Array::new();
    for i in 0..n as usize {
        array.push(&wasm_bindgen::JsValue::from_f64(
            unsafe { *bufs.add(i) } as f64
        ));
    }
    with_gl(|s| s.gl.draw_buffers(&array));
}

pub(crate) unsafe extern "C" fn gl_read_buffer(src: GLenum) {
    with_gl(|s| s.gl.read_buffer(src));
}

pub(crate) unsafe extern "C" fn gl_bind_buffer_base(target: GLenum, index: GLuint, buffer: GLuint) {
    with_gl(|s| {
        let buffer = (buffer != 0)
            .then(|| s.buffers.get(&Buffer::from(buffer)))
            .flatten();
        s.gl.bind_buffer_base(target, index, buffer);
    });
}

pub(crate) unsafe extern "C" fn gl_bind_buffer_range(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    with_gl(|s| {
        let buffer = (buffer != 0)
            .then(|| s.buffers.get(&Buffer::from(buffer)))
            .flatten();
        s.gl.bind_buffer_range_with_i32_and_i32(target, index, buffer, offset, size);
    });
}

pub(crate) unsafe extern "C" fn gl_copy_buffer_sub_data(
    read_target: GLenum,
    write_target: GLenum,
    read_offset: GLintptr,
    write_offset: GLintptr,
    size: GLsizeiptr,
) {
    with_gl(|s| {
        s.gl.copy_buffer_sub_data_with_i32_and_i32_and_i32(
            read_target,
            write_target,
            read_offset,
            write_offset,
            size,
        )
    });
}

pub(crate) unsafe extern "C" fn gl_invalidate_sub_framebuffer(
    target: GLenum,
    num_attachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    if num_attachments <= 0 || attachments.is_null() {
        return;
    }
    let array = js_sys::Array::new();
    for i in 0..num_attachments as usize {
        array.push(&wasm_bindgen::JsValue::from_f64(
            unsafe { *attachments.add(i) } as f64,
        ));
    }
    with_gl(|s| {
        let _ =
            s.gl.invalidate_sub_framebuffer(target, &array, x, y, width, height);
    });
}

pub(crate) unsafe extern "C" fn gl_get_buffer_sub_data(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut c_void,
) {
    if size <= 0 || data.is_null() {
        return;
    }
    with_gl(|s| {
        let bytes = unsafe { std::slice::from_raw_parts_mut(data.cast::<u8>(), size as usize) };
        s.gl.get_buffer_sub_data_with_i32_and_u8_array(target, offset, bytes);
    });
}

pub(crate) unsafe extern "C" fn gl_map_buffer_range(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    _access: GLbitfield,
) -> *mut c_void {
    if length <= 0 {
        return std::ptr::null_mut();
    }
    with_gl(|s| {
        let data = vec![0u8; length as usize];
        let ptr = data.as_ptr() as *mut c_void;
        s.mapped_buffer = Some(crate::state::MappedBuffer {
            target,
            offset,
            data,
        });
        ptr
    })
}

pub(crate) unsafe extern "C" fn gl_unmap_buffer(target: GLenum) -> GLboolean {
    with_gl(|s| {
        if let Some(mapped) = s.mapped_buffer.take() {
            if mapped.target != target {
                return GL_FALSE;
            }
            let view = unsafe { js_sys::Uint8Array::view(&mapped.data) };
            s.gl.buffer_sub_data_with_i32_and_array_buffer_view(target, mapped.offset, &view);
            GL_TRUE
        } else {
            GL_FALSE
        }
    })
}

pub(crate) unsafe extern "C" fn gl_flush_mapped_buffer_range(
    _target: GLenum,
    _offset: GLintptr,
    _length: GLsizeiptr,
) {
}

pub(crate) unsafe extern "C" fn gl_fence_sync(
    condition: GLenum,
    _flags: GLbitfield,
) -> *mut c_void {
    with_gl(|s| match s.gl.fence_sync(condition, 0) {
        Some(sync) => {
            let handle = s.alloc_handle::<SyncHandle>();
            s.syncs.insert(handle, sync);
            handle.get() as usize as *mut c_void
        }
        None => std::ptr::null_mut(),
    })
}

pub(crate) unsafe extern "C" fn gl_client_wait_sync(
    sync: *mut c_void,
    flags: GLbitfield,
    timeout: u64,
) -> GLenum {
    const GL_WAIT_FAILED: GLenum = 0x911C;
    let handle = SyncHandle::from(sync as usize as GLuint);
    with_gl(|s| {
        if let Some(sync) = s.syncs.get(&handle) {
            s.gl.client_wait_sync_with_u32(sync, flags, timeout.min(u32::MAX as u64) as u32)
        } else {
            GL_WAIT_FAILED
        }
    })
}

pub(crate) unsafe extern "C" fn gl_delete_sync(sync: *mut c_void) {
    if sync.is_null() {
        return;
    }
    let handle = SyncHandle::from(sync as usize as GLuint);
    with_gl(|s| {
        if let Some(sync) = s.syncs.remove(&handle) {
            s.gl.delete_sync(Some(&sync));
        }
    });
}

pub(crate) unsafe extern "C" fn gl_is_sync(sync: *mut c_void) -> GLboolean {
    if sync.is_null() {
        return GL_FALSE;
    }
    let handle = SyncHandle::from(sync as usize as GLuint);
    with_gl(|s| match s.syncs.get(&handle) {
        Some(sync) => {
            if s.gl.is_sync(Some(sync)) {
                GL_TRUE
            } else {
                GL_FALSE
            }
        }
        None => GL_FALSE,
    })
}

pub(crate) unsafe extern "C" fn gl_wait_sync(sync: *mut c_void, flags: GLbitfield, _timeout: u64) {
    if sync.is_null() {
        return;
    }
    let handle = SyncHandle::from(sync as usize as GLuint);
    with_gl(|s| {
        if let Some(sync) = s.syncs.get(&handle) {
            s.gl.wait_sync_with_f64(sync, flags, -1.0);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_gen_samplers(count: GLsizei, samplers: *mut GLuint) {
    if count <= 0 || samplers.is_null() {
        return;
    }
    with_gl(|s| {
        for i in 0..count as usize {
            let handle = match s.gl.create_sampler() {
                Some(sampler) => {
                    let handle = s.alloc_handle::<Sampler>();
                    s.samplers.insert(handle, sampler);
                    handle.get()
                }
                None => 0,
            };
            unsafe { *samplers.add(i) = handle };
        }
    });
}

pub(crate) unsafe extern "C" fn gl_delete_samplers(count: GLsizei, samplers: *const GLuint) {
    if count <= 0 || samplers.is_null() {
        return;
    }
    with_gl(|s| {
        for i in 0..count as usize {
            let id = unsafe { *samplers.add(i) };
            if let Some(sampler) = s.samplers.remove(&Sampler::from(id)) {
                s.gl.delete_sampler(Some(&sampler));
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_bind_sampler(unit: GLuint, sampler: GLuint) {
    with_gl(|s| {
        let sampler = s.samplers.get(&Sampler::from(sampler));
        s.gl.bind_sampler(unit, sampler);
    });
}

pub(crate) unsafe extern "C" fn gl_sampler_parameteri(
    sampler: GLuint,
    pname: GLenum,
    param: GLint,
) {
    with_gl(|s| {
        if let Some(sampler) = s.samplers.get(&Sampler::from(sampler)) {
            s.gl.sampler_parameteri(sampler, pname, param);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_sampler_parameterf(
    sampler: GLuint,
    pname: GLenum,
    param: GLfloat,
) {
    with_gl(|s| {
        if let Some(sampler) = s.samplers.get(&Sampler::from(sampler)) {
            s.gl.sampler_parameterf(sampler, pname, param);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_sampler_parameter_iv(
    sampler: GLuint,
    pname: GLenum,
    params: *const GLint,
) {
    if params.is_null() {
        return;
    }
    with_gl(|s| {
        if let Some(sampler) = s.samplers.get(&Sampler::from(sampler)) {
            s.gl.sampler_parameteri(sampler, pname, unsafe { *params });
        }
    });
}

pub(crate) unsafe extern "C" fn gl_gen_transform_feedbacks(count: GLsizei, ids: *mut GLuint) {
    if count <= 0 || ids.is_null() {
        return;
    }

    with_gl(|s| {
        for i in 0..count as usize {
            let handle = match s.gl.create_transform_feedback() {
                Some(transform_feedback) => {
                    let handle = s.alloc_handle::<TransformFeedback>();
                    s.transform_feedbacks.insert(handle, transform_feedback);
                    handle.get()
                }
                None => 0,
            };
            unsafe { *ids.add(i) = handle };
        }
    });
}

pub(crate) unsafe extern "C" fn gl_delete_transform_feedbacks(count: GLsizei, ids: *const GLuint) {
    if count <= 0 || ids.is_null() {
        return;
    }

    with_gl(|s| {
        for i in 0..count as usize {
            let id = unsafe { *ids.add(i) };
            if let Some(transform_feedback) =
                s.transform_feedbacks.remove(&TransformFeedback::from(id))
            {
                s.gl.delete_transform_feedback(Some(&transform_feedback));
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_bind_transform_feedback(target: GLenum, id: GLuint) {
    with_gl(|s| {
        let transform_feedback = (id != 0)
            .then(|| s.transform_feedbacks.get(&TransformFeedback::from(id)))
            .flatten();
        s.gl.bind_transform_feedback(target, transform_feedback);
    });
}

pub(crate) unsafe extern "C" fn gl_is_transform_feedback(id: GLuint) -> GLboolean {
    with_gl(|s| {
        s.transform_feedbacks
            .get(&TransformFeedback::from(id))
            .map(|transform_feedback| {
                if s.gl.is_transform_feedback(Some(transform_feedback)) {
                    GL_TRUE
                } else {
                    GL_FALSE
                }
            })
            .unwrap_or(GL_FALSE)
    })
}

pub(crate) unsafe extern "C" fn gl_begin_transform_feedback(primitive_mode: GLenum) {
    with_gl(|s| s.gl.begin_transform_feedback(primitive_mode));
}

pub(crate) unsafe extern "C" fn gl_end_transform_feedback() {
    with_gl(|s| s.gl.end_transform_feedback());
}

pub(crate) unsafe extern "C" fn gl_pause_transform_feedback() {
    with_gl(|s| s.gl.pause_transform_feedback());
}

pub(crate) unsafe extern "C" fn gl_resume_transform_feedback() {
    with_gl(|s| s.gl.resume_transform_feedback());
}

pub(crate) unsafe extern "C" fn gl_transform_feedback_varyings(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const c_char,
    buffer_mode: GLenum,
) {
    if count < 0 || varyings.is_null() {
        return;
    }

    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            let array = js_sys::Array::new();
            for i in 0..count as usize {
                let varying = unsafe { *varyings.add(i) };
                let varying = unsafe { CStr::from_ptr(varying) }.to_string_lossy();
                array.push(&wasm_bindgen::JsValue::from_str(&varying));
            }
            s.gl.transform_feedback_varyings(program, &array.into(), buffer_mode);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_uniform_block_index(
    program: GLuint,
    uniform_block_name: *const c_char,
) -> GLuint {
    const GL_INVALID_INDEX: GLuint = u32::MAX;

    if uniform_block_name.is_null() {
        return GL_INVALID_INDEX;
    }

    let uniform_block_name = unsafe { CStr::from_ptr(uniform_block_name) }
        .to_str()
        .unwrap_or("");
    with_gl(|s| {
        s.programs
            .get(&Program::from(program))
            .map(|program| s.gl.get_uniform_block_index(program, uniform_block_name))
            .unwrap_or(GL_INVALID_INDEX)
    })
}

pub(crate) unsafe extern "C" fn gl_uniform_block_binding(
    program: GLuint,
    uniform_block_index: GLuint,
    uniform_block_binding: GLuint,
) {
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            s.gl.uniform_block_binding(program, uniform_block_index, uniform_block_binding);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_active_uniform_block_iv(
    program: GLuint,
    uniform_block_index: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    if params.is_null() {
        return;
    }

    with_gl(|s| {
        unsafe { *params = 0 };
        if let Some(program) = s.programs.get(&Program::from(program)) {
            match s
                .gl
                .get_active_uniform_block_parameter(program, uniform_block_index, pname)
            {
                Ok(value) => write_glint_array_from_js(params, &value),
                Err(_) => {
                    let _ = s.gl.get_error();
                }
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_active_uniform_block_name(
    program: GLuint,
    uniform_block_index: GLuint,
    buf_size: GLsizei,
    length: *mut GLsizei,
    uniform_block_name: *mut c_char,
) {
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            let name =
                s.gl.get_active_uniform_block_name(program, uniform_block_index)
                    .unwrap_or_default();
            copy_info_log(
                name.as_bytes(),
                buf_size,
                length,
                uniform_block_name.cast::<GLchar>(),
            );
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_queryiv(_target: GLenum, pname: GLenum, params: *mut GLint) {
    if !params.is_null() {
        unsafe {
            *params = match pname {
                0x8865 => 0,
                0x8864 => 0,
                _ => 0,
            };
        }
    }
}

pub(crate) unsafe extern "C" fn gl_get_internalformativ(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    buf_size: GLsizei,
    params: *mut GLint,
) {
    if buf_size <= 0 || params.is_null() {
        return;
    }
    for i in 0..buf_size as usize {
        unsafe { *params.add(i) = 0 };
    }

    if target != Gl::RENDERBUFFER {
        return;
    }

    let fallback_num_samples = match internalformat {
        0x8051 | 0x8056 | 0x8058 | 0x8059 | 0x8D62 | 0x8C43 => 1,
        _ => 0,
    };

    const GL_SAMPLES: GLenum = 0x80A9;
    const GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380;
    if pname == GL_NUM_SAMPLE_COUNTS {
        with_gl(|s| {
            match s
                .gl
                .get_internalformat_parameter(target, internalformat, GL_SAMPLES)
            {
                Ok(value) => {
                    if let Some(array) = value.dyn_ref::<js_sys::Int32Array>() {
                        let len = array.length() as GLint;
                        unsafe { *params = if len > 0 { len } else { fallback_num_samples } };
                    } else if let Some(number) = value.as_f64() {
                        unsafe {
                            *params = if number > 0.0 {
                                1
                            } else {
                                fallback_num_samples
                            };
                        }
                    } else {
                        unsafe { *params = fallback_num_samples };
                    }
                }
                Err(_) => {
                    let _ = s.gl.get_error();
                    unsafe { *params = fallback_num_samples };
                }
            }
        });
        return;
    }

    if pname != GL_SAMPLES {
        return;
    }

    with_gl(|s| {
        match s
            .gl
            .get_internalformat_parameter(target, internalformat, pname)
        {
            Ok(value) => {
                if let Some(array) = value.dyn_ref::<js_sys::Int32Array>() {
                    let len = (array.length() as GLsizei).min(buf_size) as usize;
                    if len > 0 {
                        for i in 0..len {
                            unsafe { *params.add(i) = array.get_index(i as u32) };
                        }
                    } else {
                        unsafe { *params = fallback_num_samples };
                    }
                } else if let Some(number) = value.as_f64() {
                    unsafe {
                        *params = if number > 0.0 {
                            number as GLint
                        } else {
                            fallback_num_samples
                        };
                    }
                } else {
                    unsafe { *params = fallback_num_samples };
                }
            }
            Err(_) => {
                let _ = s.gl.get_error();
                unsafe { *params = fallback_num_samples };
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_program_binary(
    _program: GLuint,
    _buf_size: GLsizei,
    length: *mut GLsizei,
    _binary_format: *mut GLenum,
    _binary: *mut c_void,
) {
    if !length.is_null() {
        unsafe { *length = 0 };
    }
}

pub(crate) unsafe extern "C" fn gl_program_binary(
    _program: GLuint,
    _binary_format: GLenum,
    _binary: *const c_void,
    _length: GLsizei,
) {
}

pub(crate) unsafe extern "C" fn gl_program_parameteri(
    _program: GLuint,
    _pname: GLenum,
    _value: GLint,
) {
}

pub(crate) unsafe extern "C" fn gl_tex_image_3d(
    target: GLenum,
    level: GLint,
    internal_format: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    with_gl(|s| {
        let data = if pixels.is_null() {
            None
        } else {
            let len = pixel_byte_size_3d(width, height, depth, format, type_);
            Some(unsafe { std::slice::from_raw_parts(pixels.cast::<u8>(), len) })
        };
        let _ = s.gl.tex_image_3d_with_opt_u8_array(
            target,
            level,
            internal_format,
            width,
            height,
            depth,
            border,
            format,
            type_,
            data,
        );
    });
}

pub(crate) unsafe extern "C" fn gl_tex_sub_image_3d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    with_gl(|s| {
        let data = if pixels.is_null() {
            None
        } else {
            let len = pixel_byte_size_3d(width, height, depth, format, type_);
            Some(unsafe { std::slice::from_raw_parts(pixels.cast::<u8>(), len) })
        };
        let _ = s.gl.tex_sub_image_3d_with_opt_u8_array(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data,
        );
    });
}

pub(crate) unsafe extern "C" fn gl_tex_storage_3d(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    with_gl(|s| {
        s.gl.tex_storage_3d(target, levels, internalformat, width, height, depth)
    });
}
