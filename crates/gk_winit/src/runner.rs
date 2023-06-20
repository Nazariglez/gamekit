use crate::Manager;
use gk_app::{App, GKState};
use gk_core::GKWindowId;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};

pub fn runner<S: GKState + 'static>(mut app: App<S>) -> Result<(), String> {
    if app.get_mut_plugin::<Manager>().is_none() {
        return Err("Cannot find Winit's Window Manager".to_string());
    }

    let event_loop = EventLoop::new();
    event_loop.run(move |evt, event_loop, control_flow| {
        {
            let manager = app.get_mut_plugin::<Manager>().unwrap();
            if manager.request_exit {
                control_flow.set_exit();
                return;
            }

            set_event_loop(manager, event_loop);
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
            }
            Event::MainEventsCleared => {
                app.tick();
            }
            _ => (),
        }

        let manager = app.get_mut_plugin::<Manager>().unwrap();
        unset_event_loop(manager);
    });

    Ok(())
}

fn set_event_loop(manager: &mut Manager, event_loop: &EventLoopWindowTarget<()>) {
    manager.event_loop_ptr = Some(event_loop as *const _);
}

fn unset_event_loop(manager: &mut Manager) {
    manager.event_loop_ptr = None;
}

/// Deference the EventLoop Raw pointer.
/// We do this because we need to have a reference to it to create new windows
/// but it's only available inside the application's loop
/// and cannot be stored due it's lifetime passed to the loop¡s callback
/// although the lifetime for EventLoop on winit is ' static
///
/// # Safety
///
/// if `event_loop_ptr` is `Some(ptr)` means that the pointer should be safe to use
/// because is set when the event's loop start and unset when it finish.
pub(crate) unsafe fn event_loop(manager: &Manager) -> Option<&EventLoopWindowTarget<()>> {
    manager.event_loop_ptr.map(|ptr| &*ptr)
}
