use crate::frame::GKDrawFrame;
use crate::render_texture::GKRenderTexture;

#[derive(Debug)]
pub enum RenderTarget<'a, DF, RT>
where
    DF: GKDrawFrame + 'a,
    RT: GKRenderTexture + 'a,
{
    Frame(&'a DF),
    Texture(&'a RT),
}
