use gamekit::app::{
    event,
    window::{WindowEvent, WindowEventId},
};
use gamekit::platform::PlatformConfig;
use gamekit::prelude::*;
use gk_backend::Platform;
use gk_gfx::{Color, GKDevice, Gfx, GfxConfig, RenderPipeline, RenderPipelineDescriptor, Renderer};

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

fn main() -> Result<(), String> {
    gamekit::init_with(|gfx: &mut Gfx| {
        let pip = gfx.create_render_pipeline(SHADER).build().unwrap();
        Ok(State { pip })
    })
    .add_config(PlatformConfig::default())?
    .add_config(GfxConfig::default())?
    // .on(on_window_event)
    .on(on_draw)
    .build()
}

// fn on_window_event(evt: &WindowEvent, gfx: &mut Gfx, state: &mut State) {
//     // Initialize the pipeline when the first window is created
//     if state.pip.is_some() {
//         return;
//     }
//
//     match evt.event {
//         WindowEventId::Init => {
//             let pip = gfx.create_render_pipeline(SHADER).build().unwrap();
//             // state.pip = Some(pip);
//         }
//         _ => {}
//     }
// }

fn on_draw(evt: &event::Draw, platform: &mut Platform, gfx: &mut Gfx, state: &mut State) {
    // if let Some(pip) = &state.pip {
    let mut renderer = Renderer::new();
    renderer.begin(Color::RED, 0, 0);
    renderer.apply_pipeline(&state.pip);
    renderer.draw(0..3);
    gfx.render(evt.window_id, &renderer).unwrap();
    // }
}