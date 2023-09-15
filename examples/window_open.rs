use gamekit::app::App;
use gk_sys::keyboard::KeyboardEvent;
use gk_sys::mouse::MouseEvent;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(App::config())?
        .on(|evt: &KeyboardEvent| println!("{:?}", evt))
        .on(|evt: &MouseEvent| println!("{:?}", evt))
        .build()
}
