use super::*;

pub(crate) unsafe extern "C" fn gl_bind_vertex_array(array: GLuint) {
    with_gl(|s| {
        let array = (array != 0)
            .then(|| s.vertex_arrays.get(&VertexArray::from(array)))
            .flatten();
        s.gl.bind_vertex_array(array);
    });
}

pub(crate) unsafe extern "C" fn gl_delete_vertex_arrays(n: GLsizei, arrays: *const GLuint) {
    with_gl(|s| {
        for i in 0..n as usize {
            let id = unsafe { *arrays.add(i) };
            if let Some(array) = s.vertex_arrays.remove(&VertexArray::from(id)) {
                s.gl.delete_vertex_array(Some(&array));
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_disable_vertex_attrib_array(index: GLuint) {
    with_gl(|s| s.gl.disable_vertex_attrib_array(index));
}

pub(crate) unsafe extern "C" fn gl_enable_vertex_attrib_array(index: GLuint) {
    with_gl(|s| s.gl.enable_vertex_attrib_array(index));
}

gen_objects!(
    gl_gen_vertex_arrays,
    create_vertex_array,
    vertex_arrays,
    VertexArray
);

pub(crate) unsafe extern "C" fn gl_vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
) {
    with_gl(|s| {
        s.gl.vertex_attrib_pointer_with_i32(
            index,
            size,
            type_,
            normalized != 0,
            stride,
            pointer as i32,
        )
    });
}

pub(crate) unsafe extern "C" fn gl_vertex_attrib1f(index: GLuint, x: GLfloat) {
    with_gl(|s| s.gl.vertex_attrib1f(index, x));
}

pub(crate) unsafe extern "C" fn gl_vertex_attrib2fv(index: GLuint, v: *const GLfloat) {
    with_gl(|s| {
        let array = unsafe { std::slice::from_raw_parts(v, 2) };
        s.gl.vertex_attrib2fv_with_f32_array(index, array);
    });
}

pub(crate) unsafe extern "C" fn gl_vertex_attrib3fv(index: GLuint, v: *const GLfloat) {
    with_gl(|s| {
        let array = unsafe { std::slice::from_raw_parts(v, 3) };
        s.gl.vertex_attrib3fv_with_f32_array(index, array);
    });
}

pub(crate) unsafe extern "C" fn gl_vertex_attrib4fv(index: GLuint, v: *const GLfloat) {
    with_gl(|s| {
        let array = unsafe { std::slice::from_raw_parts(v, 4) };
        s.gl.vertex_attrib4fv_with_f32_array(index, array);
    });
}
