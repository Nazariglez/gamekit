use gamekit::app::event;
use gamekit::gfx::{
    Buffer, Color, Gfx, Primitive, RenderPipeline, Renderer, VertexFormat, VertexLayout,
};
use gamekit::platform::Platform;
use gamekit::prelude::*;

// language=wgsl
const SHADER: &str = r#"
@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position - 0.5, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 1.0, 1.0);
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
            .with_primitive(Primitive::Lines)
            .with_vertex_layout(VertexLayout::new().with_attr(0, VertexFormat::Float32x2))
            .build()?;

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            0.0, 1.0, // top-left
            0.0, 0.0, // bottom-left
            1.0, 0.0, // bottom-right
            1.0, 1.0, // top-right
        ];
        let vbo = gfx.create_vertex_buffer(vertices).build()?;

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 1, 2, 2, 0, // first triangle lines
            0, 2, 2, 3, 3, 0, // second triangle lines
        ];
        let ebo = gfx.create_index_buffer(indices).build()?;

        Ok(State { pip, vbo, ebo })
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
    renderer.begin(Color::BLACK, 0, 0);
    renderer.apply_pipeline(&state.pip);
    let bindings = [&state.vbo, &state.ebo];
    renderer.apply_bindings(&bindings);
    renderer.draw(0..12);
    gfx.render(evt.window_id, &renderer).unwrap();
}
