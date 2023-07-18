use gamekit::gfx::{Gfx, GfxConfig};
use gamekit::prelude::*;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .add_config(GfxConfig)?
        .on_update(|platform: &mut Platform, gfx: &mut Gfx| {
            platform.window_ids().iter().for_each(|id| {
                gfx.draw(id);
            });
        })
        .build()
}
