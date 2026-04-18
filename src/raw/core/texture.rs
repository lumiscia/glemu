use super::*;

pub(crate) unsafe extern "C" fn gl_active_texture(texture: GLenum) {
    with_gl(|s| s.gl.active_texture(texture));
}

pub(crate) unsafe extern "C" fn gl_bind_texture(target: GLenum, texture: GLuint) {
    with_gl(|s| {
        let texture = (texture != 0)
            .then(|| s.textures.get(&Texture::from(texture)))
            .flatten();
        s.gl.bind_texture(target, texture);
    });
}

pub(crate) unsafe extern "C" fn gl_copy_tex_image_2d(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
) {
    with_gl(|s| {
        s.gl.copy_tex_image_2d(target, level, internalformat, x, y, width, height, border)
    });
}

pub(crate) unsafe extern "C" fn gl_copy_tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl(|s| {
        s.gl.copy_tex_sub_image_2d(target, level, xoffset, yoffset, x, y, width, height)
    });
}

pub(crate) unsafe extern "C" fn gl_compressed_tex_image_2d(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    image_size: GLsizei,
    data: *const c_void,
) {
    with_gl(|s| {
        if data.is_null() || image_size <= 0 {
            s.gl.compressed_tex_image_2d_with_u8_array(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                &[],
            );
            return;
        }

        let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), image_size as usize) };
        s.gl.compressed_tex_image_2d_with_u8_array(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            bytes,
        );
    });
}

pub(crate) unsafe extern "C" fn gl_compressed_tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    image_size: GLsizei,
    data: *const c_void,
) {
    with_gl(|s| {
        if data.is_null() || image_size <= 0 {
            let mut empty = Vec::<u8>::new();
            s.gl.compressed_tex_sub_image_2d_with_u8_array(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                empty.as_mut_slice(),
            );
            return;
        }

        let bytes = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), image_size as usize) };
        let mut owned = bytes.to_vec();
        s.gl.compressed_tex_sub_image_2d_with_u8_array(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            owned.as_mut_slice(),
        );
    });
}

pub(crate) unsafe extern "C" fn gl_delete_textures(n: GLsizei, textures: *const GLuint) {
    with_gl(|s| {
        for i in 0..n as usize {
            let id = unsafe { *textures.add(i) };
            if let Some(texture) = s.textures.remove(&Texture::from(id)) {
                s.gl.delete_texture(Some(&texture));
            }
        }
    });
}

gen_objects!(gl_gen_textures, create_texture, textures, Texture);

pub(crate) unsafe extern "C" fn gl_is_texture(texture: GLuint) -> GLboolean {
    with_gl(|s| {
        s.textures
            .get(&Texture::from(texture))
            .map(|texture| {
                if s.gl.is_texture(Some(texture)) {
                    GL_TRUE
                } else {
                    GL_FALSE
                }
            })
            .unwrap_or(GL_FALSE)
    })
}

pub(crate) unsafe extern "C" fn gl_pixel_store_i(pname: GLenum, param: GLint) {
    with_gl(|s| s.gl.pixel_storei(pname, param));
}

pub(crate) unsafe extern "C" fn gl_tex_image_2d(
    target: GLenum,
    level: GLint,
    internal_format: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    with_gl(|s| {
        let data = if pixels.is_null() {
            None
        } else {
            let len = pixel_byte_size(width, height, format, type_);
            Some(unsafe { std::slice::from_raw_parts(pixels.cast::<u8>(), len) })
        };
        let _ =
            s.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                type_,
                data,
            );
    });
}

pub(crate) unsafe extern "C" fn gl_tex_parameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    with_gl(|s| s.gl.tex_parameterf(target, pname, param));
}

pub(crate) unsafe extern "C" fn gl_tex_parameterfv(
    target: GLenum,
    pname: GLenum,
    params: *const GLfloat,
) {
    with_gl(|s| {
        if !params.is_null() {
            s.gl.tex_parameterf(target, pname, unsafe { *params });
        }
    });
}

pub(crate) unsafe extern "C" fn gl_tex_parameteri(target: GLenum, pname: GLenum, param: GLint) {
    with_gl(|s| s.gl.tex_parameteri(target, pname, param));
}

pub(crate) unsafe extern "C" fn gl_tex_parameteriv(
    target: GLenum,
    pname: GLenum,
    params: *const GLint,
) {
    with_gl(|s| {
        if !params.is_null() {
            s.gl.tex_parameteri(target, pname, unsafe { *params });
        }
    });
}

pub(crate) unsafe extern "C" fn gl_tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    with_gl(|s| {
        let len = pixel_byte_size(width, height, format, type_);
        let bytes = unsafe { std::slice::from_raw_parts(pixels.cast::<u8>(), len) };
        let _ =
            s.gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                type_,
                Some(bytes),
            );
    });
}

pub(crate) unsafe extern "C" fn gl_generate_mipmap(target: GLenum) {
    with_gl(|s| s.gl.generate_mipmap(target));
}

pub(crate) unsafe extern "C" fn gl_tex_storage_2d(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl(|s| {
        s.gl.tex_storage_2d(target, levels, internalformat, width, height)
    });
}
