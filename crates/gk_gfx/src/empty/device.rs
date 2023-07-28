use std::hash::Hash;
use raw_window_handle::HasRawWindowHandle;
use super::pipeline::Pipeline;
use crate::{GKDevice, RenderPipelineDescriptor};

pub struct Device;

impl Device {
    pub fn new() -> Result<Self, String> {
        Ok(Self)
    }
}

impl GKDevice<Pipeline> for Device {
    fn init_context<H: HasRawWindowHandle>(&mut self, _id: &H) -> Result<(), String> {
        Ok(())
    }

    fn create_render_pipeline(&mut self, _desc: RenderPipelineDescriptor) -> Result<Pipeline, String> {
        Ok(Pipeline)
    }
}

