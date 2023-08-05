use crate::texture::GKSampler;
use wgpu::Sampler as RawSampler;

pub struct Sampler {
    pub(crate) raw: RawSampler,
}

impl GKSampler for Sampler {}
