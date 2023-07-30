use crate::GfxAttributes;
use wgpu::{
    Adapter, Device, Instance, PowerPreference, Queue, Surface as RawSurface, SurfaceCapabilities,
    SurfaceConfiguration, SurfaceTexture,
};

pub(crate) struct Context {
    /// force the use of integrated gpu
    pub integrated_gpu: bool,
    /// force limits compatibles with webgl2
    pub compatibility_mode: bool,

    // - wgpu inner types
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl Context {
    pub fn new(attrs: GfxAttributes) -> Result<Self, String> {
        let instance = Instance::default();
        let (adapter, device, queue) = pollster::block_on(generate_inner(
            &instance,
            None,
            attrs.compatible_mode,
            attrs.integrated_gpu,
        ))?;

        Ok(Self {
            integrated_gpu: attrs.integrated_gpu,
            compatibility_mode: attrs.compatible_mode,
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn is_surface_compatible(&self, surface: &RawSurface) -> bool {
        self.adapter.is_surface_supported(surface)
    }

    pub fn ensure_surface_compatibility(&mut self, surface: &RawSurface) -> Result<(), String> {
        let (adapter, device, queue) = pollster::block_on(generate_inner(
            &self.instance,
            Some(surface),
            self.compatibility_mode,
            self.integrated_gpu,
        ))?;
        self.adapter = adapter;
        self.device = device;
        self.queue = queue;
        Ok(())
    }
}

async fn generate_inner(
    instance: &Instance,
    surface: Option<&RawSurface>,
    compatibility_mode: bool,
    integrated_gpu: bool,
) -> Result<(Adapter, Device, Queue), String> {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: if integrated_gpu {
                PowerPreference::LowPower
            } else {
                PowerPreference::HighPerformance
            },
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .ok_or_else(|| "Cannot create WGPU Adapter for {:?}".to_string())?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: if compatibility_mode {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None,
        )
        .await
        .map_err(|err| err.to_string())?;

    Ok((adapter, device, queue))
}
