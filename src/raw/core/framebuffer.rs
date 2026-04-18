use super::*;

pub(crate) unsafe extern "C" fn gl_bind_framebuffer(target: GLenum, framebuffer: GLuint) {
    with_gl(|s| {
        let framebuffer = (framebuffer != 0)
            .then(|| s.framebuffers.get(&Framebuffer::from(framebuffer)))
            .flatten();
        s.gl.bind_framebuffer(target, framebuffer);
    });
}

pub(crate) unsafe extern "C" fn gl_bind_renderbuffer(target: GLenum, renderbuffer: GLuint) {
    with_gl(|s| {
        let renderbuffer = (renderbuffer != 0)
            .then(|| s.renderbuffers.get(&Renderbuffer::from(renderbuffer)))
            .flatten();
        s.gl.bind_renderbuffer(target, renderbuffer);
    });
}

pub(crate) unsafe extern "C" fn gl_blit_framebuffer(
    src_x0: GLint,
    src_y0: GLint,
    src_x1: GLint,
    src_y1: GLint,
    dst_x0: GLint,
    dst_y0: GLint,
    dst_x1: GLint,
    dst_y1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    with_gl(|s| {
        s.gl.blit_framebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        )
    });
}

pub(crate) unsafe extern "C" fn gl_check_framebuffer_status(target: GLenum) -> GLenum {
    with_gl(|s| s.gl.check_framebuffer_status(target))
}

pub(crate) unsafe extern "C" fn gl_delete_framebuffers(n: GLsizei, framebuffers: *const GLuint) {
    with_gl(|s| {
        for i in 0..n as usize {
            let id = unsafe { *framebuffers.add(i) };
            if let Some(framebuffer) = s.framebuffers.remove(&Framebuffer::from(id)) {
                s.gl.delete_framebuffer(Some(&framebuffer));
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_delete_renderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    with_gl(|s| {
        for i in 0..n as usize {
            let id = unsafe { *renderbuffers.add(i) };
            if let Some(renderbuffer) = s.renderbuffers.remove(&Renderbuffer::from(id)) {
                s.gl.delete_renderbuffer(Some(&renderbuffer));
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_framebuffer_renderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    with_gl(|s| {
        let renderbuffer = (renderbuffer != 0)
            .then(|| s.renderbuffers.get(&Renderbuffer::from(renderbuffer)))
            .flatten();
        s.gl.framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer);
    });
}

pub(crate) unsafe extern "C" fn gl_framebuffer_texture_2d(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    with_gl(|s| {
        let texture = (texture != 0)
            .then(|| s.textures.get(&Texture::from(texture)))
            .flatten();
        s.gl.framebuffer_texture_2d(target, attachment, textarget, texture, level);
    });
}

gen_objects!(
    gl_gen_framebuffers,
    create_framebuffer,
    framebuffers,
    Framebuffer
);

gen_objects!(
    gl_gen_renderbuffers,
    create_renderbuffer,
    renderbuffers,
    Renderbuffer
);

pub(crate) unsafe extern "C" fn gl_get_framebuffer_attachment_parameter_iv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_gl(|s| {
        if params.is_null() {
            return;
        }
        unsafe { *params = 0 };
        if let Ok(value) =
            s.gl.get_framebuffer_attachment_parameter(target, attachment, pname)
        {
            if let Some(number) = value.as_f64() {
                unsafe { *params = number as GLint };
            } else if let Some(flag) = value.as_bool() {
                unsafe { *params = flag as GLint };
            } else if value.is_object() {
                unsafe { *params = 1 };
            }
        }
    });
}

pub(crate) unsafe extern "C" fn gl_get_renderbuffer_parameter_iv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_gl(|s| {
        let value = s.gl.get_renderbuffer_parameter(target, pname);
        unsafe {
            *params = value
                .as_f64()
                .map(|number| number as GLint)
                .or_else(|| value.as_bool().map(|flag| flag as GLint))
                .unwrap_or_default();
        }
    });
}

pub(crate) unsafe extern "C" fn gl_invalidate_framebuffer(
    target: GLenum,
    num_attachments: GLsizei,
    attachments: *const GLenum,
) {
    with_gl(|s| {
        let array = js_sys::Array::new();
        for i in 0..num_attachments as usize {
            let attachment = unsafe { *attachments.add(i) };
            array.push(&wasm_bindgen::JsValue::from_f64(attachment as f64));
        }
        let _ = s.gl.invalidate_framebuffer(target, &array);
    });
}

pub(crate) unsafe extern "C" fn gl_renderbuffer_storage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl(|s| {
        s.gl.renderbuffer_storage(target, internalformat, width, height)
    });
}

pub(crate) unsafe extern "C" fn gl_renderbuffer_storage_multisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl(|s| {
        s.gl.renderbuffer_storage_multisample(target, samples, internalformat, width, height)
    });
}
