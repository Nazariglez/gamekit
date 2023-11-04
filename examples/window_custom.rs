use gamekit::app::App;
use gk_sys::window::WindowAttributes;

fn main() -> Result<(), String> {
    let config = App::config().with_window(
        WindowAttributes::default()
            .with_title("Custom Title - GameKit")
            .with_size(400, 300)
            .with_max_size(450, 350)
            .with_min_size(350, 250)
            .with_resizable(true),
    );
    gamekit::init().add_config(config)?.build()
}
