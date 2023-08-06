use crate::GKBindGroup;
use wgpu::BindGroup as RawBindGroup;

pub struct BindGroup {
    pub(crate) raw: RawBindGroup,
    pub(crate) layout: wgpu::BindGroupLayout,
}

impl GKBindGroup for BindGroup {}
