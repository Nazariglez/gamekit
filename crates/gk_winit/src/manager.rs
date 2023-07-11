use crate::event_loop::EventLoopPtr;
use crate::Window;
use gk_app::window::{GKWindow, GKWindowId, GKWindowManager};
use gk_app::Plugin;
use std::collections::HashMap;
pub use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window as WWindow;

pub struct Manager {
    pub windows: HashMap<GKWindowId, Window>,
    pub(crate) event_loop: EventLoopPtr,
    pub(crate) request_exit: bool,
}

impl Plugin for Manager {}

impl Manager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::default(),
            event_loop: EventLoopPtr::new(),
            request_exit: false,
        }
    }
}

impl GKWindowManager<Window> for Manager {
    fn create(&mut self) -> Result<GKWindowId, String> {
        // SAFETY: if it's `Some` means that we're inside the event's loop and this is available
        let event_loop = self.event_loop.inner();
        match event_loop {
            Some(event_loop) => {
                let win = Window::new(event_loop)?;
                let id = win.id();
                self.windows.insert(id, win);
                Ok(id)
            }
            None => Err("Cannot create window because EventLoop is not initialized".to_string()),
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
}
