use crate::Pipeline;
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use hashbrown::HashMap;
use std::borrow::Cow;
use wgpu::{
    Device, Instance, PowerPreference, Queue, Surface, SurfaceCapabilities, SurfaceConfiguration,
    SurfaceTexture,
};

pub struct Context {
    surface: Surface,
    config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    capabilities: SurfaceCapabilities,
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

pub struct GfxDevice {
    instance: Instance,
    contexts: HashMap<GKWindowId, Context>,
    vsync: bool,
    compatibility_mode: bool,
    integrated_gpu: bool,
}

impl Plugin for GfxDevice {}

impl GfxDevice {
    pub fn new() -> Result<Self, String> {
        let instance = Instance::default();
        Ok(Self {
            instance,
            contexts: HashMap::default(),
            vsync: false,
            compatibility_mode: false,
            integrated_gpu: false,
        })
    }

    pub fn add_context<W: GKWindow>(&mut self, window: &W) -> Result<(), String> {
        let surface = unsafe { self.instance.create_surface(window) }.unwrap();
        let adapter =
            pollster::block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: if self.integrated_gpu {
                    PowerPreference::LowPower
                } else {
                    PowerPreference::HighPerformance
                },
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .ok_or_else(|| format!("Cannot create WGPU Adapter for {:?}", window.id()))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: if self.compatibility_mode {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None,
        ))
        .map_err(|err| err.to_string())?;

        let (width, height) = window.physical_size();
        let capabilities = surface.get_capabilities(&adapter);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width,
            height,
            present_mode: if self.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let ctx = Context {
            surface,
            config,
            device,
            queue,
            capabilities,
        };

        self.contexts.insert(window.id(), ctx);

        Ok(())
    }

    pub fn create_pipeline(&mut self, shader: &str) -> Result<Pipeline, String> {
        let (_, ctx) = self.contexts.iter().next().ok_or_else(|| {
            "There is no context available yet. Try to create a window first.".to_string()
        })?;
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader)),
            });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_format = ctx.capabilities.formats[0];

        let raw = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Ok(Pipeline { raw })
    }

    pub fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        if let Some(ctx) = self.contexts.get_mut(&id) {
            ctx.resize(width, height);
        }
    }

    pub fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        let ctx = self
            .contexts
            .get(&window)
            .ok_or_else(|| format!("No WGPU context for {:?}", window))?;
        let frame = ctx.frame()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&renderer.pipeline.raw);
            rpass.draw(0..3, 0..1);
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}

pub struct Renderer<'a> {
    pipeline: &'a Pipeline,
}

impl<'a> Renderer<'a> {
    pub fn new(pip: &'a Pipeline) -> Self {
        Self { pipeline: pip }
    }
}
