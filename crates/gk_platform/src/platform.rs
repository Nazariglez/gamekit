use crate::backend::{Manager, Window};
use crate::window::{GKWindowAttributes, GKWindowId, GKWindowManager};
use gk_app::Plugin;
use hashbrown::hash_map::{Values, ValuesMut};

pub struct Platform {
    pub(crate) manager: Manager,
    main_window: Option<GKWindowId>,
    window_ids: Vec<GKWindowId>,
}

impl Platform {
    pub(crate) fn new() -> Self {
        Self {
            manager: Manager::new(),
            main_window: None,
            window_ids: vec![],
        }
    }

    pub fn create_window(&mut self, attrs: GKWindowAttributes) -> Result<GKWindowId, String> {
        let id = self.manager.create(attrs)?;
        self.window_ids.push(id);
        Ok(id)
    }

    pub fn window(&mut self, id: GKWindowId) -> Option<&mut Window> {
        self.manager.window(id)
    }

    pub fn main_window(&mut self) -> Option<&mut Window> {
        self.main_window.and_then(|id| self.window(id))
    }

    pub fn set_main_window(&mut self, win_id: GKWindowId) {
        self.main_window = Some(win_id);
    }

    pub fn window_ids(&self) -> &[GKWindowId] {
        &self.window_ids
    }

    pub fn windows(&self) -> Values<'_, GKWindowId, Window> {
        self.manager.windows.values()
    }

    pub fn windows_mut(&mut self) -> ValuesMut<'_, GKWindowId, Window> {
        self.manager.windows.values_mut()
    }

    pub fn close(&mut self, id: GKWindowId) {
        let closed = self.manager.close(id);
        if closed {
            let pos = self
                .window_ids
                .iter()
                .position(|stored_id| *stored_id == id);
            if let Some(pos) = pos {
                self.window_ids.remove(pos);
            }
        }
    }

    pub fn exit(&mut self) {
        self.manager.exit();
    }
}

impl Plugin for Platform {}
