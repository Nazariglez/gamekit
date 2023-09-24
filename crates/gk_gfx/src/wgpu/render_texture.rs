use crate::frame::GKDrawFrame;
use crate::render_target::RenderTarget;
use crate::render_texture::GKRenderTexture;
use crate::{Texture, TextureId};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RenderTexture {
    id: TextureId,
    texture: Texture,
    depth_texture: Texture,
}

impl GKRenderTexture for RenderTexture {
    fn id(&self) -> TextureId {
        self.texture.id
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }

    fn into_inner(self) -> Texture {
        let Self { texture, .. } = self;
        texture
    }
}

impl Deref for RenderTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        self.texture()
    }
}

impl<'a, DF> Into<RenderTarget<'a, DF, RenderTexture>> for &'a RenderTexture
where
    DF: GKDrawFrame,
{
    fn into(self) -> RenderTarget<'a, DF, RenderTexture> {
        RenderTarget::Texture(self)
    }
}
