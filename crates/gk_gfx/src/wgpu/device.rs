use super::buffer::Buffer;
use super::context::Context;
use super::pipeline::RenderPipeline;
use super::surface::Surface;
use super::utils::wgpu_color;
use crate::device::{GKDevice, GKRenderPipeline, RenderPipelineDescriptor};
use crate::wgpu::utils::{wgpu_buffer_usages, wgpu_step_mode, wgpu_vertex_format};
use crate::{BufferDescriptor, BufferUsage, GfxAttributes, Renderer};
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use hashbrown::HashMap;
use std::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
pub use wgpu::Color;
use wgpu::IndexFormat;

pub struct Device {
    attrs: GfxAttributes,
    ctx: Context,
    surfaces: HashMap<GKWindowId, Surface>,
}

impl Plugin for Device {}

impl GKDevice<RenderPipeline, Buffer> for Device {
    fn new(attrs: GfxAttributes) -> Result<Self, String> {
        let context = Context::new(attrs)?;
        Ok(Self {
            attrs,
            ctx: context,
            surfaces: HashMap::default(),
        })
    }

    fn init_surface<W: GKWindow>(&mut self, window: &W) -> Result<(), String> {
        if self.surfaces.contains_key(&window.id()) {
            return Ok(());
        }

        let surface = Surface::new(&mut self.ctx, window, self.attrs)?;
        self.surfaces.insert(window.id(), surface);

        Ok(())
    }

    fn create_render_pipeline(
        &mut self,
        desc: RenderPipelineDescriptor,
    ) -> Result<RenderPipeline, String> {
        let shader = self
            .ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: desc.label,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(desc.shader)),
            });

        let pipeline_layout =
            self.ctx
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: desc.label,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let swapchain_format = self
            .surfaces
            .iter()
            .next()
            .map_or(wgpu::TextureFormat::Rgba8UnormSrgb, |(_, surface)| {
                surface.capabilities.formats[0]
            });

        let (attrs, mut buffers) = match &desc.vertex_layout {
            Some(vl) => {
                let mut offset = 0;
                let attrs = vl
                    .attributes
                    .iter()
                    .map(|attr| {
                        let a = wgpu::VertexAttribute {
                            format: wgpu_vertex_format(attr.format),
                            offset,
                            shader_location: attr.location as _,
                        };
                        offset += a.format.size();
                        a
                    })
                    .collect::<Vec<_>>();

                let layout = wgpu::VertexBufferLayout {
                    array_stride: offset,
                    step_mode: wgpu_step_mode(vl.step_mode),
                    attributes: &[],
                };

                (vec![attrs], vec![layout]) // todo multple layout?
            }
            _ => (vec![], vec![]),
        };

        buffers
            .iter_mut()
            .enumerate()
            .for_each(|(i, buff)| buff.attributes = &attrs[i]);

        let raw = self
            .ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: desc.label,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &buffers,
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

    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<Buffer, String> {
        let raw = self.ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: desc.label,
            contents: desc.content,
            usage: wgpu_buffer_usages(desc.usage),
        });

        let usage = desc.usage;

        Ok(Buffer { raw, usage })
    }

    fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        if let Some(surface) = self.surfaces.get_mut(&id) {
            surface.resize(&self.ctx.device, width, height);
        }
    }

    fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        let surface = self
            .surfaces
            .get(&window)
            .ok_or_else(|| format!("No WGPU context for {:?}", window))?;
        let frame = surface.frame()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .ctx
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

                rp.buffers.iter().for_each(|buff| {
                    match buff.usage {
                        BufferUsage::Vertex => rpass.set_vertex_buffer(0, buff.raw.slice(..)),
                        BufferUsage::Index => {
                            rpass.set_index_buffer(buff.raw.slice(..), IndexFormat::default())
                        } // TODO indexformat!
                        BufferUsage::Uniform => {
                            // TODO
                        }
                    }
                });

                if !rp.vertices.is_empty() {
                    rpass.draw(rp.vertices.clone(), 0..1);
                }
            }
        });

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
