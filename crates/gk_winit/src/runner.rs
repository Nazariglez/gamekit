use crate::Manager;
use gk_app::{App, GKState};
use gk_core::GKWindowId;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};

pub fn runner<S: GKState + 'static>(mut app: App<S>) -> Result<(), String> {
    let manager = app.get_mut_plugin::<Manager>().unwrap();
    // .ok_or_else(|| Err("Cannot find Winit's Window Manager".to_string()))?;

    let event_loop = manager.event_loop.take().unwrap();
    // .ok_or_else(|| Err("Something went wrong adquiring the Winit's EventLoop.".to_string()))?;

    event_loop.run(move |evt, event_loop, control_flow| {
        {
            let manager = app.get_mut_plugin::<Manager>().unwrap();
            if manager.request_exit {
                control_flow.set_exit();
                return;
            }

            manager.event_loop.set(event_loop);
        }

        control_flow.set_wait();
        println!("{evt:?}");

        match evt {
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
                }
            }
            Event::MainEventsCleared => {
                app.tick();
            }
            _ => (),
        }

        let manager = app.get_mut_plugin::<Manager>().unwrap();
        manager.event_loop.unset();
    });

    Ok(())
}
