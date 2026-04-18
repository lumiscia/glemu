use super::*;

pub(crate) unsafe extern "C" fn gl_bind_buffer(target: GLenum, buffer: GLuint) {
    with_gl(|s| {
        let buffer = (buffer != 0)
            .then(|| s.buffers.get(&Buffer::from(buffer)))
            .flatten();
        s.gl.bind_buffer(target, buffer);
    });
}

pub(crate) unsafe extern "C" fn gl_buffer_data(
    target: GLenum,
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum,
) {
    with_gl(|s| {
        if data.is_null() {
            s.gl.buffer_data_with_i32(target, size, usage);
        } else {
            let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), size as usize) };
            let array = Uint8Array::from(bytes);
            s.gl.buffer_data_with_array_buffer_view(target, &array, usage);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_buffer_sub_data(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
) {
    with_gl(|s| {
        let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), size as usize) };
        let array = Uint8Array::from(bytes);
        s.gl.buffer_sub_data_with_i32_and_array_buffer_view(target, offset, &array);
    });
}

pub(crate) unsafe extern "C" fn gl_delete_buffers(n: GLsizei, buffers: *const GLuint) {
    with_gl(|s| {
        for i in 0..n as usize {
            let id = unsafe { *buffers.add(i) };
            if let Some(buffer) = s.buffers.remove(&Buffer::from(id)) {
                s.gl.delete_buffer(Some(&buffer));
            }
        }
    });
}

gen_objects!(gl_gen_buffers, create_buffer, buffers, Buffer);

pub(crate) unsafe extern "C" fn gl_get_buffer_parameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_gl(|s| {
        let value = s.gl.get_buffer_parameter(target, pname);
        if let Some(number) = value.as_f64() {
            unsafe { *params = number as GLint };
        }
    });
}
