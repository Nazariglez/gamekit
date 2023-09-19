use crate::{GKBindGroup, GKBindGroupLayoutId};
use wgpu::{BindGroup as RawBindGroup, BindGroupLayout};

pub struct BindGroup {
    pub(crate) raw: RawBindGroup,
}

impl GKBindGroup for BindGroup {}

pub struct BindGroupLayoutId {
    pub(crate) raw: BindGroupLayout,
}

impl GKBindGroupLayoutId for BindGroupLayoutId {}

impl PartialEq for BindGroupLayoutId {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
