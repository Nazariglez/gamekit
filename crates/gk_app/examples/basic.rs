use gk_app::event::AppEvent;
use gk_app::prelude::*;
use gk_core::events::{SuperEvent, SuperEvent2};
use gk_core::window::{GKWindowId, GKWindowManager};
use gk_winit::{runner, Manager, Window, WinitConfig};

#[derive(AppState)]
struct State {
    id: i32,
    i: i32,
    // win_id: GKWindowId,
}

struct PP {
    id: i32,
}

impl Plugin for PP {}

fn main() {
    AppBuilder::init_with(|pp: &mut PP, manager: &mut Manager| {
        // let win_id = manager.create()?;
        Ok(State {
            id: 9999,
            i: pp.id,
            // win_id,
        })
    })
    .add_config(WinitConfig)
    .unwrap()
    .add_plugin(PP { id: 1234 })
    .on_update(|state: &mut State, pp: &mut PP| {
        println!("state.id: {}x{}, pp.id: {}", state.id, state.i, pp.id);
    })
    .listen_event(|evt: &AppEvent, ee: &mut EventQueue<State>| {
        println!("-> {evt:?}");
        if let AppEvent::PostUpdate = evt {
            println!("here... {:?}", evt);
            panic!();
        } else {
            ee.queue(SuperEvent);
        }
    })
    .listen_event(|evt: &SuperEvent, ee: &mut EventQueue<State>| {
        println!("SuperEvent");
        ee.queue(SuperEvent2);
    })
    .listen_event(|evt: &SuperEvent2, ee: &mut EventQueue<State>| {
        println!("SuperEvent2");
    })
    .build()
    .unwrap();
}
