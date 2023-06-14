use std::collections::HashMap;
use std::ops::Deref;
use winit::event::{Event, WindowEvent};
use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use winit::event_loop::EventLoop;
pub use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window as WWindow;

pub struct Manager {
    window_ids: u64,
    windows: HashMap<GKWindowId, Window>,
    event_loop_ptr: Option<*const EventLoopWindowTarget<()>>
}

impl Manager {
    fn set_event_loop(&mut self, event_loop: &EventLoopWindowTarget<()>) {
        self.event_loop_ptr = Some(event_loop as *const _);
    }

    fn unset_event_loop(&mut self) {
        self.event_loop_ptr = None;
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
    unsafe fn event_loop(&self) -> Option<&EventLoopWindowTarget<()>> {
        self.event_loop_ptr.map(|ptr| &*ptr)
    }
}

impl GKWindowManager<Window> for Manager {
    fn new() -> Result<Self, String> {
        Ok(Self {
            window_ids: 0,
            windows: HashMap::default(),
            event_loop_ptr: None,
        })
    }

    fn create(&mut self) -> Result<GKWindowId, String> {
        // SAFETY: if it's `Some` means that we're inside the event's loop and this is available
        let event_loop = unsafe { self.event_loop() };
            match event_loop {
                Some(event_loop) => {
                    let raw = WWindow::new(event_loop)
                        .map_err(|err| err.to_string())?;

                    let id = GKWindowId::new(self.window_ids);
                    let win = Window {
                        id,
                        raw,
                    };

                    self.window_ids += 1;
                    self.windows.insert(id, win);
                    Ok(id)
                }
                None => Err("EventLoop must be initialized".to_string())
            }
    }

    fn window(&mut self, id: GKWindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    fn close(&mut self, id: GKWindowId) -> bool {
        self.windows.remove(&id).is_some()
    }

    fn run<F: FnMut()>(&mut self, f: F) {
        let event_loop = EventLoop::default();
        event_loop.run(|event, event_loop, control_flow| {

        });
    }

    fn create_runner<F: FnMut(&mut Manager) + 'static>(mut self, mut f: F) -> Box<dyn FnOnce()> {
        let event_loop = EventLoop::new();
        Box::new(move || {
            event_loop.run(move |evt, event_loop, control_flow| {
                self.set_event_loop(event_loop);
                control_flow.set_wait();
                println!("{evt:?}");

                match evt {
                    Event::WindowEvent {
                        window_id, event: WindowEvent::CloseRequested,
                    } => control_flow.set_exit(),
                    _ => (),
                }

                f(&mut self);
                self.unset_event_loop();
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