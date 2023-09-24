use crate::{Texture, TextureFormat, TextureId};
use gk_macro::ResourceId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ResourceId)]
pub struct RenderTextureId(u64);

pub trait GKRenderTexture {
    fn id(&self) -> RenderTextureId;
    fn texture(&self) -> &Texture;
    fn into_inner(self) -> Texture;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RenderTextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub format: TextureFormat,
    pub depth: bool,
    pub width: u32,
    pub height: u32,
}
