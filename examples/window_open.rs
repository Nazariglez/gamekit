use gamekit::platform::Platform;

fn main() -> Result<(), String> {
    gamekit::init().add_config(Platform::config())?.build()
}
