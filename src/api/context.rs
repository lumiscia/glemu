use crate::registry::{
    current_context_id, has_gl_context, register_gl_context, set_gl_context, with_context,
    with_context_mut,
};
use crate::state::ContextState;
use crate::types::{
    Buffer as RawBuffer, ContextId, Framebuffer as RawFramebuffer, Program as RawProgram,
    Query as RawQuery, Renderbuffer as RawRenderbuffer, ResourceHandle, Sampler as RawSampler,
    Shader as RawShader, Texture as RawTexture, TransformFeedback as RawTransformFeedback,
    VertexArray as RawVertexArray,
};
use wasm_bindgen::JsValue;
use web_sys::{
    WebGl2RenderingContext as Gl, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderbuffer,
    WebGlSampler, WebGlShader, WebGlTexture, WebGlTransformFeedback, WebGlVertexArrayObject,
};

macro_rules! define_handle {
    ($name:ident, $raw:ty, $kind:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name {
            context_id: ContextId,
            raw: $raw,
        }

        impl $name {
            pub(crate) const fn new(context_id: ContextId, raw: $raw) -> Self {
                Self { context_id, raw }
            }

            pub const fn context_id(self) -> ContextId {
                self.context_id
            }

            pub const fn raw_id(self) -> u32 {
                self.raw.get()
            }
        }

        impl From<$name> for u32 {
            fn from(value: $name) -> Self {
                value.raw.get()
            }
        }

        impl ApiHandle for $name {
            type Raw = $raw;

            const KIND: &'static str = $kind;

            fn context_id(self) -> ContextId {
                self.context_id
            }

            fn raw(self) -> Self::Raw {
                self.raw
            }
        }
    };
}

trait ApiHandle: Copy {
    type Raw: ResourceHandle;

    const KIND: &'static str;

    fn context_id(self) -> ContextId;
    fn raw(self) -> Self::Raw;
}

define_handle!(Buffer, RawBuffer, "buffer");
define_handle!(Framebuffer, RawFramebuffer, "framebuffer");
define_handle!(Program, RawProgram, "program");
define_handle!(Query, RawQuery, "query");
define_handle!(Renderbuffer, RawRenderbuffer, "renderbuffer");
define_handle!(Sampler, RawSampler, "sampler");
define_handle!(Shader, RawShader, "shader");
define_handle!(Texture, RawTexture, "texture");
define_handle!(
    TransformFeedback,
    RawTransformFeedback,
    "transform feedback"
);
define_handle!(VertexArray, RawVertexArray, "vertex array");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Context {
    context_id: ContextId,
}

impl Context {
    pub fn register(gl: Gl) -> Self {
        Self {
            context_id: register_gl_context(gl),
        }
    }

    pub fn from_webgl2_context(gl: Gl) -> Self {
        Self::register(gl)
    }

    pub fn from_context_id(context_id: ContextId) -> Option<Self> {
        has_gl_context(context_id).then_some(Self { context_id })
    }

    pub fn current() -> Option<Self> {
        current_context_id().and_then(Self::from_context_id)
    }

    pub const fn id(self) -> ContextId {
        self.context_id
    }

    pub fn make_current(&self) -> bool {
        set_gl_context(self.context_id)
    }

    pub fn webgl2_context(&self) -> Gl {
        self.with_state(|state| state.gl.clone())
    }

    pub fn create_buffer(&self) -> Option<Buffer> {
        self.with_state_mut(|state| {
            let buffer = state.gl.create_buffer()?;
            let raw = state.alloc_handle::<RawBuffer>();
            state.buffers.insert(raw, buffer);
            Some(Buffer::new(self.context_id, raw))
        })
    }

    pub fn bind_buffer(&self, target: u32, buffer: Option<Buffer>) {
        let buffer = buffer.map(|buffer| self.expect_handle(buffer));
        self.with_state_mut(|state| {
            let buffer = buffer.map(|raw| self.expect_live_buffer(state, raw));
            state.gl.bind_buffer(target, buffer);
        });
    }

    pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: Option<Buffer>) {
        let buffer = buffer.map(|buffer| self.expect_handle(buffer));
        self.with_state_mut(|state| {
            let buffer = buffer.map(|raw| self.expect_live_buffer(state, raw));
            state.gl.bind_buffer_base(target, index, buffer);
        });
    }

    pub fn bind_buffer_range(
        &self,
        target: u32,
        index: u32,
        buffer: Option<Buffer>,
        offset: i32,
        size: i32,
    ) {
        let buffer = buffer.map(|buffer| self.expect_handle(buffer));
        self.with_state_mut(|state| {
            let buffer = buffer.map(|raw| self.expect_live_buffer(state, raw));
            state
                .gl
                .bind_buffer_range_with_i32_and_i32(target, index, buffer, offset, size);
        });
    }

    pub fn get_buffer_sub_data(&self, target: u32, src_byte_offset: i32, dst_data: &mut [u8]) {
        self.with_state(|state| {
            state
                .gl
                .get_buffer_sub_data_with_i32_and_u8_array(target, src_byte_offset, dst_data);
        });
    }

    pub fn delete_buffer(&self, buffer: Buffer) {
        let raw = self.expect_handle(buffer);
        self.with_state_mut(|state| {
            let buffer = state
                .buffers
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Buffer>(raw.get()));
            state.gl.delete_buffer(Some(&buffer));
        });
    }

    pub fn create_framebuffer(&self) -> Option<Framebuffer> {
        self.with_state_mut(|state| {
            let framebuffer = state.gl.create_framebuffer()?;
            let raw = state.alloc_handle::<RawFramebuffer>();
            state.framebuffers.insert(raw, framebuffer);
            Some(Framebuffer::new(self.context_id, raw))
        })
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<Framebuffer>) {
        let framebuffer = framebuffer.map(|framebuffer| self.expect_handle(framebuffer));
        self.with_state_mut(|state| {
            let framebuffer = framebuffer.map(|raw| self.expect_live_framebuffer(state, raw));
            state.gl.bind_framebuffer(target, framebuffer);
        });
    }

    pub fn delete_framebuffer(&self, framebuffer: Framebuffer) {
        let raw = self.expect_handle(framebuffer);
        self.with_state_mut(|state| {
            let framebuffer = state
                .framebuffers
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Framebuffer>(raw.get()));
            state.gl.delete_framebuffer(Some(&framebuffer));
        });
    }

    pub fn create_program(&self) -> Option<Program> {
        self.with_state_mut(|state| {
            let program = state.gl.create_program()?;
            let raw = state.alloc_handle::<RawProgram>();
            state.programs.insert(raw, program);
            Some(Program::new(self.context_id, raw))
        })
    }

    pub fn use_program(&self, program: Option<Program>) {
        let program = program.map(|program| self.expect_handle(program));
        self.with_state_mut(|state| {
            let program = program.map(|raw| self.expect_live_program(state, raw));
            state.gl.use_program(program);
        });
    }

    pub fn delete_program(&self, program: Program) {
        let raw = self.expect_handle(program);
        self.with_state_mut(|state| {
            let program = state
                .programs
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Program>(raw.get()));
            state.gl.delete_program(Some(&program));
        });
    }

    pub fn attach_shader(&self, program: Program, shader: Shader) {
        let program = self.expect_handle(program);
        let shader = self.expect_handle(shader);
        self.with_state_mut(|state| {
            let program = self.expect_live_program(state, program);
            let shader = self.expect_live_shader(state, shader);
            state.gl.attach_shader(program, shader);
        });
    }

    pub fn detach_shader(&self, program: Program, shader: Shader) {
        let program = self.expect_handle(program);
        let shader = self.expect_handle(shader);
        self.with_state_mut(|state| {
            let program = self.expect_live_program(state, program);
            let shader = self.expect_live_shader(state, shader);
            state.gl.detach_shader(program, shader);
        });
    }

    pub fn get_attrib_location(&self, program: Program, name: &str) -> i32 {
        let program = self.expect_handle(program);
        self.with_state(|state| {
            let program = self.expect_live_program(state, program);
            state.gl.get_attrib_location(program, name)
        })
    }

    pub fn get_uniform_block_index(&self, program: Program, name: &str) -> u32 {
        let program = self.expect_handle(program);
        self.with_state(|state| {
            let program = self.expect_live_program(state, program);
            state.gl.get_uniform_block_index(program, name)
        })
    }

    pub fn uniform_block_binding(
        &self,
        program: Program,
        uniform_block_index: u32,
        uniform_block_binding: u32,
    ) {
        let program = self.expect_handle(program);
        self.with_state_mut(|state| {
            let program = self.expect_live_program(state, program);
            state
                .gl
                .uniform_block_binding(program, uniform_block_index, uniform_block_binding);
        });
    }

    pub fn active_uniform_block_name(
        &self,
        program: Program,
        uniform_block_index: u32,
    ) -> Option<String> {
        let program = self.expect_handle(program);
        self.with_state(|state| {
            let program = self.expect_live_program(state, program);
            state
                .gl
                .get_active_uniform_block_name(program, uniform_block_index)
        })
    }

    pub fn active_uniform_block_parameter(
        &self,
        program: Program,
        uniform_block_index: u32,
        pname: u32,
    ) -> Result<JsValue, JsValue> {
        let program = self.expect_handle(program);
        self.with_state(|state| {
            let program = self.expect_live_program(state, program);
            state
                .gl
                .get_active_uniform_block_parameter(program, uniform_block_index, pname)
        })
    }

    pub fn transform_feedback_varyings(
        &self,
        program: Program,
        varyings: &[&str],
        buffer_mode: u32,
    ) {
        let program = self.expect_handle(program);
        self.with_state_mut(|state| {
            let program = self.expect_live_program(state, program);
            let varyings =
                js_sys::Array::from_iter(varyings.iter().copied().map(JsValue::from_str));
            state
                .gl
                .transform_feedback_varyings(program, &varyings.into(), buffer_mode);
        });
    }

    pub fn link_program(&self, program: Program) {
        let program = self.expect_handle(program);
        self.with_state_mut(|state| {
            let program = self.expect_live_program(state, program);
            state.gl.link_program(program);
        });
    }

    pub fn create_query(&self) -> Option<Query> {
        self.with_state_mut(|state| {
            let query = state.gl.create_query()?;
            let raw = state.alloc_handle::<RawQuery>();
            state.queries.insert(raw, query);
            Some(Query::new(self.context_id, raw))
        })
    }

    pub fn delete_query(&self, query: Query) {
        let raw = self.expect_handle(query);
        self.with_state_mut(|state| {
            let query = state
                .queries
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Query>(raw.get()));
            state.gl.delete_query(Some(&query));
        });
    }

    pub fn create_renderbuffer(&self) -> Option<Renderbuffer> {
        self.with_state_mut(|state| {
            let renderbuffer = state.gl.create_renderbuffer()?;
            let raw = state.alloc_handle::<RawRenderbuffer>();
            state.renderbuffers.insert(raw, renderbuffer);
            Some(Renderbuffer::new(self.context_id, raw))
        })
    }

    pub fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<Renderbuffer>) {
        let renderbuffer = renderbuffer.map(|renderbuffer| self.expect_handle(renderbuffer));
        self.with_state_mut(|state| {
            let renderbuffer = renderbuffer.map(|raw| self.expect_live_renderbuffer(state, raw));
            state.gl.bind_renderbuffer(target, renderbuffer);
        });
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Renderbuffer) {
        let raw = self.expect_handle(renderbuffer);
        self.with_state_mut(|state| {
            let renderbuffer = state
                .renderbuffers
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Renderbuffer>(raw.get()));
            state.gl.delete_renderbuffer(Some(&renderbuffer));
        });
    }

    pub fn create_sampler(&self) -> Option<Sampler> {
        self.with_state_mut(|state| {
            let sampler = state.gl.create_sampler()?;
            let raw = state.alloc_handle::<RawSampler>();
            state.samplers.insert(raw, sampler);
            Some(Sampler::new(self.context_id, raw))
        })
    }

    pub fn bind_sampler(&self, unit: u32, sampler: Option<Sampler>) {
        let sampler = sampler.map(|sampler| self.expect_handle(sampler));
        self.with_state_mut(|state| {
            let sampler = sampler.map(|raw| self.expect_live_sampler(state, raw));
            state.gl.bind_sampler(unit, sampler);
        });
    }

    pub fn delete_sampler(&self, sampler: Sampler) {
        let raw = self.expect_handle(sampler);
        self.with_state_mut(|state| {
            let sampler = state
                .samplers
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Sampler>(raw.get()));
            state.gl.delete_sampler(Some(&sampler));
        });
    }

    pub fn create_shader(&self, shader_type: u32) -> Option<Shader> {
        self.with_state_mut(|state| {
            let shader = state.gl.create_shader(shader_type)?;
            let raw = state.alloc_handle::<RawShader>();
            state.shaders.insert(raw, shader);
            Some(Shader::new(self.context_id, raw))
        })
    }

    pub fn shader_source(&self, shader: Shader, source: &str) {
        let shader = self.expect_handle(shader);
        self.with_state_mut(|state| {
            let shader = self.expect_live_shader(state, shader);
            state.gl.shader_source(shader, source);
        });
    }

    pub fn compile_shader(&self, shader: Shader) {
        let shader = self.expect_handle(shader);
        self.with_state_mut(|state| {
            let shader = self.expect_live_shader(state, shader);
            state.gl.compile_shader(shader);
        });
    }

    pub fn delete_shader(&self, shader: Shader) {
        let raw = self.expect_handle(shader);
        self.with_state_mut(|state| {
            let shader = state
                .shaders
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Shader>(raw.get()));
            state.gl.delete_shader(Some(&shader));
        });
    }

    pub fn create_texture(&self) -> Option<Texture> {
        self.with_state_mut(|state| {
            let texture = state.gl.create_texture()?;
            let raw = state.alloc_handle::<RawTexture>();
            state.textures.insert(raw, texture);
            Some(Texture::new(self.context_id, raw))
        })
    }

    pub fn bind_texture(&self, target: u32, texture: Option<Texture>) {
        let texture = texture.map(|texture| self.expect_handle(texture));
        self.with_state_mut(|state| {
            let texture = texture.map(|raw| self.expect_live_texture(state, raw));
            state.gl.bind_texture(target, texture);
        });
    }

    pub fn delete_texture(&self, texture: Texture) {
        let raw = self.expect_handle(texture);
        self.with_state_mut(|state| {
            let texture = state
                .textures
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<Texture>(raw.get()));
            state.gl.delete_texture(Some(&texture));
        });
    }

    pub fn copy_tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    ) {
        self.with_state(|state| {
            state
                .gl
                .copy_tex_image_2d(target, level, internal_format, x, y, width, height, border);
        });
    }

    pub fn tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue> {
        self.with_state(|state| {
            state.gl.tex_image_3d_with_opt_u8_array(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                format,
                ty,
                pixels,
            )
        })
    }

    pub fn tex_sub_image_3d(
        &self,
        target: u32,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue> {
        self.with_state(|state| {
            state.gl.tex_sub_image_3d_with_opt_u8_array(
                target, level, x_offset, y_offset, z_offset, width, height, depth, format, ty,
                pixels,
            )
        })
    }

    pub fn tex_storage_3d(
        &self,
        target: u32,
        levels: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        depth: i32,
    ) {
        self.with_state(|state| {
            state
                .gl
                .tex_storage_3d(target, levels, internal_format, width, height, depth);
        });
    }

    pub fn create_transform_feedback(&self) -> Option<TransformFeedback> {
        self.with_state_mut(|state| {
            let transform_feedback = state.gl.create_transform_feedback()?;
            let raw = state.alloc_handle::<RawTransformFeedback>();
            state.transform_feedbacks.insert(raw, transform_feedback);
            Some(TransformFeedback::new(self.context_id, raw))
        })
    }

    pub fn bind_transform_feedback(
        &self,
        target: u32,
        transform_feedback: Option<TransformFeedback>,
    ) {
        let transform_feedback =
            transform_feedback.map(|transform_feedback| self.expect_handle(transform_feedback));
        self.with_state_mut(|state| {
            let transform_feedback =
                transform_feedback.map(|raw| self.expect_live_transform_feedback(state, raw));
            state.gl.bind_transform_feedback(target, transform_feedback);
        });
    }

    pub fn is_transform_feedback(&self, transform_feedback: TransformFeedback) -> bool {
        let raw = self.expect_handle(transform_feedback);
        self.with_state(|state| {
            let transform_feedback = self.expect_live_transform_feedback(state, raw);
            state.gl.is_transform_feedback(Some(transform_feedback))
        })
    }

    pub fn begin_transform_feedback(&self, primitive_mode: u32) {
        self.with_state(|state| state.gl.begin_transform_feedback(primitive_mode));
    }

    pub fn end_transform_feedback(&self) {
        self.with_state(|state| state.gl.end_transform_feedback());
    }

    pub fn pause_transform_feedback(&self) {
        self.with_state(|state| state.gl.pause_transform_feedback());
    }

    pub fn resume_transform_feedback(&self) {
        self.with_state(|state| state.gl.resume_transform_feedback());
    }

    pub fn delete_transform_feedback(&self, transform_feedback: TransformFeedback) {
        let raw = self.expect_handle(transform_feedback);
        self.with_state_mut(|state| {
            let transform_feedback = state
                .transform_feedbacks
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<TransformFeedback>(raw.get()));
            state
                .gl
                .delete_transform_feedback(Some(&transform_feedback));
        });
    }

    pub fn create_vertex_array(&self) -> Option<VertexArray> {
        self.with_state_mut(|state| {
            let vertex_array = state.gl.create_vertex_array()?;
            let raw = state.alloc_handle::<RawVertexArray>();
            state.vertex_arrays.insert(raw, vertex_array);
            Some(VertexArray::new(self.context_id, raw))
        })
    }

    pub fn bind_vertex_array(&self, vertex_array: Option<VertexArray>) {
        let vertex_array = vertex_array.map(|vertex_array| self.expect_handle(vertex_array));
        self.with_state_mut(|state| {
            let vertex_array = vertex_array.map(|raw| self.expect_live_vertex_array(state, raw));
            state.gl.bind_vertex_array(vertex_array);
        });
    }

    pub fn delete_vertex_array(&self, vertex_array: VertexArray) {
        let raw = self.expect_handle(vertex_array);
        self.with_state_mut(|state| {
            let vertex_array = state
                .vertex_arrays
                .remove(&raw)
                .unwrap_or_else(|| self.missing_live_handle::<VertexArray>(raw.get()));
            state.gl.delete_vertex_array(Some(&vertex_array));
        });
    }

    pub fn clear_depth_f(&self, depth: f32) {
        self.with_state(|state| state.gl.clear_depth(depth));
    }

    pub(crate) fn with_state<R>(&self, f: impl FnOnce(&ContextState) -> R) -> R {
        with_context(self.context_id, f).unwrap_or_else(|| self.missing_context())
    }

    pub(crate) fn with_state_mut<R>(&self, f: impl FnOnce(&mut ContextState) -> R) -> R {
        with_context_mut(self.context_id, f).unwrap_or_else(|| self.missing_context())
    }

    fn expect_handle<H: ApiHandle>(&self, handle: H) -> H::Raw {
        if handle.context_id() != self.context_id {
            panic!(
                "{} handle {} belongs to context {}, not context {}",
                H::KIND,
                handle.raw().get(),
                handle.context_id().get(),
                self.context_id.get()
            );
        }
        handle.raw()
    }

    fn expect_live_buffer<'a>(&self, state: &'a ContextState, raw: RawBuffer) -> &'a WebGlBuffer {
        state
            .buffers
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Buffer>(raw.get()))
    }

    fn expect_live_framebuffer<'a>(
        &self,
        state: &'a ContextState,
        raw: RawFramebuffer,
    ) -> &'a WebGlFramebuffer {
        state
            .framebuffers
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Framebuffer>(raw.get()))
    }

    fn expect_live_program<'a>(
        &self,
        state: &'a ContextState,
        raw: RawProgram,
    ) -> &'a WebGlProgram {
        state
            .programs
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Program>(raw.get()))
    }

    fn expect_live_renderbuffer<'a>(
        &self,
        state: &'a ContextState,
        raw: RawRenderbuffer,
    ) -> &'a WebGlRenderbuffer {
        state
            .renderbuffers
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Renderbuffer>(raw.get()))
    }

    fn expect_live_sampler<'a>(
        &self,
        state: &'a ContextState,
        raw: RawSampler,
    ) -> &'a WebGlSampler {
        state
            .samplers
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Sampler>(raw.get()))
    }

    fn expect_live_shader<'a>(&self, state: &'a ContextState, raw: RawShader) -> &'a WebGlShader {
        state
            .shaders
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Shader>(raw.get()))
    }

    fn expect_live_texture<'a>(
        &self,
        state: &'a ContextState,
        raw: RawTexture,
    ) -> &'a WebGlTexture {
        state
            .textures
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<Texture>(raw.get()))
    }

    fn expect_live_transform_feedback<'a>(
        &self,
        state: &'a ContextState,
        raw: RawTransformFeedback,
    ) -> &'a WebGlTransformFeedback {
        state
            .transform_feedbacks
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<TransformFeedback>(raw.get()))
    }

    fn expect_live_vertex_array<'a>(
        &self,
        state: &'a ContextState,
        raw: RawVertexArray,
    ) -> &'a WebGlVertexArrayObject {
        state
            .vertex_arrays
            .get(&raw)
            .unwrap_or_else(|| self.missing_live_handle::<VertexArray>(raw.get()))
    }

    fn missing_context(&self) -> ! {
        panic!("glemu context {} is not registered", self.context_id.get());
    }

    fn missing_live_handle<H: ApiHandle>(&self, raw_id: u32) -> ! {
        panic!(
            "{} handle {} is not live in context {}",
            H::KIND,
            raw_id,
            self.context_id.get()
        );
    }
}
