use crate::consts::MAX_BIND_GROUPS_PER_PIPELINE;
use crate::{BindGroupLayoutId, BindGroupLayoutRef, GKRenderPipeline};
use arrayvec::ArrayVec;
use wgpu::RenderPipeline as RawRenderPipeline;

pub struct RenderPipeline {
    pub(crate) raw: RawRenderPipeline,
    pub(crate) index_format: wgpu::IndexFormat,
    pub(crate) uses_depth: bool,
    pub(crate) uses_stencil: bool,
    pub(crate) bind_group_layout: ArrayVec<BindGroupLayoutRef, MAX_BIND_GROUPS_PER_PIPELINE>,
}

impl GKRenderPipeline for RenderPipeline {
    fn bind_group_layout_id(&self, index: u32) -> Result<&BindGroupLayoutRef, String> {
        self.bind_group_layout
            .get(index as usize)
            .ok_or_else(|| format!("Invalid Bind Group '{}' in pipeline", index))
    }
}
