use crate::frame::GKDrawFrame;
use wgpu::{SurfaceTexture, TextureView};

pub struct DrawFrame {
    pub(crate) frame: SurfaceTexture,
    pub(crate) view: TextureView,
}

impl GKDrawFrame for DrawFrame {}
