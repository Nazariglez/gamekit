use gamekit::app::{
    event,
    window::{WindowEvent, WindowEventId},
};
use gamekit::gfx::{GfxConfig, GfxDevice, Pipeline, Renderer};
use gamekit::platform::{Platform, PlatformConfig};
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

#[derive(AppState, Default)]
struct State {
    pip: Option<Pipeline>,
}

fn main() -> Result<(), String> {
    gamekit::init_with(|| Ok(State::default()))
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig::default())?
        .once(|evt: &event::Update| println!("Update!"))
        .on(
            |evt: &WindowEvent, gfx: &mut GfxDevice, state: &mut State| {
                println!("-> window event {:?}", evt);
                match evt.event {
                    WindowEventId::Init => {
                        let pip = gfx.create_pipeline(SHADER).unwrap();
                        state.pip = Some(pip);
                    }
                    _ => {}
                }
            },
        )
        .on(
            |evt: &event::Draw, platform: &mut Platform, gfx: &mut GfxDevice, state: &mut State| {
                println!("-<<<< draw");
                if let Some(pip) = &state.pip {
                    let renderer = Renderer::new(pip);
                    gfx.render(evt.window_id, &renderer).unwrap();
                }
                platform.exit();
            },
        )
        .build()
}
