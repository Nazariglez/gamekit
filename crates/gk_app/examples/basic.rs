use gk_app::{App, AppBuilder, FromStorage, GKState, Plugin, Plugins, Storage};
use gk_core::GKWindowManager;
use gk_macro::AppState;
use gk_winit::{runner, Manager, Window};

#[derive(AppState)]
struct State<T> {
    id: i32,
    i: T,
}

struct PP {
    id: i32,
}

impl Plugin for PP {}

fn main() {
    let b = 1090;
    let s = State { id: 9999, i: b };
    AppBuilder::init_with(|| Ok(s))
        .set_runner(runner)
        .add_plugin(PP { id: 1234 })
        .add_plugin(Manager::new())
        // .set_event(|state: &mut State<'_>, pp: &mut PP| {
        //     {
        //         ::std::io::_print(
        //             format_args!("from event callback {0} {1}\n", state.id, pp.id),
        //         );
        //     };
        // })
        .run()
        .unwrap();
}
