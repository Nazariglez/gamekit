use crate::GKRenderPipeline;
use wgpu::RenderPipeline as RawRenderPipeline;

pub struct RenderPipeline {
    pub(crate) raw: RawRenderPipeline,
}

impl GKRenderPipeline for RenderPipeline {}
