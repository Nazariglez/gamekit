use super::pipeline::Pipeline;
use crate::renderer::Renderer;
use crate::{GKDevice, RenderPipelineDescriptor};
use gk_app::window::{GKWindow, GKWindowId};

pub struct Device;

impl GKDevice<Pipeline> for Device {
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(Self)
    }
    fn init_context<W: GKWindow>(&mut self, win: &W) -> Result<(), String> {
        Ok(())
    }
    fn create_render_pipeline(
        &mut self,
        _desc: RenderPipelineDescriptor,
    ) -> Result<Pipeline, String> {
        Ok(Pipeline)
    }
    fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        // no-op
    }
    fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        Ok(())
    }
}