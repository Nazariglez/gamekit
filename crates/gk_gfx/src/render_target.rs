use crate::frame::GKDrawFrame;
use crate::texture::GKTexture;

// TODO the enum can be moved to the sys but not impl of From which depends on the inner types
#[derive(Debug)]
pub enum RenderTarget<'a, DF, T>
where
    DF: GKDrawFrame + 'a,
    T: GKTexture + 'a,
{
    Frame(&'a DF),
    Texture(&'a T),
}
