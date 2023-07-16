use gamekit::prelude::*;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .build()
}
