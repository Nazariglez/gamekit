use crate::GKBindGroup;
use wgpu::BindGroup as RawBindGroup;

pub struct BindGroup {
    pub(crate) raw: RawBindGroup,
}

impl GKBindGroup for BindGroup {}
