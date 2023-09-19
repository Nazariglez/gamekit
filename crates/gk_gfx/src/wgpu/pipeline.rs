use crate::consts::{MAX_BIND_GROUPS_PER_PIPELINE, MAX_VERTEX_BUFFERS};
use crate::{BindGroupLayout, BindGroupLayoutId, GKRenderPipeline, VertexLayout};
use arrayvec::ArrayVec;
use wgpu::RenderPipeline as RawRenderPipeline;

pub struct RenderPipeline {
    pub(crate) raw: RawRenderPipeline,
    pub(crate) index_format: wgpu::IndexFormat,
    pub(crate) uses_depth: bool,
    pub(crate) uses_stencil: bool,
    pub(crate) bind_group_layout: ArrayVec<wgpu::BindGroupLayout, MAX_BIND_GROUPS_PER_PIPELINE>,
}

impl GKRenderPipeline for RenderPipeline {
    fn bind_group_layout(&self, index: u32) -> Result<BindGroupLayoutId, String> {
        let raw = self
            .bind_group_layout
            .get(index as _)
            .ok_or_else(|| format!("BindGroup '{}' is not available on the pipeline", index))?;

        Ok(BindGroupLayoutId { raw })
    }
}
