use gamekit::app::event;
use gamekit::gfx::{Canvas, Gfx, GfxConfig};
use gamekit::prelude::*;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig::default())?
        .once(|evt: &event::Init| println!("Init!"))
        .once(|evt: &event::Update| println!("Update!"))
        .on(|evt: &Canvas, gfx: &mut Gfx| {
            gfx.draw(&evt.window);
            println!("----> DRAW");
        })
        .build()
}
