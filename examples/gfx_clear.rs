use gamekit::app::event;
use gamekit::gfx::{Color, Gfx, Renderer};
use gamekit::platform::Platform;
use gamekit::time::Time;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(Platform::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &event::Draw, gfx: &mut Gfx, time: &mut Time) {
    let color = Color::rgb(time.elapsed_f32().cos(), time.elapsed_f32().sin(), 1.0);
    let mut renderer = Renderer::new();
    renderer.begin(color, 0, 0);
    gfx.render(evt.window_id, &renderer).unwrap();

    println!("FPS: {} -> {:.5}", time.fps(), time.delta_f32());
}
