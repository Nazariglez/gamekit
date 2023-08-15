use super::context::Context;
use crate::attrs::GfxAttributes;
use crate::Texture;
use gk_app::window::GKWindow;
use wgpu::{
    Device, Surface as RawSurface, SurfaceCapabilities, SurfaceConfiguration, SurfaceTexture,
};

pub(crate) struct Surface {
    pub surface: RawSurface,
    pub config: SurfaceConfiguration,
    pub capabilities: SurfaceCapabilities,
    pub depth_texture: Option<Texture>,
}

impl Surface {
    pub fn new<W: GKWindow>(
        ctx: &mut Context,
        window: &W,
        attrs: GfxAttributes,
    ) -> Result<Self, String> {
        log::trace!("Creating a new Surface for Window {:?}", window.id());
        let surface = unsafe { ctx.instance.create_surface(window) }.map_err(|e| e.to_string())?;

        if !ctx.is_surface_compatible(&surface) {
            log::trace!(
                "Generating WGPU adapter compatible with {:?} surface.",
                window.id()
            );
            ctx.ensure_surface_compatibility(&surface)?;
        }

        let (width, height) = window.physical_size();
        let capabilities = surface.get_capabilities(&ctx.adapter);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width,
            height,
            present_mode: if attrs.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&ctx.device, &config);

        Ok(Self {
            surface,
            config,
            capabilities,
            depth_texture: None,
        })
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(device, &self.config);
    }

    pub fn frame(&self) -> Result<SurfaceTexture, String> {
        self.surface
            .get_current_texture()
            .map_err(|e| e.to_string())
    }

    pub fn add_depth_texture(&mut self, tex: Texture) {
        self.depth_texture = Some(tex);
    }
}
