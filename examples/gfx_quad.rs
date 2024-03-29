use gamekit::app::App;
use gamekit::gfx::{
    Buffer, Color, DrawFrame, Gfx, IndexFormat, RenderPipeline, VertexFormat, VertexLayout,
};
use gamekit::prelude::*;

// language=wgsl
const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.position = vec4<f32>(model.position - 0.5, 0.0, 1.0);
    return out;
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
    ebo: Buffer,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x2)
                    .with_attr(1, VertexFormat::Float32x3),
            )
            .with_index_format(IndexFormat::UInt16)
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            0.0, 1.0,   1.0, 0.0, 0.0,
            0.0, 0.0,   1.0, 1.0, 0.0,
            1.0, 0.0,   0.0, 0.0, 1.0,
            1.0, 1.0,   0.0, 1.0, 0.0,
        ];
        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];
        let ebo = gfx.create_index_buffer(indices).build()?;

        Ok(State { pip, vbo, ebo })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &DrawFrame, gfx: &mut Gfx, state: &mut State) {
    let mut renderer = evt.create_renderer();
    renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
    renderer.apply_pipeline(&state.pip);
    renderer.apply_buffers(&[&state.vbo, &state.ebo]);
    renderer.draw(0..6);
    gfx.render(&renderer).unwrap();
}
