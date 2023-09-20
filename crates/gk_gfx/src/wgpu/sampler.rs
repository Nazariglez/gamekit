use std::sync::Arc;
use crate::texture::GKSampler;
use wgpu::Sampler as RawSampler;

#[derive(Clone)]
pub struct Sampler {
    pub(crate) raw: Arc<RawSampler>,
}

impl GKSampler for Sampler {}
