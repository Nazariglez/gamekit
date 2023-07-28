use raw_window_handle::HasRawWindowHandle;
use gk_app::Plugin;
use gk_app::window::GKWindowId;
use crate::{GKDevice, RenderPipelineDescriptor};
use crate::{Device, Pipeline};
use crate::renderer::Renderer;

pub struct Gfx {
    pub(crate) raw: Device
}

impl Plugin for Gfx {}

impl Gfx {
    pub fn new() -> Result<Self, String> {
        let raw = Device::new()?;
        Ok(Self {
            raw
        })
    }

    pub fn init_context<H: HasRawWindowHandle>(&mut self, id: &H) -> Result<(), String> {
        self.raw.init_context(id)
    }

    pub fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<Pipeline, String> {
        self.raw.create_render_pipeline(desc)
    }

    pub fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
    }

    pub fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        Ok(())
    }
}