use crate::Manager;
use gk_app::{App, GKState};
use gk_core::events::Event as GkEvent;
use gk_core::window::GKWindowId;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};

pub fn runner<S: GKState + 'static>(mut app: App<S>) -> Result<(), String> {
    let manager = app
        .get_mut_plugin::<Manager>()
        .ok_or_else(|| "Cannot find Winit's Window Manager".to_string())?;

    let event_loop = manager
        .event_loop
        .take()
        .ok_or_else(|| "Something went wrong acquiring the Winit's EventLoop.".to_string())?;

    app.initialize();

    event_loop.run(move |evt, event_loop, control_flow| {
        app.get_mut_plugin::<Manager>()
            .unwrap()
            .event_loop
            .set(event_loop);

        control_flow.set_wait();
        println!("{evt:?}");

        match evt {
            Event::LoopDestroyed => {
                app.event(GkEvent::Close);
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } => {
                let manager = app.get_mut_plugin::<Manager>().unwrap();
                let raw_id: u64 = window_id.into();
                let id: GKWindowId = raw_id.into();
                manager.windows.remove(&id);

                if manager.windows.is_empty() {
                    manager.request_exit = true;
                    println!("Close");
                }
            }
            Event::MainEventsCleared => {
                app.tick();
            }
            _ => (),
        }

        let manager = app.get_mut_plugin::<Manager>().unwrap();
        manager.event_loop.unset();
        if manager.request_exit {
            control_flow.set_exit();
        }
    });

    Ok(())
}
