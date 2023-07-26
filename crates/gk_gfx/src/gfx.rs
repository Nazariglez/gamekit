use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use hashbrown::HashMap;
use wgpu::{Device, Instance, PowerPreference, Queue, Surface, SurfaceConfiguration};

pub struct Context {
    surface: Surface,
    config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
}

impl Context {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
}

pub struct GfxDevice {
    instance: Instance,
    contexts: HashMap<GKWindowId, Context>,
}

impl Plugin for GfxDevice {}

impl GfxDevice {
    pub fn new() -> Result<Self, String> {
        let instance = Instance::default();
        Ok(Self {
            instance,
            contexts: HashMap::default(),
        })
    }

    pub fn add_context<W: GKWindow>(&mut self, window: &W) -> Result<(), String> {
        let surface = unsafe { self.instance.create_surface(window) }.unwrap();
        let adapter =
            pollster::block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .ok_or_else(|| format!("Cannot create WGPU Adapter for {:?}", window.id()))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: wgpu::Limits::downlevel_webgl2_defaults(), // todo allow to set this from config
            },
            None,
        ))
        .map_err(|err| err.to_string())?;

        let (width, height) = window.physical_size();
        let caps = surface.get_capabilities(&adapter);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&self.device, &config);

        let ctx = Context {
            surface,
            config,
            device,
            queue,
        };

        self.contexts.insert(window.id(), ctx);

        Ok(())
    }

    fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        if let Some(ctx) = self.contexts.get_mut(&id) {
            ctx.resize(width, height);
        }
    }
}
