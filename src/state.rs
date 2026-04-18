use crate::helpers::{gl_parameter_string, nul_terminated_bytes, supported_extension_bytes};
use crate::types::{
    Buffer, Framebuffer, GLintptr, GLuint, Program, Query, Renderbuffer, ResourceHandle, Sampler,
    Shader, SyncHandle, Texture, TransformFeedback, UniformLocation, VertexArray,
};
use std::collections::HashMap;
use web_sys::{
    WebGl2RenderingContext as Gl, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlQuery,
    WebGlRenderbuffer, WebGlSampler, WebGlShader, WebGlSync, WebGlTexture, WebGlTransformFeedback,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct MappedBuffer {
    pub(crate) target: u32,
    pub(crate) offset: GLintptr,
    pub(crate) data: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CachedStrings {
    pub(crate) version: Vec<u8>,
    pub(crate) vendor: Vec<u8>,
    pub(crate) renderer: Vec<u8>,
    pub(crate) shading_language_version: Vec<u8>,
    pub(crate) extensions: Vec<Vec<u8>>,
}

pub(crate) struct ContextState {
    pub(crate) gl: Gl,
    pub(crate) textures: HashMap<Texture, WebGlTexture>,
    pub(crate) buffers: HashMap<Buffer, WebGlBuffer>,
    pub(crate) programs: HashMap<Program, WebGlProgram>,
    pub(crate) shaders: HashMap<Shader, WebGlShader>,
    pub(crate) framebuffers: HashMap<Framebuffer, WebGlFramebuffer>,
    pub(crate) renderbuffers: HashMap<Renderbuffer, WebGlRenderbuffer>,
    pub(crate) vertex_arrays: HashMap<VertexArray, WebGlVertexArrayObject>,
    pub(crate) queries: HashMap<Query, WebGlQuery>,
    pub(crate) samplers: HashMap<Sampler, WebGlSampler>,
    pub(crate) syncs: HashMap<SyncHandle, WebGlSync>,
    pub(crate) uniform_locations: HashMap<UniformLocation, WebGlUniformLocation>,
    pub(crate) transform_feedbacks: HashMap<TransformFeedback, WebGlTransformFeedback>,
    pub(crate) cached_strings: CachedStrings,
    pub(crate) next_resource_id: GLuint,
    pub(crate) mapped_buffer: Option<MappedBuffer>,
}

impl ContextState {
    pub(crate) fn new(gl: Gl) -> Self {
        Self {
            cached_strings: CachedStrings {
                version: nul_terminated_bytes("OpenGL ES 3.0".to_owned()),
                vendor: gl_parameter_string(&gl, Gl::VENDOR, "WebGL"),
                renderer: gl_parameter_string(&gl, Gl::RENDERER, "WebGL2 Renderer"),
                shading_language_version: nul_terminated_bytes("OpenGL ES GLSL ES 3.00".to_owned()),
                extensions: supported_extension_bytes(&gl),
            },
            gl,
            textures: HashMap::new(),
            buffers: HashMap::new(),
            programs: HashMap::new(),
            shaders: HashMap::new(),
            framebuffers: HashMap::new(),
            renderbuffers: HashMap::new(),
            vertex_arrays: HashMap::new(),
            queries: HashMap::new(),
            samplers: HashMap::new(),
            syncs: HashMap::new(),
            uniform_locations: HashMap::new(),
            transform_feedbacks: HashMap::new(),
            next_resource_id: 1,
            mapped_buffer: None,
        }
    }

    pub(crate) fn alloc_id(&mut self) -> GLuint {
        let id = self.next_resource_id;
        self.next_resource_id = self
            .next_resource_id
            .checked_add(1)
            .expect("resource id overflowed");
        id
    }

    pub(crate) fn alloc_handle<H>(&mut self) -> H
    where
        H: ResourceHandle,
    {
        H::new(self.alloc_id())
    }
}
