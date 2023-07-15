use super::window::Window;
use crate::window::{CursorIcon, GKWindow, GKWindowId, GKWindowManager};
use crate::GKWindowAttributes;
use hashbrown::HashMap;

#[derive(Default)]
pub struct Manager {
    windows: HashMap<GKWindowId, Window>,
    pub(crate) request_exit: bool,
}

impl GKWindowManager<Window> for Manager {
    fn new() -> Self {
        Default::default()
    }

    fn create(&mut self, attrs: GKWindowAttributes) -> Result<GKWindowId, String> {
        let count = self.windows.len();
        let id: GKWindowId = (count as u64).into();
        let win = Window {
            id,
            size: attrs.size.unwrap_or((800, 600)),
            position: attrs.position.unwrap_or((0, 0)),
            title: attrs.title.clone(),
            cursor: CursorIcon::Default,
            resizable: attrs.resizable,
            min_size: None,
            max_size: None,
        };
        self.windows.insert(id, win);
        Ok(id)
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
