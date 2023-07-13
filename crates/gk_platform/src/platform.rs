use crate::{GKWindow, GKWindowAttributes, GKWindowId, GKWindowManager};
use gk_app::Plugin;
use std::marker::PhantomData;

#[derive(Default)]
pub struct WindowManager<W, M>
where
    W: GKWindow + 'static,
    M: GKWindowManager<W> + 'static,
{
    pub(crate) manager: M,
    main_window: Option<GKWindowId>,
    _w: PhantomData<W>,
}

impl<W, M> WindowManager<W, M>
where
    W: GKWindow + 'static,
    M: GKWindowManager<W> + 'static,
{
    pub fn create(&mut self) -> WindowBuilder<W, M> {
        WindowBuilder::new(self)
    }

    pub fn window(&mut self, id: GKWindowId) -> Option<&mut W> {
        self.manager.window(id)
    }

    pub fn main_window(&mut self) -> Option<&mut W> {
        self.main_window.and_then(|id| self.window(id))
    }

    pub fn set_main_window(&mut self, win_id: GKWindowId) {
        self.main_window = Some(win_id);
    }

    pub fn exit(&mut self) {
        self.manager.exit();
    }
}

impl<W, M> Plugin for WindowManager<W, M>
where
    W: GKWindow + 'static,
    M: GKWindowManager<W> + 'static,
{
}

pub struct WindowBuilder<'a, W, M>
where
    W: GKWindow + 'static,
    M: GKWindowManager<W> + 'static,
{
    manager: &'a mut WindowManager<W, M>,
    attrs: GKWindowAttributes,
}

impl<'a, W, M> WindowBuilder<'a, W, M>
where
    W: GKWindow + 'static,
    M: GKWindowManager<W> + 'static,
{
    fn new(manager: &'a mut WindowManager<W, M>) -> Self {
        Self {
            manager,
            attrs: Default::default(),
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.attrs.size = Some((width, height));
        self
    }

    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.attrs.min_size = Some((width, height));
        self
    }

    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.attrs.max_size = Some((width, height));
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.attrs.position = Some((x, y));
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.attrs.resizable = resizable;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.attrs.title = title.to_string();
        self
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.attrs.fullscreen = fullscreen;
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.attrs.maximized = maximized;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.attrs.visible = visible;
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.attrs.transparent = transparent;
        self
    }

    pub fn build(self) -> Result<GKWindowId, String> {
        let Self { manager, attrs } = self;
        manager.manager.create(attrs)
    }
}
