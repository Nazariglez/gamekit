use crate::window::{CursorIcon, GKWindow, GKWindowId, GKWindowManager};
use hashbrown::HashMap;

#[derive(Default)]
pub struct Manager {
    windows: HashMap<GKWindowId, Window>,
    pub(crate) request_exit: bool,
}

impl GKWindowManager<Window> for Manager {
    fn create(&mut self) -> Result<GKWindowId, String> {
        let count = self.windows.len();
        let id: GKWindowId = (count as u64).into();
        let win = Window {
            id,
            ..Default::default()
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

pub struct Window {
    id: GKWindowId,
    size: (u32, u32),
    position: (i32, i32),
    title: String,
    cursor: CursorIcon,
    resizable: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: 0u64.into(),
            size: (0, 0),
            position: (0, 0),
            title: "Window".to_string(),
            cursor: CursorIcon::Default,
            resizable: false,
        }
    }
}

impl GKWindow for Window {
    fn id(&self) -> GKWindowId {
        self.id
    }

    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn width(&self) -> u32 {
        self.size.0
    }

    fn height(&self) -> u32 {
        self.size.1
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }

    fn scale(&self) -> f64 {
        1.0
    }

    fn position(&self) -> Result<(i32, i32), String> {
        Ok(self.position)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    fn fullscreen(&self) -> bool {
        false
    }

    fn set_fullscreen(&mut self, _fullscreen: bool) {
        // no-op
    }

    fn request_focus(&mut self) {
        // no-op
    }

    fn has_focus(&self) -> bool {
        true
    }

    fn set_cursor_icon(&mut self, cursor: CursorIcon) {
        self.cursor = cursor;
    }

    fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    fn set_maximized(&mut self, _maximized: bool) {
        // no-op
    }

    fn maximized(&self) -> bool {
        false
    }

    fn set_minimized(&mut self, _minimized: bool) {
        // no-op
    }

    fn minimized(&self) -> bool {
        false
    }

    fn set_visible(&mut self, _visible: bool) {
        // no-op
    }

    fn visible(&self) -> bool {
        false
    }

    fn set_transparent(&mut self, _transparent: bool) {
        // no-op
    }

    fn transparent(&self) -> bool {
        false
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    fn resizable(&self) -> bool {
        self.resizable
    }
}
