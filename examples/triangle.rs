use gamekit::app::event;
use gamekit::gfx::{
    Buffer, Color, Gfx, GfxConfig, RenderPipeline, Renderer, VertexFormat, VertexLayout,
};
use gamekit::platform::PlatformConfig;
use gamekit::prelude::*;

// language=wgsl
const SHADER: &str = r#"
// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
    vbo: Buffer,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x3)
                    .with_attr(1, VertexFormat::Float32x3),
            )
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            0.0, 0.5, 0.0,      1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 1.0, 0.0,
            0.5, -0.5, 0.0,     0.0, 0.0, 1.0,
        ];

        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        Ok(State { pip, vbo })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig::default())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &event::Draw, gfx: &mut Gfx, state: &mut State) {
    let mut renderer = Renderer::new();
    renderer.begin(Color::WHITE, 0, 0);
    renderer.apply_pipeline(&state.pip);
    let bindings = [&state.vbo];
    renderer.apply_bindings(&bindings);
    renderer.draw(0..3);
    gfx.render(evt.window_id, &renderer).unwrap();
}
