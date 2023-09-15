use gamekit::app::App;
use gamekit::gfx::{
    BindGroup, Buffer, Color, Gfx, RenderPipeline, UniformBinding, VertexFormat, VertexLayout,
};
use gamekit::prelude::*;
use gamekit::sys::event;
use gamekit::time::Time;

// Number of triangles to draw
const INSTANCES: usize = 1000;

// language=wgsl
const SHADER: &str = r#"
struct Locals {
    count: f32,
};

@group(0) @binding(0)
var<uniform> locals: Locals;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    // Values to change position and color
    let n = f32(model.instance_index) * 0.1;
    let j = f32(model.vertex_index) * 0.2;
    let pos = model.position - vec2<f32>(sin(n + locals.count), cos(n + locals.count)) * fract(n) * 0.9;

    var output: VertexOutput;
    output.color = vec3<f32>(fract(n - j), 1.0 - fract(n), fract(n + j));
    output.position = vec4<f32>(pos, 0.0, 1.0);

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
    ubo: Buffer,
    bind_group: BindGroup,
    count: f32,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        #[rustfmt::skip]
        let position: &[f32] = &[
            -0.2, -0.2,
            0.2, -0.2,
            0.0, 0.2
        ];

        let vbo = gfx.create_vertex_buffer(position).build()?;

        let count: f32 = 0.0;
        let ubo = gfx
            .create_uniform_buffer(&[count])
            .with_write_flag(true)
            .build()?;

        let bind_group = gfx
            .create_bind_group()
            .with_uniform(UniformBinding::new(0, &ubo).with_vertex_visibility(true))
            .build()?;

        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(VertexLayout::new().with_attr(0, VertexFormat::Float32x2))
            .with_bind_group(&bind_group)
            .build()?;

        Ok(State {
            pip,
            vbo,
            ubo,
            bind_group,
            count,
        })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .on(on_update)
        .build()
}

fn on_update(_: &event::UpdateEvent, time: &mut Time, state: &mut State) {
    state.count += 0.15 * time.delta_f32();
}

fn on_draw(frame: &DrawFrame, gfx: &mut Gfx, state: &mut State) {
    let mut renderer = frame.create_renderer();
    renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
    renderer.apply_pipeline(&state.pip);
    renderer.apply_buffers(&[&state.vbo]);
    renderer.apply_bindings(&[&state.bind_group]);
    renderer.draw_instanced(0..3, INSTANCES as _);
    gfx.render(&renderer).unwrap();

    // update the uniform to animate the triangles
    gfx.write_buffer(&state.ubo)
        .with_data(&[state.count])
        .build()
        .unwrap();
}
