use gamekit::app::App;
use gamekit::gfx::{Color, Gfx, Renderer};
use gamekit::sys::event::DrawEvent;
use gamekit::time::Time;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &DrawEvent, gfx: &mut Gfx, time: &mut Time) {
    // Calculate different colors per frame
    let color = Color::rgb(time.elapsed_f32().cos(), time.elapsed_f32().sin(), 1.0);

    // create a new frame
    let mut frame = gfx.create_frame(evt.window_id).unwrap();

    // new render with a new pass clearing the screen
    let mut renderer = Renderer::new();
    renderer.begin_pass().clear_color(color);

    // render to the frame
    gfx.render(&mut frame, &renderer).unwrap();

    // present the frame to the screen
    gfx.present(frame).unwrap();
}
