use gk_app::event::AppEvent;
use gk_app::prelude::*;
use gk_platform::{GKWindow, PlatformConfig, Windows};

#[derive(AppState)]
struct State {
    id: i32,
    i: i32,
}

struct PP {
    id: i32,
}

impl Plugin for PP {}

fn main() {
    let win_config = PlatformConfig::default().window(Default::default());

    AppBuilder::init_with(|pp: &mut PP, windows: &mut Windows| Ok(State { id: 9999, i: pp.id }))
        .add_config(win_config)
        .unwrap()
        .add_plugin(PP { id: 1234 })
        .on_init(|windows: &mut Windows| {
            let id = windows
                .create()
                .title("Lol")
                .size(100, 100)
                .build()
                .unwrap();
        })
        .on_update(|state: &mut State, pp: &mut PP, windows: &mut Windows| {
            println!(
                "state.id: {}x{}, pp.id: {} -> {:?}",
                state.id,
                state.i,
                pp.id,
                windows.main_window().map(|w| w.id()).unwrap_or(999.into()),
            );
        })
        .listen_event(|evt: &AppEvent, ee: &mut EventQueue<State>| {
            println!("-> {evt:?}");
            if let AppEvent::PostUpdate = evt {
                println!("here... {:?}", evt);
                // panic!();
            }
        })
        .build()
        .unwrap();
}
