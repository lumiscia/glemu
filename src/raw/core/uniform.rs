use super::*;

uniform_scalar!(gl_uniform1f, uniform1f, GLfloat);

uniform_scalar!(gl_uniform1i, uniform1i, GLint);

uniform_vec!(gl_uniform1fv, uniform1fv_with_f32_array, GLfloat, 1);

uniform_vec!(gl_uniform1iv, uniform1iv_with_i32_array, GLint, 1);

pub(crate) unsafe extern "C" fn gl_uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    with_gl(|s| {
        if let Some(loc) = s
            .uniform_locations
            .get(&UniformLocation::from(location as GLuint))
        {
            s.gl.uniform2f(Some(loc), v0, v1);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_uniform2i(location: GLint, v0: GLint, v1: GLint) {
    with_gl(|s| {
        if let Some(loc) = s
            .uniform_locations
            .get(&UniformLocation::from(location as GLuint))
        {
            s.gl.uniform2i(Some(loc), v0, v1);
        }
    });
}

uniform_vec!(gl_uniform2fv, uniform2fv_with_f32_array, GLfloat, 2);

uniform_vec!(gl_uniform2iv, uniform2iv_with_i32_array, GLint, 2);

pub(crate) unsafe extern "C" fn gl_uniform3f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
) {
    with_gl(|s| {
        if let Some(loc) = s
            .uniform_locations
            .get(&UniformLocation::from(location as GLuint))
        {
            s.gl.uniform3f(Some(loc), v0, v1, v2);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    with_gl(|s| {
        if let Some(loc) = s
            .uniform_locations
            .get(&UniformLocation::from(location as GLuint))
        {
            s.gl.uniform3i(Some(loc), v0, v1, v2);
        }
    });
}

uniform_vec!(gl_uniform3fv, uniform3fv_with_f32_array, GLfloat, 3);

uniform_vec!(gl_uniform3iv, uniform3iv_with_i32_array, GLint, 3);

pub(crate) unsafe extern "C" fn gl_uniform4f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    with_gl(|s| {
        if let Some(loc) = s
            .uniform_locations
            .get(&UniformLocation::from(location as GLuint))
        {
            s.gl.uniform4f(Some(loc), v0, v1, v2, v3);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_uniform4i(
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
    v3: GLint,
) {
    with_gl(|s| {
        if let Some(loc) = s
            .uniform_locations
            .get(&UniformLocation::from(location as GLuint))
        {
            s.gl.uniform4i(Some(loc), v0, v1, v2, v3);
        }
    });
}

uniform_vec!(gl_uniform4fv, uniform4fv_with_f32_array, GLfloat, 4);

uniform_vec!(gl_uniform4iv, uniform4iv_with_i32_array, GLint, 4);

uniform_matrix!(gl_uniform_matrix2fv, uniform_matrix2fv_with_f32_array, 2);

uniform_matrix!(gl_uniform_matrix3fv, uniform_matrix3fv_with_f32_array, 3);

uniform_matrix!(gl_uniform_matrix4fv, uniform_matrix4fv_with_f32_array, 4);
