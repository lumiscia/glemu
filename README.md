# glemu

`glemu` is a WebGL2-focused shim crate for `wasm32-unknown-unknown`. It keeps OpenGL-style integer IDs mapped to `web_sys` WebGL objects, exposes proc-address lookup for proc-table based GL consumers, and provides a typed `Context` API for direct WebGL2 usage.

The crate is influenced by both Emscripten's `libglemu.js` approach to GL shims and glow's `web_sys` backend structure.

It was mainly created for [rust-skia](https://github.com/rust-skia/rust-skia), as there was no fitting crate that provided the same functionality.

## Public API

`glemu` intentionally exposes a small public surface:

- `Context` for typed WebGL2 access
- typed handle wrappers: `Buffer`, `Framebuffer`, `Program`, `Query`, `Renderbuffer`, `Sampler`, `Shader`, `Texture`, `TransformFeedback`, and `VertexArray`
- `ContextId`
- `get_proc_address` for proc-table assembly
- context registry helpers for proc-table consumers:
  - `register_gl_context`
  - `set_gl_context`
  - `drop_gl_context`
  - `current_context_id`

Everything under the raw implementation modules is internal and may change without notice.

## Proc-address API

Use `get_proc_address("glFunctionName")` when you need a raw function pointer by symbol name. If a consumer wants a callback-style proc resolver, it can trivially wrap `get_proc_address` itself.

The proc table surface keeps the expected GL symbol names stable, including supported aliases such as `glBindVertexArrayOES`.

## Context registration helpers

For proc-table consumers, register a `web_sys::WebGl2RenderingContext` with `register_gl_context`. The returned `ContextId` becomes current immediately. Use `set_gl_context` to switch the current context, `current_context_id` to inspect it, and `drop_gl_context` to unregister it when the context is no longer needed.

## Typed `Context` API

`Context` is a small typed wrapper over the registered WebGL2 state:

- `Context::register` / `Context::from_webgl2_context` create and register a context
- `Context::current` and `Context::from_context_id` recover typed access later
- `Context::make_current` switches the active registered context
- resource creation methods return typed handles bound to the originating context
- texture upload helpers cover byte uploads and DOM/media sources such as `ImageData`, `HtmlImageElement`, `HtmlCanvasElement`, `HtmlVideoElement`, `ImageBitmap`, and optionally `VideoFrame`

A minimal browser demo lives in `examples/wasm-pack-demo`.

```bash
cd examples/wasm-pack-demo
wasm-pack build --target web --dev --out-dir web/pkg
python3 -m http.server --directory web 8001
```

Then open <http://localhost:8001/>.
