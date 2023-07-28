use wgpu::RenderPipeline as RawRenderPipeline;
use crate::GKRenderPipeline;

pub struct Pipeline {
    pub(crate) raw: RawRenderPipeline,
}

impl GKRenderPipeline for Pipeline {}