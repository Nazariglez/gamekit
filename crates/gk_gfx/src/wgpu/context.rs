use wgpu::{Device, Queue, Surface, SurfaceCapabilities, SurfaceConfiguration, SurfaceTexture};

pub(crate) struct Context {
    pub surface: Surface,
    pub config: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    pub capabilities: SurfaceCapabilities,
}

impl Context {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn frame(&self) -> Result<SurfaceTexture, String> {
        self.surface
            .get_current_texture()
            .map_err(|e| e.to_string())
    }
}
