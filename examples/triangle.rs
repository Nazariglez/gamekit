use gamekit::app::event;
use gamekit::gfx::{Color, Gfx, GfxConfig, RenderPipeline, Renderer};
use gamekit::platform::PlatformConfig;
use gamekit::prelude::*;

// language=wgsl
const SHADER: &str = r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}
"#;

#[derive(AppState)]
struct State {
    pip: RenderPipeline,
}

impl State {
    fn new(gfx: &mut Gfx) -> Result<Self, String> {
        let pip = gfx.create_render_pipeline(SHADER).build().unwrap();
        Ok(State { pip })
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
    renderer.begin(Color::RED, 0, 0);
    renderer.apply_pipeline(&state.pip);
    renderer.draw(0..3);
    gfx.render(evt.window_id, &renderer).unwrap();
}
