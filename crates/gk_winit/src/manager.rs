use std::collections::HashMap;
use std::ops::Deref;
use winit::event::{Event, WindowEvent};
use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use winit::event_loop::EventLoop;
pub use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window as WWindow;

pub struct Manager {
    windows: HashMap<GKWindowId, Window>,
    event_loop_ptr: Option<*const EventLoopWindowTarget<()>>,
    request_exit: bool,
}

impl GKWindowManager<Window> for Manager {
    fn new() -> Result<Self, String> {
        Ok(Self {
            windows: HashMap::default(),
            event_loop_ptr: None,
            request_exit: false,
        })
    }

    fn create(&mut self) -> Result<GKWindowId, String> {
        // SAFETY: if it's `Some` means that we're inside the event's loop and this is available
        let event_loop = unsafe { event_loop(self) };
            match event_loop {
                Some(event_loop) => {
                    let raw = WWindow::new(event_loop)
                        .map_err(|err| err.to_string())?;

                    let raw_id: u64 = raw.id().into();
                    let id = raw_id.into();
                    let win = Window {
                        id,
                        raw,
                    };

                    self.windows.insert(id, win);
                    Ok(id)
                }
                None => Err("Cannot create window because EventLoop is not initialized".to_string())
            }
    }

    fn window(&mut self, id: GKWindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    fn close(&mut self, id: GKWindowId) -> bool {
        self.windows.remove(&id).is_some()
    }

    fn exit(&mut self) {
        self.request_exit = true;
    }

    fn create_runner<F: FnMut(&mut Manager) + 'static>(mut self, mut f: F) -> Box<dyn FnOnce()> {
        let event_loop = EventLoop::new();
        Box::new(move || {
            event_loop.run(move |evt, event_loop, control_flow| {
                if self.request_exit {
                    control_flow.set_exit();
                    return;
                }

                set_event_loop(&mut self, event_loop);
                control_flow.set_wait();
                println!("{evt:?}");

                match evt {
                    Event::WindowEvent {
                        window_id, event: WindowEvent::CloseRequested,
                    } => {
                        let raw_id: u64 = window_id.into();
                        let id: GKWindowId = raw_id.into();
                        self.windows.remove(&id);
                    },
                    _ => (),
                }

                f(&mut self);
                unset_event_loop(&mut self);
            });
        })
    }
}

pub struct Window {
    id: GKWindowId,
    raw: WWindow,
}

impl GKWindow for Window {
    fn id(&self) -> GKWindowId {
        self.id
    }

    fn width(&self) -> u32 {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }
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
unsafe fn event_loop(manager: &Manager) -> Option<&EventLoopWindowTarget<()>> {
    manager.event_loop_ptr.map(|ptr| &*ptr)
}

