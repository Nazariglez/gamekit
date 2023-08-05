use crate::texture::GKTexture;
use wgpu::{Texture as RawTexture, TextureView};

pub struct Texture {
    pub(crate) raw: RawTexture,
    pub(crate) view: TextureView,
}

impl GKTexture for Texture {}
