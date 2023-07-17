use gamekit::gfx::Gfx;
use gamekit::prelude::*;

#[derive(AppState)]
struct State {
    gfx: Gfx,
}

impl State {
    pub fn new() -> Result<Self, String> {
        Gfx::new().map(|gfx| Self { gfx })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(|| State::new())
        .add_config(PlatformConfig::default())?
        .on_init(|windows: &mut Windows, state: &mut State| {
            let id = windows.create().title("2").size(400, 400).build().unwrap();
            let ids = windows.window_ids().to_vec();
            ids.iter().cloned().for_each(|id| {
                state.gfx.create_surface(windows.window(id).unwrap());
            });
        })
        .on_update(|windows: &mut Windows, state: &mut State| {
            windows.window_ids().iter().for_each(|id| {
                state.gfx.draw(id);
            });
        })
        .build()
}
