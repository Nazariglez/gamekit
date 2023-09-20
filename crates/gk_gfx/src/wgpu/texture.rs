use std::sync::Arc;
use crate::texture::GKTexture;
use wgpu::{Texture as RawTexture, TextureView};

#[derive(Clone)]
pub struct Texture {
    pub(crate) raw: Arc<RawTexture>,
    pub(crate) view: Arc<TextureView>,
    pub(crate) size: (f32, f32),
}

impl GKTexture for Texture {
    fn size(&self) -> (f32, f32) {
        self.size
    }
}
