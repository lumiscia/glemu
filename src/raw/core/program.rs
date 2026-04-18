use super::*;

pub(crate) unsafe extern "C" fn gl_attach_shader(program: GLuint, shader: GLuint) {
    with_gl(|s| {
        if let (Some(program), Some(shader)) = (
            s.programs.get(&Program::from(program)),
            s.shaders.get(&Shader::from(shader)),
        ) {
            s.gl.attach_shader(program, shader);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_bind_attrib_location(
    program: GLuint,
    index: GLuint,
    name: *const c_char,
) {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            s.gl.bind_attrib_location(program, index, name);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_compile_shader(shader: GLuint) {
    with_gl(|s| {
        if let Some(shader) = s.shaders.get(&Shader::from(shader)) {
            s.gl.compile_shader(shader);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_create_program() -> GLuint {
    with_gl(|s| match s.gl.create_program() {
        Some(program) => {
            let handle = s.alloc_handle::<Program>();
            s.programs.insert(handle, program);
            handle.get()
        }
        None => 0,
    })
}

pub(crate) unsafe extern "C" fn gl_create_shader(shader_type: GLenum) -> GLuint {
    with_gl(|s| match s.gl.create_shader(shader_type) {
        Some(shader) => {
            let handle = s.alloc_handle::<Shader>();
            s.shaders.insert(handle, shader);
            handle.get()
        }
        None => 0,
    })
}

pub(crate) unsafe extern "C" fn gl_delete_program(program: GLuint) {
    with_gl(|s| {
        if let Some(program) = s.programs.remove(&Program::from(program)) {
            s.gl.delete_program(Some(&program));
        }
    });
}

pub(crate) unsafe extern "C" fn gl_detach_shader(program: GLuint, shader: GLuint) {
    with_gl(|s| {
        if let (Some(program), Some(shader)) = (
            s.programs.get(&Program::from(program)),
            s.shaders.get(&Shader::from(shader)),
        ) {
            s.gl.detach_shader(program, shader);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_delete_shader(shader: GLuint) {
    with_gl(|s| {
        if let Some(shader) = s.shaders.remove(&Shader::from(shader)) {
            s.gl.delete_shader(Some(&shader));
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_program_info_log(
    program: GLuint,
    max_length: GLsizei,
    length: *mut GLsizei,
    info_log: *mut c_char,
) {
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            let log = s.gl.get_program_info_log(program).unwrap_or_default();
            copy_info_log(
                log.as_bytes(),
                max_length,
                length,
                info_log.cast::<GLchar>(),
            );
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_program_iv(
    program: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            let value = s.gl.get_program_parameter(program, pname);
            unsafe {
                *params = value
                    .as_f64()
                    .map(|number| number as GLint)
                    .or_else(|| value.as_bool().map(|flag| flag as GLint))
                    .unwrap_or_default();
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_shader_info_log(
    shader: GLuint,
    max_length: GLsizei,
    length: *mut GLsizei,
    info_log: *mut c_char,
) {
    with_gl(|s| {
        if let Some(shader) = s.shaders.get(&Shader::from(shader)) {
            let log = s.gl.get_shader_info_log(shader).unwrap_or_default();
            copy_info_log(
                log.as_bytes(),
                max_length,
                length,
                info_log.cast::<GLchar>(),
            );
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_shaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    with_gl(|s| {
        if let Some(shader) = s.shaders.get(&Shader::from(shader)) {
            let value = s.gl.get_shader_parameter(shader, pname);
            unsafe {
                *params = value
                    .as_f64()
                    .map(|number| number as GLint)
                    .or_else(|| value.as_bool().map(|flag| flag as GLint))
                    .unwrap_or_default();
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_shader_precision_format(
    _shader_type: GLenum,
    _precision_type: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    unsafe {
        *range = 127;
        *range.add(1) = 127;
        *precision = 23;
    }
}

pub(crate) unsafe extern "C" fn gl_get_uniform_location(
    program: GLuint,
    name: *const c_char,
) -> GLint {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            match s.gl.get_uniform_location(program, name) {
                Some(location) => {
                    let handle = s.alloc_handle::<UniformLocation>();
                    s.uniform_locations.insert(handle, location);
                    handle.get() as GLint
                }
                None => -1,
            }
        } else {
            -1
        }
    })
}

pub(crate) unsafe extern "C" fn gl_link_program(program: GLuint) {
    with_gl(|s| {
        if let Some(program) = s.programs.get(&Program::from(program)) {
            s.gl.link_program(program);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_shader_source(
    shader: GLuint,
    count: GLsizei,
    strings: *const *const c_char,
    lengths: *const GLint,
) {
    with_gl(|s| {
        if let Some(shader) = s.shaders.get(&Shader::from(shader)) {
            let mut source = String::new();
            for index in 0..count as usize {
                let ptr = unsafe { *strings.add(index) };
                let chunk = if lengths.is_null() || unsafe { *lengths.add(index) } < 0 {
                    unsafe { CStr::from_ptr(ptr) }
                        .to_string_lossy()
                        .into_owned()
                } else {
                    let len = unsafe { *lengths.add(index) } as usize;
                    let bytes = unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), len) };
                    String::from_utf8_lossy(bytes).into_owned()
                };
                source.push_str(&chunk);
            }
            s.gl.shader_source(shader, &source);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_use_program(program: GLuint) {
    with_gl(|s| {
        let program = (program != 0)
            .then(|| s.programs.get(&Program::from(program)))
            .flatten();
        s.gl.use_program(program);
    });
}

pub(crate) unsafe extern "C" fn gl_get_attrib_location(
    program: GLuint,
    name: *const c_char,
) -> GLint {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    with_gl(|s| {
        s.programs
            .get(&Program::from(program))
            .map(|program| s.gl.get_attrib_location(program, name))
            .unwrap_or(-1)
    })
}
