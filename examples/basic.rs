use gamekit::prelude::*;

fn main() -> Result<(), String> {
    let platform = PlatformConfig::with_window(Default::default());
    gamekit::init().add_config(platform)?.build()
}
