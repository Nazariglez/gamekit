use super::context::Context;
use crate::device::{GKDevice, GKRenderPipeline, RenderPipelineDescriptor};
use crate::wgpu::utils::wgpu_color;
use crate::{RenderPipeline, Renderer};
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use hashbrown::HashMap;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::borrow::Cow;
use std::ops::Range;
pub use wgpu::Color;
use wgpu::{
    Device as RawDevice, Instance, PowerPreference, Queue, Surface, SurfaceCapabilities,
    SurfaceConfiguration, SurfaceTexture,
};

pub struct Device {
    instance: Instance,
    contexts: HashMap<GKWindowId, Context>,
    vsync: bool,
    compatibility_mode: bool,
    integrated_gpu: bool,
}

impl Plugin for Device {}

impl GKDevice<RenderPipeline> for Device {
    fn new() -> Result<Self, String> {
        let instance = Instance::default();
        Ok(Self {
            instance,
            contexts: HashMap::default(),
            vsync: false,
            compatibility_mode: false,
            integrated_gpu: false,
        })
    }

    fn init_context<W: GKWindow>(&mut self, window: &W) -> Result<(), String> {
        if self.contexts.contains_key(&window.id()) {
            return Ok(());
        }

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
            .ok_or_else(|| "Cannot create WGPU Adapter for {:?}".to_string())?;

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

    fn create_render_pipeline(
        &mut self,
        desc: RenderPipelineDescriptor,
    ) -> Result<RenderPipeline, String> {
        let (_, ctx) = self.contexts.iter().next().ok_or_else(|| {
            "There is no context available yet. Try to create a window first.".to_string()
        })?;
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: desc.label,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(desc.shader)),
            });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: desc.label,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_format = ctx.capabilities.formats[0];

        let raw = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: desc.label,
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
                depth_stencil: None, // todo from desc
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Ok(RenderPipeline { raw })
    }

    fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        if let Some(ctx) = self.contexts.get_mut(&id) {
            ctx.resize(width, height);
        }
    }

    fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
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

        renderer.passes.iter().for_each(|rp| {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu_color(rp.color)),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            if let Some(pip) = rp.pipeline {
                rpass.set_pipeline(&pip.raw);
                if !rp.vertices.is_empty() {
                    rpass.draw(rp.vertices.clone(), 0..1);
                }
            }
        });

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
