use crate::frame::GKDrawFrame;
use crate::wgpu::surface::Surface;
use gk_sys::window::WindowId;
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

pub struct DrawFrame {
    pub(crate) window_id: WindowId,
    pub(crate) surface: Surface,
    pub(crate) frame: SurfaceTexture,
    pub(crate) view: TextureView,
    pub(crate) encoder: CommandEncoder,
}

impl GKDrawFrame for DrawFrame {}
