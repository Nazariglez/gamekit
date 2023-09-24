use crate::frame::GKDrawFrame;
use crate::render_target::RenderTarget;
use crate::render_texture::{GKRenderTexture, RenderTextureId};
use crate::Texture;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RenderTexture {
    pub(crate) id: RenderTextureId,
    pub(crate) texture: Texture,
    pub(crate) depth_texture: Option<Texture>,
}

impl GKRenderTexture for RenderTexture {
    fn id(&self) -> RenderTextureId {
        self.id
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
