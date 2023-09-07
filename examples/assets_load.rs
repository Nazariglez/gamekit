use gamekit::app::event;
use gamekit::assets::{AssetLoaded, Assets};
use gamekit::platform::Platform;

fn main() -> Result<(), String> {
    gamekit::init()
        .add_config(Platform::config())?
        .add_config(Assets::config())?
        .on(|evt: &AssetLoaded| {
            println!("Asset loaded {:?}", evt);
        })
        .once(|evt: &event::Init, assets: &mut Assets| {
            assets.load(&asset_path("rust-logo-512x512.png"))
        })
        .on(|evt: &event::Update, assets: &mut Assets| {
            assets.update();
        })
        .build()
}

fn asset_path(path: &str) -> String {
    let base = if cfg!(target_arch = "wasm32") {
        "./assets"
    } else {
        "./examples/assets"
    };

    format!("{base}/{path}")
}
