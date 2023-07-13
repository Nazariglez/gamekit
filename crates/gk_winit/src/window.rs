use crate::utils::win_id;
use gk_app::window::{CursorIcon, GKWindow, GKWindowId};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Fullscreen, Window as RawWindow, WindowBuilder};

pub struct Window {
    id: GKWindowId,
    raw: RawWindow,
    title: String,
}

impl Window {
    pub(crate) fn new(event_loop: &EventLoopWindowTarget<()>) -> Result<Self, String> {
        let raw = RawWindow::new(event_loop).map_err(|err| err.to_string())?;
        let id = win_id(raw.id());
        let title = format!("GameKit Window {}", <GKWindowId as Into<u64>>::into(id));
        raw.set_title(&title);
        let win = Window { id, raw, title };
        Ok(win)
    }
}

impl GKWindow for Window {
    fn id(&self) -> GKWindowId {
        self.id
    }

    fn size(&self) -> (u32, u32) {
        let scale_factor = self.raw.scale_factor();
        let size = self.raw.inner_size().to_logical::<u32>(scale_factor);
        (size.width, size.height)
    }

    fn width(&self) -> u32 {
        let (w, _) = self.size();
        w
    }

    fn height(&self) -> u32 {
        let (_, h) = self.size();
        h
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.raw.set_inner_size(LogicalSize::new(width, height));
    }

    fn scale(&self) -> f64 {
        self.raw.scale_factor()
    }

    fn position(&self) -> Result<(i32, i32), String> {
        let pos = self
            .raw
            .outer_position()
            .map_err(|err| err.to_string())?
            .to_logical::<i32>(self.scale());

        Ok(pos.into())
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.raw.set_outer_position(LogicalPosition::new(x, y));
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.raw.set_title(&self.title);
    }

    fn fullscreen(&self) -> bool {
        self.raw.fullscreen().is_some()
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        let mode = fullscreen.then(|| Fullscreen::Borderless(self.raw.current_monitor()));
        self.raw.set_fullscreen(mode);
    }

    fn request_focus(&mut self) {
        self.raw.focus_window();
    }

    fn has_focus(&self) -> bool {
        self.has_focus()
    }

    fn set_cursor_icon(&mut self, cursor: CursorIcon) {
        todo!()
    }

    fn cursor(&self) -> CursorIcon {
        todo!()
    }

    fn set_maximized(&mut self, maximized: bool) {
        self.raw.set_maximized(maximized);
    }

    fn maximized(&self) -> bool {
        self.raw.is_maximized()
    }

    fn set_minimized(&mut self, minimized: bool) {
        self.raw.set_minimized(minimized);
    }

    fn minimized(&self) -> bool {
        self.raw.is_minimized().unwrap_or(false)
    }

    fn set_visible(&mut self, visible: bool) {
        self.raw.set_visible(visible);
    }

    fn visible(&self) -> bool {
        self.raw.is_visible().unwrap_or(false)
    }

    fn set_transparent(&mut self, transparent: bool) {
        todo!()
    }

    fn transparent(&self) -> bool {
        todo!()
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.raw.set_resizable(resizable);
    }

    fn resizable(&self) -> bool {
        self.raw.is_resizable()
    }
}
