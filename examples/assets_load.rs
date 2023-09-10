use gamekit::app::event;
use gamekit::assets::{AssetLoad, AssetLoader};
use gamekit::platform::Platform;
use gamekit::prelude::*;
use log::warn;

#[derive(AppState, Default)]
struct State {
    loaded: usize,
}

fn main() -> Result<(), String> {
    gamekit::init_with(|| Ok(State::default()))
        .add_config(AssetLoader::config())?
        .add_config(Platform::config())?
        .once(on_init)
        .on(on_asset_load)
        .build()
}

fn on_init(_: &event::Init, loader: &mut AssetLoader) {
    loader
        .load(&asset_path("cube.png"))
        .load(&asset_path("bunny.png"));
}

fn on_asset_load(evt: &AssetLoad, app: &mut Platform, state: &mut State) {
    let id = evt.id();
    let loaded = match evt.data() {
        Ok(buff) => format!("Loaded -> '{}' ({} bytes)", id, buff.len()),
        Err(err) => err,
    };
    log::info!("Asset load event {}: {}", evt.id(), loaded);
}

fn asset_path(path: &str) -> String {
    let base = if cfg!(target_arch = "wasm32") {
        "./assets"
    } else {
        "./examples/assets"
    };

    format!("{base}/{path}")
}
