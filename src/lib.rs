//! WebGL2 shim bindings for `wasm32-unknown-unknown`.
//!
//! `glemu` keeps Emscripten-style integer object IDs mapped to `web_sys`
//! WebGL objects, exposes proc-address lookup helpers for proc-table driven GL
//! consumers, and provides a typed [`Context`] API for direct WebGL2 use.
//!
//! Public API:
//! - when the `api` feature is enabled: [`Context`] plus typed handle wrappers
//!   such as [`Buffer`] and [`Texture`]
//! - [`ContextId`]
//! - when the `raw-proc` feature is enabled: [`get_proc_address`]
//! - [`register_gl_context`], [`set_gl_context`], [`drop_gl_context`], and
//!   [`current_context_id`]
//!
//! Feature flags:
//! - `api`: enables the typed [`Context`] API
//! - `raw-proc`: enables proc-table lookup via [`get_proc_address`]
//! - `image-data`: enables `ImageData` texture upload helpers
//! - `dom-uploads`: enables `ImageBitmap` / canvas / image / video upload helpers
//! - `video-frame`: enables `VideoFrame` upload helpers
//!
//! Disable default features and opt back in selectively to reduce `web-sys`
//! surface area in consumer wasm binaries.
//!
//! The raw implementation modules are internal details.
//!
//! See `examples/wasm-pack-demo` for a minimal browser demo built with
//! `wasm-pack`.

#[cfg(feature = "api")]
mod api;
mod helpers;
#[cfg(feature = "raw-proc")]
mod raw;
mod registry;
mod state;
mod types;

#[cfg(feature = "api")]
pub use api::{
    Buffer, Context, Framebuffer, Program, Query, Renderbuffer, Sampler, Shader, Texture,
    TransformFeedback, VertexArray,
};
#[cfg(feature = "raw-proc")]
pub use raw::get_proc_address;
pub use registry::{current_context_id, drop_gl_context, register_gl_context, set_gl_context};
pub use types::ContextId;
