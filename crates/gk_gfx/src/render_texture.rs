use crate::{Texture, TextureId};

pub trait GKRenderTexture {
    fn id(&self) -> TextureId;
    fn texture(&self) -> &Texture;
    fn into_inner(self) -> Texture;
}
