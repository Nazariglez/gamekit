use gamekit::app::event;
use gamekit::gfx::{
    Buffer, Color, Gfx, RenderPipeline, Renderer, Texture, VertexFormat, VertexLayout,
};
use gamekit::platform::Platform;
use gamekit::prelude::*;

// language=wgsl
const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texcoord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec3<f32>,
    @location(0) texcoord: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.texcoord = model.texcoord;
//    out.position = vec4<f32>(model.position.x, ap - 0.5, 0.0, 1.0);
    out.position = model.position;
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
    texture: Texture,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx
            .create_render_pipeline(SHADER)
            .with_vertex_layout(
                VertexLayout::new()
                    .with_attr(0, VertexFormat::Float32x3)
                    .with_attr(1, VertexFormat::Float32x2),
            )
            .build()?;

        let texture = gfx.create_texture_2d().build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            //pos               //coords
            0.5,  0.5, 0.0,     1.0, 1.0,
            0.5, -0.5, 0.0,     1.0, 0.0,
            -0.5, -0.5, 0.0,    0.0, 0.0,
            -0.5,  0.5, 0.0,    0.0, 1.0
        ];
        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        #[rustfmt::skip]
            let indices = &[
            0, 1, 3,
            1, 2, 3,
        ];
        let ebo = gfx.create_index_buffer(indices).build()?;

        Ok(State {
            pip,
            vbo,
            ebo,
            texture,
        })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(Platform::config())?
        .add_config(Gfx::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &event::Draw, gfx: &mut Gfx, state: &mut State) {
    let mut renderer = Renderer::new();
    renderer.begin(Color::rgb(0.1, 0.2, 0.3), 0, 0);
    renderer.apply_pipeline(&state.pip);
    let bindings = [&state.vbo, &state.ebo];
    renderer.apply_bindings(&bindings);
    renderer.draw(0..6);
    gfx.render(evt.window_id, &renderer).unwrap();
}
