use crate::texture::GKTexture;
use wgpu::Texture as RawTexture;

pub struct Texture {
    pub(crate) raw: RawTexture,
}

impl GKTexture for Texture {}
