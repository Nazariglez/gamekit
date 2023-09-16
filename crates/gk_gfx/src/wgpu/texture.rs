use crate::texture::GKTexture;
use wgpu::{Texture as RawTexture, TextureView};

pub struct Texture {
    pub(crate) raw: RawTexture,
    pub(crate) view: TextureView,
    pub(crate) size: (f32, f32),
}

impl GKTexture for Texture {
    fn size(&self) -> (f32, f32) {
        self.size
    }
}
