use gamekit::gfx::{Color, DrawFrame, Gfx};
use gamekit::platform::Platform;
use gamekit::time::Time;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(Platform::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_frame)
        .build()
}

fn on_frame(frame: &DrawFrame, gfx: &mut Gfx, time: &mut Time) {
    let color = Color::rgb(time.elapsed_f32().cos(), time.elapsed_f32().sin(), 1.0);
    let mut renderer = frame.create_renderer();
    renderer.clear(Some(color), None, None);
    gfx.render(&renderer).unwrap();
}
