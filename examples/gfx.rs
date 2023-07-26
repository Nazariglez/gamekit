use gamekit::app::event;
use gamekit::gfx::{Gfx, GfxConfig};
use gamekit::platform::PlatformConfig;

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
