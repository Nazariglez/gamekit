use gk_app::{App, AppBuilder, FromStorage, GKState, Plugin, Plugins, Storage};
use gk_core::GKWindowManager;
use gk_macro::AppState;
use gk_winit::{runner, Manager, Window};

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
    AppBuilder::init_with(|pp: &mut PP| Ok(State { id: 9999, i: pp.id }))
        .set_runner(runner)
        .add_plugin(PP { id: 1234 })
        .add_plugin(Manager::new())
        .set_event(|state: &mut State, pp: &mut PP| {
            println!("state.id: {}x{}, pp.id: {}", state.id, state.i, pp.id);
        })
        .run()
        .unwrap();
}
