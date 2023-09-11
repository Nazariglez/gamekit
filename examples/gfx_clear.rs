use gamekit::app::App;
use gamekit::gfx::{Color, DrawFrame, Gfx};
use gamekit::time::Time;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .build()
}

fn on_draw(frame: &DrawFrame, gfx: &mut Gfx, time: &mut Time) {
    let color = Color::rgb(time.elapsed_f32().cos(), time.elapsed_f32().sin(), 1.0);
    let mut renderer = frame.create_renderer();
    renderer.clear(Some(color), None, None);
    gfx.render(&renderer).unwrap();
}
