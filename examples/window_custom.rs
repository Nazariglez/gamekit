use gamekit::platform::Platform;
use gk_app::window::GKWindowAttributes;

fn main() -> Result<(), String> {
    let config = Platform::config().with_window(
        GKWindowAttributes::default()
            .with_title("Custom Title - GameKit")
            .with_size(400, 300)
            .with_max_size(450, 350)
            .with_min_size(350, 250)
            .with_resizable(true),
    );
    gamekit::init().add_config(config)?.build()
}
