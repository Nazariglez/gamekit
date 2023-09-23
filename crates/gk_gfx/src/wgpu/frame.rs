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
    pub(crate) present_check: FramePresented,
}

impl GKDrawFrame for DrawFrame {}

#[derive(Default)]
pub(crate) struct FramePresented(bool);
impl FramePresented {
    pub fn validate(&mut self) {
        self.0 = true;
    }
}

impl Drop for FramePresented {
    fn drop(&mut self) {
        debug_assert!(self.0, "DrawFrame must be presented before drop it");
        log::error!("DrawFrame must be presented before drop it");
    }
}
