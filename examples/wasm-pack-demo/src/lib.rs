use glemu::{Buffer, Context, Program, VertexArray};
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as Gl;

const VERTEX_STRIDE_FLOATS: i32 = 5;
const VERTEX_STRIDE_BYTES: i32 = VERTEX_STRIDE_FLOATS * 4;
const VERTEX_SHADER_SOURCE: &str = r#"#version 300 es
in vec2 a_position;
in vec3 a_color;

out vec3 v_color;

void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
    v_color = a_color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"#version 300 es
precision mediump float;

in vec3 v_color;
out vec4 out_color;

void main() {
    out_color = vec4(v_color, 1.0);
}
"#;

#[wasm_bindgen]
pub struct Demo {
    context: Context,
    program: Program,
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
}

#[wasm_bindgen]
impl Demo {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: Gl) -> Result<Self, JsValue> {
        let context = Context::from_webgl2_context(gl);
        context.make_current();

        let vertex_shader = compile_shader(&context, Gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?;
        let fragment_shader =
            compile_shader(&context, Gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?;
        let program = context
            .create_program()
            .ok_or_else(|| JsValue::from_str("failed to create shader program"))?;
        context.attach_shader(program, vertex_shader);
        context.attach_shader(program, fragment_shader);
        context.link_program(program);
        context.detach_shader(program, vertex_shader);
        context.detach_shader(program, fragment_shader);
        context.delete_shader(vertex_shader);
        context.delete_shader(fragment_shader);
        context.use_program(Some(program));

        let vertex_array = context
            .create_vertex_array()
            .ok_or_else(|| JsValue::from_str("failed to create vertex array"))?;
        context.bind_vertex_array(Some(vertex_array));

        let vertex_buffer = context
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("failed to create vertex buffer"))?;
        context.bind_buffer(Gl::ARRAY_BUFFER, Some(vertex_buffer));

        let gl = context.webgl2_context();
        let initial_vertices = triangle_vertices(0.0);
        let vertex_data = Float32Array::from(initial_vertices.as_slice());
        gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &vertex_data, Gl::DYNAMIC_DRAW);

        let position_location = attrib_location(&context, program, "a_position")?;
        let color_location = attrib_location(&context, program, "a_color")?;
        gl.enable_vertex_attrib_array(position_location);
        gl.vertex_attrib_pointer_with_i32(
            position_location,
            2,
            Gl::FLOAT,
            false,
            VERTEX_STRIDE_BYTES,
            0,
        );
        gl.enable_vertex_attrib_array(color_location);
        gl.vertex_attrib_pointer_with_i32(
            color_location,
            3,
            Gl::FLOAT,
            false,
            VERTEX_STRIDE_BYTES,
            2 * 4,
        );

        Ok(Self {
            context,
            program,
            vertex_array,
            vertex_buffer,
        })
    }

    pub fn render_frame(&self, time_ms: f64) -> Result<(), JsValue> {
        self.context.make_current();
        self.context.use_program(Some(self.program));
        self.context.bind_vertex_array(Some(self.vertex_array));
        self.context
            .bind_buffer(Gl::ARRAY_BUFFER, Some(self.vertex_buffer));

        let gl = self.context.webgl2_context();
        let vertices = triangle_vertices(time_ms as f32 * 0.001);
        let vertex_data = Float32Array::from(vertices.as_slice());
        gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &vertex_data, Gl::DYNAMIC_DRAW);

        let width = gl.drawing_buffer_width();
        let height = gl.drawing_buffer_height();
        gl.viewport(0, 0, width, height);

        let phase = (time_ms / 1000.0) as f32;
        let red = 0.35 + 0.35 * phase.sin();
        let blue = 0.45 + 0.35 * phase.cos();

        gl.clear_color(red, 0.2, blue, 1.0);
        gl.clear(Gl::COLOR_BUFFER_BIT);
        gl.draw_arrays(Gl::TRIANGLES, 0, 3);
        Ok(())
    }
}

fn compile_shader(
    context: &Context,
    shader_type: u32,
    source: &str,
) -> Result<glemu::Shader, JsValue> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("failed to create shader"))?;
    context.shader_source(shader, source);
    context.compile_shader(shader);
    Ok(shader)
}

fn attrib_location(context: &Context, program: Program, name: &str) -> Result<u32, JsValue> {
    let location = context.get_attrib_location(program, name);
    if location < 0 {
        return Err(JsValue::from_str("failed to resolve attribute location"));
    }
    Ok(location as u32)
}

fn triangle_vertices(time_seconds: f32) -> [f32; 15] {
    let wobble = 0.15 * time_seconds.sin();
    let lift = 0.08 * (time_seconds * 1.7).cos();

    [
        -0.65,
        -0.55 + lift,
        1.0,
        0.35,
        0.2,
        0.65,
        -0.45 - lift,
        0.2,
        0.75,
        1.0,
        wobble,
        0.72,
        1.0,
        0.9,
        0.25,
    ]
}
