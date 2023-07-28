use crate::GKRenderPipeline;
use wgpu::RenderPipeline as RawRenderPipeline;

pub struct Pipeline {
    pub(crate) raw: RawRenderPipeline,
}

impl GKRenderPipeline for Pipeline {}
