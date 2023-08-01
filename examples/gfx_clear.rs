use gamekit::app::event;
use gamekit::gfx::{Color, Gfx, GfxConfig, Renderer};
use gamekit::platform::PlatformConfig;
use gamekit::time::{Time, TimeConfig};

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig::default())?
        .add_config(TimeConfig::default())?
        .on(on_draw)
        .build()
}

fn on_draw(evt: &event::Draw, gfx: &mut Gfx, time: &mut Time) {
    let color = Color::rgb(time.elapsed_f32().cos(), time.elapsed_f32().sin(), 1.0);
    let mut renderer = Renderer::new();
    renderer.begin(color, 0, 0);
    gfx.render(evt.window_id, &renderer).unwrap();
}
