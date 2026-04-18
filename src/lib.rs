//! WebGL2 shim bindings for `wasm32-unknown-unknown`.
//!
//! `glemu` keeps Emscripten-style integer object IDs mapped to `web_sys`
//! WebGL objects, exposes proc-address lookup helpers for proc-table driven GL
//! consumers, and provides a typed [`Context`] API for direct WebGL2 use.
//!
//! Public API:
//! - [`Context`] plus typed handle wrappers such as [`Buffer`] and [`Texture`]
//! - [`ContextId`]
//! - [`get_proc_address`]
//! - [`register_gl_context`], [`set_gl_context`], [`drop_gl_context`], and
//!   [`current_context_id`]
//!
//! The raw implementation modules are internal details.
//!
//! See `examples/wasm-pack-demo` for a minimal browser demo built with
//! `wasm-pack`.

mod api;
mod helpers;
mod raw;
mod registry;
mod state;
mod types;

pub use api::{
    Buffer, Context, Framebuffer, Program, Query, Renderbuffer, Sampler, Shader, Texture,
    TransformFeedback, VertexArray,
};
pub use raw::get_proc_address;
pub use registry::{current_context_id, drop_gl_context, register_gl_context, set_gl_context};
pub use types::ContextId;
