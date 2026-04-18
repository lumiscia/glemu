use super::{core, es3};
use std::os::raw::c_void;

pub fn get_proc_address(name: &str) -> *const c_void {
    match name {
        "glActiveTexture" => core::gl_active_texture as *const c_void,
        "glAttachShader" => core::gl_attach_shader as *const c_void,
        "glBindAttribLocation" => core::gl_bind_attrib_location as *const c_void,
        "glBindBuffer" => core::gl_bind_buffer as *const c_void,
        "glBindFramebuffer" => core::gl_bind_framebuffer as *const c_void,
        "glBindRenderbuffer" => core::gl_bind_renderbuffer as *const c_void,
        "glBindTexture" => core::gl_bind_texture as *const c_void,
        "glBindVertexArray" | "glBindVertexArrayOES" => core::gl_bind_vertex_array as *const c_void,
        "glBlendColor" => core::gl_blend_color as *const c_void,
        "glBlendEquation" => core::gl_blend_equation as *const c_void,
        "glBlendEquationSeparate" => core::gl_blend_equation_separate as *const c_void,
        "glBlendFunc" => core::gl_blend_func as *const c_void,
        "glBlendFuncSeparate" => core::gl_blend_func_separate as *const c_void,
        "glBlitFramebuffer" => core::gl_blit_framebuffer as *const c_void,
        "glBufferData" => core::gl_buffer_data as *const c_void,
        "glBufferSubData" => core::gl_buffer_sub_data as *const c_void,
        "glCheckFramebufferStatus" => core::gl_check_framebuffer_status as *const c_void,
        "glClear" => core::gl_clear as *const c_void,
        "glClearColor" => core::gl_clear_color as *const c_void,
        "glClearDepthf" => core::gl_clear_depthf as *const c_void,
        "glClearStencil" => core::gl_clear_stencil as *const c_void,
        "glColorMask" => core::gl_color_mask as *const c_void,
        "glCompileShader" => core::gl_compile_shader as *const c_void,
        "glCompressedTexImage2D" => core::gl_compressed_tex_image_2d as *const c_void,
        "glCompressedTexSubImage2D" => core::gl_compressed_tex_sub_image_2d as *const c_void,
        "glCopyTexImage2D" => core::gl_copy_tex_image_2d as *const c_void,
        "glCopyTexSubImage2D" => core::gl_copy_tex_sub_image_2d as *const c_void,
        "glCreateProgram" => core::gl_create_program as *const c_void,
        "glCreateShader" => core::gl_create_shader as *const c_void,
        "glCullFace" => core::gl_cull_face as *const c_void,
        "glDeleteBuffers" => core::gl_delete_buffers as *const c_void,
        "glDeleteFramebuffers" => core::gl_delete_framebuffers as *const c_void,
        "glDeleteProgram" => core::gl_delete_program as *const c_void,
        "glDetachShader" => core::gl_detach_shader as *const c_void,
        "glDeleteQueries" | "glDeleteQueriesEXT" => core::gl_delete_queries as *const c_void,
        "glDeleteRenderbuffers" => core::gl_delete_renderbuffers as *const c_void,
        "glDeleteShader" => core::gl_delete_shader as *const c_void,
        "glDeleteTextures" => core::gl_delete_textures as *const c_void,
        "glDeleteVertexArrays" | "glDeleteVertexArraysOES" => {
            core::gl_delete_vertex_arrays as *const c_void
        }
        "glDepthMask" => core::gl_depth_mask as *const c_void,
        "glDisable" => core::gl_disable as *const c_void,
        "glDisableVertexAttribArray" => core::gl_disable_vertex_attrib_array as *const c_void,
        "glDrawArrays" => core::gl_draw_arrays as *const c_void,
        "glDrawElements" => core::gl_draw_elements as *const c_void,
        "glEnable" => core::gl_enable as *const c_void,
        "glEnableVertexAttribArray" => core::gl_enable_vertex_attrib_array as *const c_void,
        "glFinish" => core::gl_finish as *const c_void,
        "glFlush" => core::gl_flush as *const c_void,
        "glFramebufferRenderbuffer" => core::gl_framebuffer_renderbuffer as *const c_void,
        "glFramebufferTexture2D" => core::gl_framebuffer_texture_2d as *const c_void,
        "glFrontFace" => core::gl_front_face as *const c_void,
        "glGenBuffers" => core::gl_gen_buffers as *const c_void,
        "glGenFramebuffers" => core::gl_gen_framebuffers as *const c_void,
        "glGenQueries" | "glGenQueriesEXT" => core::gl_gen_queries as *const c_void,
        "glGenRenderbuffers" => core::gl_gen_renderbuffers as *const c_void,
        "glGenTextures" => core::gl_gen_textures as *const c_void,
        "glGenVertexArrays" | "glGenVertexArraysOES" => core::gl_gen_vertex_arrays as *const c_void,
        "glGetBooleanv" => core::gl_get_booleanv as *const c_void,
        "glGetAttribLocation" => core::gl_get_attrib_location as *const c_void,
        "glGetError" => core::gl_get_error as *const c_void,
        "glGetFramebufferAttachmentParameteriv" => {
            core::gl_get_framebuffer_attachment_parameter_iv as *const c_void
        }
        "glGetIntegerv" => core::gl_get_integerv as *const c_void,
        "glGetProgramInfoLog" => core::gl_get_program_info_log as *const c_void,
        "glGetProgramiv" => core::gl_get_program_iv as *const c_void,
        "glGetRenderbufferParameteriv" => core::gl_get_renderbuffer_parameter_iv as *const c_void,
        "glGetShaderInfoLog" => core::gl_get_shader_info_log as *const c_void,
        "glGetShaderiv" => core::gl_get_shaderiv as *const c_void,
        "glGetShaderPrecisionFormat" => core::gl_get_shader_precision_format as *const c_void,
        "glGetString" => core::gl_get_string as *const c_void,
        "glGetStringi" => core::gl_get_stringi as *const c_void,
        "glGetUniformLocation" => core::gl_get_uniform_location as *const c_void,
        "glInvalidateFramebuffer" => core::gl_invalidate_framebuffer as *const c_void,
        "glIsTexture" => core::gl_is_texture as *const c_void,
        "glLineWidth" => core::gl_line_width as *const c_void,
        "glLinkProgram" => core::gl_link_program as *const c_void,
        "glPixelStorei" => core::gl_pixel_store_i as *const c_void,
        "glReadPixels" => core::gl_read_pixels as *const c_void,
        "glRenderbufferStorage" => core::gl_renderbuffer_storage as *const c_void,
        "glRenderbufferStorageMultisample" => {
            core::gl_renderbuffer_storage_multisample as *const c_void
        }
        "glScissor" => core::gl_scissor as *const c_void,
        "glShaderSource" => core::gl_shader_source as *const c_void,
        "glStencilFunc" => core::gl_stencil_func as *const c_void,
        "glStencilFuncSeparate" => core::gl_stencil_func_separate as *const c_void,
        "glStencilMask" => core::gl_stencil_mask as *const c_void,
        "glStencilMaskSeparate" => core::gl_stencil_mask_separate as *const c_void,
        "glStencilOp" => core::gl_stencil_op as *const c_void,
        "glStencilOpSeparate" => core::gl_stencil_op_separate as *const c_void,
        "glTexImage2D" => core::gl_tex_image_2d as *const c_void,
        "glTexParameterf" => core::gl_tex_parameterf as *const c_void,
        "glTexParameterfv" => core::gl_tex_parameterfv as *const c_void,
        "glTexParameteri" => core::gl_tex_parameteri as *const c_void,
        "glTexParameteriv" => core::gl_tex_parameteriv as *const c_void,
        "glTexSubImage2D" => core::gl_tex_sub_image_2d as *const c_void,
        "glUniform1f" => core::gl_uniform1f as *const c_void,
        "glUniform1fv" => core::gl_uniform1fv as *const c_void,
        "glUniform1i" => core::gl_uniform1i as *const c_void,
        "glUniform1iv" => core::gl_uniform1iv as *const c_void,
        "glUniform2f" => core::gl_uniform2f as *const c_void,
        "glUniform2fv" => core::gl_uniform2fv as *const c_void,
        "glUniform2i" => core::gl_uniform2i as *const c_void,
        "glUniform2iv" => core::gl_uniform2iv as *const c_void,
        "glUniform3f" => core::gl_uniform3f as *const c_void,
        "glUniform3fv" => core::gl_uniform3fv as *const c_void,
        "glUniform3i" => core::gl_uniform3i as *const c_void,
        "glUniform3iv" => core::gl_uniform3iv as *const c_void,
        "glUniform4f" => core::gl_uniform4f as *const c_void,
        "glUniform4fv" => core::gl_uniform4fv as *const c_void,
        "glUniform4i" => core::gl_uniform4i as *const c_void,
        "glUniform4iv" => core::gl_uniform4iv as *const c_void,
        "glUniformMatrix2fv" => core::gl_uniform_matrix2fv as *const c_void,
        "glUniformMatrix3fv" => core::gl_uniform_matrix3fv as *const c_void,
        "glUniformMatrix4fv" => core::gl_uniform_matrix4fv as *const c_void,
        "glUseProgram" => core::gl_use_program as *const c_void,
        "glVertexAttribPointer" => core::gl_vertex_attrib_pointer as *const c_void,
        "glViewport" => core::gl_viewport as *const c_void,
        "glBeginQuery" | "glBeginQueryEXT" => core::gl_begin_query as *const c_void,
        "glEndQuery" | "glEndQueryEXT" => core::gl_end_query as *const c_void,
        "glGetQueryObjectuiv" | "glGetQueryObjectuivEXT" => {
            core::gl_get_query_object_uiv as *const c_void
        }
        "glGenerateMipmap" => core::gl_generate_mipmap as *const c_void,
        "glTexStorage2D" => core::gl_tex_storage_2d as *const c_void,
        "glVertexAttrib1f" => core::gl_vertex_attrib1f as *const c_void,
        "glVertexAttrib2fv" => core::gl_vertex_attrib2fv as *const c_void,
        "glVertexAttrib3fv" => core::gl_vertex_attrib3fv as *const c_void,
        "glVertexAttrib4fv" => core::gl_vertex_attrib4fv as *const c_void,
        "glGetBufferParameteriv" => core::gl_get_buffer_parameteriv as *const c_void,
        "glIsEnabled" => core::gl_is_enabled as *const c_void,
        "glDepthFunc" => core::gl_depth_func as *const c_void,
        "glDepthRangef" => core::gl_depth_rangef as *const c_void,
        "glGetFloatv" => core::gl_get_floatv as *const c_void,
        "glDrawArraysInstanced" => es3::gl_draw_arrays_instanced as *const c_void,
        "glDrawElementsInstanced" => es3::gl_draw_elements_instanced as *const c_void,
        "glVertexAttribDivisor" => es3::gl_vertex_attrib_divisor as *const c_void,
        "glDrawRangeElements" => es3::gl_draw_range_elements as *const c_void,
        "glVertexAttribIPointer" => es3::gl_vertex_attrib_i_pointer as *const c_void,
        "glDrawBuffers" => es3::gl_draw_buffers as *const c_void,
        "glReadBuffer" => es3::gl_read_buffer as *const c_void,
        "glBindBufferBase" => es3::gl_bind_buffer_base as *const c_void,
        "glBindBufferRange" => es3::gl_bind_buffer_range as *const c_void,
        "glCopyBufferSubData" => es3::gl_copy_buffer_sub_data as *const c_void,
        "glInvalidateSubFramebuffer" => es3::gl_invalidate_sub_framebuffer as *const c_void,
        "glGetBufferSubData" => es3::gl_get_buffer_sub_data as *const c_void,
        "glMapBufferRange" => es3::gl_map_buffer_range as *const c_void,
        "glUnmapBuffer" => es3::gl_unmap_buffer as *const c_void,
        "glFlushMappedBufferRange" => es3::gl_flush_mapped_buffer_range as *const c_void,
        "glFenceSync" => es3::gl_fence_sync as *const c_void,
        "glClientWaitSync" => es3::gl_client_wait_sync as *const c_void,
        "glDeleteSync" => es3::gl_delete_sync as *const c_void,
        "glIsSync" => es3::gl_is_sync as *const c_void,
        "glWaitSync" => es3::gl_wait_sync as *const c_void,
        "glGenSamplers" => es3::gl_gen_samplers as *const c_void,
        "glDeleteSamplers" => es3::gl_delete_samplers as *const c_void,
        "glBindSampler" => es3::gl_bind_sampler as *const c_void,
        "glSamplerParameteri" => es3::gl_sampler_parameteri as *const c_void,
        "glSamplerParameterf" => es3::gl_sampler_parameterf as *const c_void,
        "glSamplerParameteriv" => es3::gl_sampler_parameter_iv as *const c_void,
        "glGenTransformFeedbacks" => es3::gl_gen_transform_feedbacks as *const c_void,
        "glDeleteTransformFeedbacks" => es3::gl_delete_transform_feedbacks as *const c_void,
        "glBindTransformFeedback" => es3::gl_bind_transform_feedback as *const c_void,
        "glIsTransformFeedback" => es3::gl_is_transform_feedback as *const c_void,
        "glBeginTransformFeedback" => es3::gl_begin_transform_feedback as *const c_void,
        "glEndTransformFeedback" => es3::gl_end_transform_feedback as *const c_void,
        "glPauseTransformFeedback" => es3::gl_pause_transform_feedback as *const c_void,
        "glResumeTransformFeedback" => es3::gl_resume_transform_feedback as *const c_void,
        "glTransformFeedbackVaryings" => es3::gl_transform_feedback_varyings as *const c_void,
        "glGetUniformBlockIndex" => es3::gl_get_uniform_block_index as *const c_void,
        "glUniformBlockBinding" => es3::gl_uniform_block_binding as *const c_void,
        "glGetActiveUniformBlockiv" => es3::gl_get_active_uniform_block_iv as *const c_void,
        "glGetActiveUniformBlockName" => es3::gl_get_active_uniform_block_name as *const c_void,
        "glTexImage3D" => es3::gl_tex_image_3d as *const c_void,
        "glTexSubImage3D" => es3::gl_tex_sub_image_3d as *const c_void,
        "glTexStorage3D" => es3::gl_tex_storage_3d as *const c_void,
        "glGetQueryiv" => es3::gl_get_queryiv as *const c_void,
        "glGetInternalformativ" => es3::gl_get_internalformativ as *const c_void,
        "glGetProgramBinary" => es3::gl_get_program_binary as *const c_void,
        "glProgramBinary" => es3::gl_program_binary as *const c_void,
        "glProgramParameteri" => es3::gl_program_parameteri as *const c_void,
        _ => std::ptr::null(),
    }
}

#[cfg(test)]
mod tests {
    use super::get_proc_address;

    #[test]
    fn resolves_known_aliases() {
        assert_eq!(
            get_proc_address("glBindVertexArray"),
            get_proc_address("glBindVertexArrayOES")
        );
        assert_eq!(
            get_proc_address("glDeleteQueries"),
            get_proc_address("glDeleteQueriesEXT")
        );
        assert_eq!(
            get_proc_address("glGetQueryObjectuiv"),
            get_proc_address("glGetQueryObjectuivEXT")
        );
    }

    #[test]
    fn resolves_known_and_unknown_symbols() {
        assert!(!get_proc_address("glGetString").is_null());
        assert!(get_proc_address("glDefinitelyMissing").is_null());
    }
}
