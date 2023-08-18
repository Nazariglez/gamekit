use crate::GKRenderPipeline;
use wgpu::RenderPipeline as RawRenderPipeline;

pub struct RenderPipeline {
    pub(crate) raw: RawRenderPipeline,
    pub(crate) index_format: wgpu::IndexFormat,
    pub(crate) uses_depth: bool,
    pub(crate) uses_stencil: bool,
}

impl GKRenderPipeline for RenderPipeline {}
