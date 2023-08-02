use crate::GKRenderPipeline;
use wgpu::RenderPipeline as RawRenderPipeline;

pub struct RenderPipeline {
    pub(crate) raw: RawRenderPipeline,
    pub(crate) index_format: wgpu::IndexFormat,
}

impl GKRenderPipeline for RenderPipeline {}
