use gk_app::{App, AppBuilder, FromAppStorage, GKState};
use gk_core::GKWindowManager;
use gk_macro::AppState;
use gk_winit::{runner, Manager, Window};

#[derive(AppState)]
struct State {}

fn main() {
    AppBuilder::init_with(|| Ok(State {}))
        .set_runner(runner)
        .add_plugin(Manager::new())
        .set_event(|state: &mut State| {
            println!("from event callback");
        })
        .run()
        .unwrap();
}
