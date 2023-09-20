use crate::texture::{GKTexture, TextureId};
use std::sync::Arc;
use wgpu::{Texture as RawTexture, TextureView};

#[derive(Clone)]
pub struct Texture {
    pub(crate) id: TextureId,
    pub(crate) raw: Arc<RawTexture>,
    pub(crate) view: Arc<TextureView>,
    pub(crate) size: (f32, f32),
}

impl GKTexture for Texture {
    fn id(&self) -> TextureId {
        self.id
    }

    fn size(&self) -> (f32, f32) {
        self.size
    }
}
