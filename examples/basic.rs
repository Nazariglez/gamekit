use gamekit::platform::PlatformConfig;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(PlatformConfig::default())?
        .build()
}
