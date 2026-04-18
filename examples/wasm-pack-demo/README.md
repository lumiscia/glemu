# glemu wasm-pack demo

This example exposes a wasm-bindgen `Demo` struct to JavaScript, accepts a `WebGl2RenderingContext` created in JS, registers it with `glemu::Context`, and renders an animated triangle through a real shader program managed from the Rust side.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
make build
```

## Run

```bash
make serve
```

Open <http://localhost:8001/>.
