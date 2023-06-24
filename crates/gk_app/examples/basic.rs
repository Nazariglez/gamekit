use gk_app::{App, AppBuilder, FromStorage, GKState, Plugin, Plugins, Storage};
use gk_core::{GKWindowId, GKWindowManager};
use gk_macro::AppState;
use gk_winit::{runner, Manager, Window};

#[derive(AppState)]
struct State {
    id: i32,
    i: i32,
    win_id: GKWindowId,
}

struct PP {
    id: i32,
}

impl Plugin for PP {}

fn main() {
    AppBuilder::init_with(|pp: &mut PP, manager: &mut Manager| {
        let win_id = manager.create()?;
        Ok(State {
            id: 9999,
            i: pp.id,
            win_id,
        })
    })
    .set_runner(runner)
    .add_plugin(PP { id: 1234 })
    .add_plugin(Manager::new())
    .set_event(|state: &mut State, pp: &mut PP| {
        println!("state.id: {}x{}, pp.id: {}", state.id, state.i, pp.id);
    })
    .build()
    .unwrap();
}
