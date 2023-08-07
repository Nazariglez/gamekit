use super::buffer::Buffer;
use super::context::Context;
use super::pipeline::RenderPipeline;
use super::surface::Surface;
use super::texture::Texture;
use super::utils::wgpu_color;
use crate::attrs::GfxAttributes;
use crate::buffer::{BufferDescriptor, BufferUsage};
use crate::device::GKDevice;
use crate::pipeline::RenderPipelineDescriptor;
use crate::renderer::Renderer;
use crate::texture::TextureDescriptor;
use crate::wgpu::utils::{
    wgpu_blend_mode, wgpu_buffer_usages, wgpu_index_format, wgpu_primitive, wgpu_shader_visibility,
    wgpu_step_mode, wgpu_texture_filter, wgpu_texture_format, wgpu_texture_wrap,
    wgpu_vertex_format,
};
use crate::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Sampler, SamplerDescriptor, TextureData,
    MAX_BINDING_ENTRIES,
};
use arrayvec::ArrayVec;
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use hashbrown::HashMap;
use std::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BlendComponent, IndexFormat, TextureDimension};

pub struct Device {
    attrs: GfxAttributes,
    ctx: Context,
    surfaces: HashMap<GKWindowId, Surface>,
}

impl Plugin for Device {}

impl GKDevice<RenderPipeline, Buffer, Texture, Sampler, BindGroup> for Device {
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

        let bind_group_layouts = &desc.bind_group_layout.map_or(vec![], |bg| vec![&bg.layout]);
        let pipeline_layout =
            self.ctx
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: desc.label,
                    bind_group_layouts,
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

        let swapchain_color_target: wgpu::ColorTargetState = swapchain_format.into();
        let color_target = wgpu::ColorTargetState {
            blend: desc.blend_mode.map_or(None, |bm| Some(wgpu_blend_mode(bm))),
            ..swapchain_color_target
        };

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
                    targets: &[Some(color_target)],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu_primitive(desc.primitive),
                    ..Default::default()
                },
                depth_stencil: None, // todo from desc
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        let index_format = wgpu_index_format(desc.index_format);
        Ok(RenderPipeline { raw, index_format })
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

    fn create_texture(
        &mut self,
        desc: TextureDescriptor,
        data: Option<TextureData>,
    ) -> Result<Texture, String> {
        let size = data.map_or(wgpu::Extent3d::default(), |d| wgpu::Extent3d {
            width: d.width,
            height: d.height,
            depth_or_array_layers: 1,
        });
        let raw = self.ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: desc.label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: wgpu_texture_format(desc.format),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        if let Some(d) = data {
            self.ctx.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &raw,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                d.bytes,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(d.width * 4),
                    rows_per_image: Some(d.height),
                },
                size,
            );
        }

        let view = raw.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Texture { raw, view })
    }

    fn create_sampler(&mut self, desc: SamplerDescriptor) -> Result<Sampler, String> {
        let raw = self.ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: desc.label,
            address_mode_u: wgpu_texture_wrap(desc.wrap_x),
            address_mode_v: wgpu_texture_wrap(desc.wrap_y),
            address_mode_w: wgpu_texture_wrap(desc.wrap_z),
            mag_filter: wgpu_texture_filter(desc.mag_filter),
            min_filter: wgpu_texture_filter(desc.min_filter),
            mipmap_filter: desc
                .mipmap_filter
                .map_or(Default::default(), |f| wgpu_texture_filter(f)),
            ..Default::default()
        });
        Ok(Sampler { raw })
    }

    fn create_bind_group(&mut self, desc: BindGroupDescriptor) -> Result<BindGroup, String> {
        let mut layout_entries: ArrayVec<_, MAX_BINDING_ENTRIES> = Default::default();
        desc.entry.iter().for_each(|entry| match entry {
            BindGroupEntry::Texture(binding) => {
                let visibility = wgpu_shader_visibility(
                    binding.visible_vertex,
                    binding.visible_fragment,
                    binding.visible_compute,
                );
                if let Some((loc, _tex)) = binding.texture {
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: loc,
                        visibility,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    });
                }

                if let Some((loc, _sampler)) = binding.sampler {
                    layout_entries.push(wgpu::BindGroupLayoutEntry {
                        binding: loc,
                        visibility,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    });
                }
            }
            BindGroupEntry::Uniform(binding) => {
                let visibility = wgpu_shader_visibility(
                    binding.visible_vertex,
                    binding.visible_fragment,
                    binding.visible_compute,
                );

                layout_entries.push(wgpu::BindGroupLayoutEntry {
                    binding: binding.location,
                    visibility,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                });
            }
        });
        let layout = self
            .ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: desc.label,
                entries: &layout_entries,
            });

        let mut entries: ArrayVec<_, MAX_BINDING_ENTRIES> = Default::default();
        desc.entry.iter().for_each(|entry| match entry {
            BindGroupEntry::Texture(binding) => {
                if let Some((loc, tex)) = binding.texture {
                    entries.push(wgpu::BindGroupEntry {
                        binding: loc,
                        resource: wgpu::BindingResource::TextureView(&tex.view),
                    });
                }

                if let Some((loc, sampler)) = binding.sampler {
                    entries.push(wgpu::BindGroupEntry {
                        binding: loc,
                        resource: wgpu::BindingResource::Sampler(&sampler.raw),
                    });
                }
            }
            BindGroupEntry::Uniform(binding) => {
                entries.push(wgpu::BindGroupEntry {
                    binding: binding.location,
                    resource: binding.uniform.raw.as_entire_binding(),
                });
            }
        });
        let raw = self
            .ctx
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: desc.label,
                layout: &layout,
                entries: &entries,
            });

        Ok(BindGroup { raw, layout })
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

                let mut vertex_buffers_slot = 0;
                let mut indexed = false;
                rp.buffers.iter().for_each(|buff| match buff.usage {
                    BufferUsage::Vertex => {
                        rpass.set_vertex_buffer(vertex_buffers_slot, buff.raw.slice(..));
                        vertex_buffers_slot += 1;
                    }
                    BufferUsage::Index => {
                        debug_assert!(!indexed, "Cannot bind more than one Index buffer");
                        indexed = true;
                        rpass.set_index_buffer(buff.raw.slice(..), pip.index_format)
                    }
                    BufferUsage::Uniform => {}
                });

                if let Some(bg) = rp.bind_group {
                    rpass.set_bind_group(0, &bg.raw, &[]);
                }

                if !rp.vertices.is_empty() {
                    if indexed {
                        rpass.draw_indexed(rp.vertices.clone(), 0, 0..1);
                    } else {
                        rpass.draw(rp.vertices.clone(), 0..1);
                    }
                }
            }
        });

        self.ctx.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
