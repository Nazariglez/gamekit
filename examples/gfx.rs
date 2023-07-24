use gamekit::gfx::{Gfx, GfxConfig};
use gamekit::prelude::*;
use gk_gfx::Canvas;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig::default())?
        .listen_event(|evt: &Canvas, gfx: &mut Gfx| gfx.draw(&evt.window))
        .build()
}
