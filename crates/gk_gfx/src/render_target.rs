use crate::frame::GKDrawFrame;
use crate::texture::GKTexture;

#[derive(Debug)]
pub enum RenderTarget<'a, DF, T>
where
    DF: GKDrawFrame + 'a,
    T: GKTexture + 'a,
{
    Frame(&'a DF),
    Texture(&'a T),
}
