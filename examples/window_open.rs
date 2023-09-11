use gamekit::app::App;

fn main() -> Result<(), String> {
    gamekit::init().add_config(App::config())?.build()
}
