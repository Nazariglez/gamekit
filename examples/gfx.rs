use gamekit::gfx::{Gfx, GfxConfig};
use gamekit::prelude::*;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig)?
        .on_init(|platform: &mut Platform| {
            let win = GKWindowAttributes::default()
                .with_title("GameKit Window 2")
                .with_size(400, 200)
                .with_resizable(true);
            let _ = platform.create_window(win).expect("All good");
        })
        .on_update(|platform: &mut Platform, gfx: &mut Gfx| {
            platform.window_ids().iter().for_each(|id| {
                gfx.draw(id);
            });
        })
        .build()
}
