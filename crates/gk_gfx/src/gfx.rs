use crate::renderer::Renderer;
use crate::{Device, Pipeline};
use crate::{GKDevice, RenderPipelineDescriptor};
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct Gfx {
    pub(crate) raw: Device,
}

impl Plugin for Gfx {}

impl Gfx {
    pub fn new() -> Result<Self, String> {
        let raw = Device::new()?;
        Ok(Self { raw })
    }

    pub fn init_context<W: GKWindow>(&mut self, win: &W) -> Result<(), String> {
        self.raw.init_context(win)
    }

    pub fn create_render_pipeline(
        &mut self,
        desc: RenderPipelineDescriptor,
    ) -> Result<Pipeline, String> {
        self.raw.create_render_pipeline(desc)
    }

    pub fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        self.raw.resize(id, width, height);
    }

    pub fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        self.raw.render(window, renderer)
    }
}
