use crate::Renderer;
use gk_sys::window::WindowId;

#[derive(Copy, Clone, Debug)]
pub struct DrawFrame {
    pub(crate) window_id: WindowId,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl DrawFrame {
    pub fn window_id(&self) -> WindowId {
        self.window_id
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn create_renderer(&self) -> Renderer {
        let mut renderer = Renderer::new();
        renderer.begin(self.width, self.height);
        renderer
    }
}
