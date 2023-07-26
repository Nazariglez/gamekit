use gamekit::app::event;
use gamekit::gfx::{Gfx, GfxConfig, Pipeline};
use gamekit::platform::PlatformConfig;

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
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

struct State {
    pip: Pipeline,
}

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig::default())?
        .once(|evt: &event::Init| println!("Init!"))
        .once(|evt: &event::Update| println!("Update!"))
        .on(|evt: &event::Draw, gfx: &mut Gfx| {
            gfx.draw(&evt.window_id);
            println!("----> DRAW");
        })
        .build()
}
