use crate::renderer::Renderer;
use crate::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BlendMode, Buffer, BufferDescriptor,
    BufferUsage, Device, GKBuffer, GfxAttributes, GfxConfig, IndexFormat, Primitive,
    RenderPipeline, Sampler, SamplerDescriptor, Texture, TextureBinding, TextureData,
    TextureDescriptor, TextureFilter, TextureFormat, TextureWrap, UniformBinding, VertexLayout,
};
use crate::{GKDevice, RenderPipelineDescriptor};
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;
use image::EncodableLayout;

pub struct Gfx {
    pub(crate) raw: Device,
}

impl Plugin for Gfx {}

impl Gfx {
    pub fn new(attrs: GfxAttributes) -> Result<Self, String> {
        let raw = Device::new(attrs)?;
        Ok(Self { raw })
    }

    pub fn config() -> GfxConfig {
        GfxConfig::default()
    }

    pub fn init_surface<W: GKWindow>(&mut self, win: &W) -> Result<(), String> {
        self.raw.init_surface(win)
    }

    pub fn create_render_pipeline<'a>(&'a mut self, shader: &'a str) -> RenderPipelineBuilder {
        RenderPipelineBuilder::new(self, shader)
    }

    pub fn create_vertex_buffer<'a, D: bytemuck::Pod>(
        &'a mut self,
        data: &'a [D],
    ) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Vertex, data)
    }

    pub fn create_index_buffer<'a, D: bytemuck::Pod>(&'a mut self, data: &'a [D]) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Index, data)
    }

    pub fn create_uniform_buffer<'a, D: bytemuck::Pod>(
        &'a mut self,
        data: &'a [D],
    ) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Uniform, data)
    }

    pub fn create_texture(&mut self) -> TextureBuilder {
        TextureBuilder::new(self)
    }

    pub fn write_buffer<'a>(&'a mut self, buffer: &'a Buffer) -> BufferWriteBuilder {
        BufferWriteBuilder::new(self, buffer)
    }

    pub fn create_sampler(&mut self) -> SamplerBuilder {
        SamplerBuilder::new(self)
    }

    pub fn create_bind_group(&mut self) -> BindGroupBuilder {
        BindGroupBuilder::new(self)
    }

    pub fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        self.raw.resize(id, width, height);
    }

    pub fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        self.raw.render(window, renderer)
    }
}

pub struct RenderPipelineBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: RenderPipelineDescriptor<'a>,
}

impl<'a> RenderPipelineBuilder<'a> {
    fn new(gfx: &'a mut Gfx, shader: &'a str) -> Self {
        let desc = RenderPipelineDescriptor {
            shader,
            ..Default::default()
        };
        Self { desc, gfx }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_vertex_layout(mut self, layout: VertexLayout) -> Self {
        self.desc.vertex_layout = Some(layout);
        self
    }

    pub fn with_index_format(mut self, format: IndexFormat) -> Self {
        self.desc.index_format = format;
        self
    }

    pub fn with_primitive(mut self, primitive: Primitive) -> Self {
        self.desc.primitive = primitive;
        self
    }

    pub fn with_bind_group(mut self, bind_group: &'a BindGroup) -> Self {
        self.desc.bind_group_layout = Some(bind_group);
        self
    }

    pub fn with_blend_mode(mut self, mode: BlendMode) -> Self {
        self.desc.blend_mode = Some(mode);
        self
    }

    pub fn build(self) -> Result<RenderPipeline, String> {
        let Self { desc, gfx } = self;
        gfx.raw.create_render_pipeline(desc)
    }
}

pub struct BufferBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: BufferDescriptor<'a>,
}

impl<'a> BufferBuilder<'a> {
    fn new<D: bytemuck::Pod>(gfx: &'a mut Gfx, usage: BufferUsage, data: &'a [D]) -> Self {
        let desc = BufferDescriptor {
            content: bytemuck::cast_slice(data),
            usage,
            ..Default::default()
        };
        Self { gfx, desc }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_static(mut self, is_static: bool) -> Self {
        self.desc.is_static = is_static;
        self
    }

    pub fn build(self) -> Result<Buffer, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_buffer(desc)
    }
}

enum TextureRawData<'a> {
    Empty,
    Image(&'a [u8]),
    Raw {
        bytes: &'a [u8],
        width: u32,
        height: u32,
    },
}

pub struct TextureBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: TextureDescriptor<'a>,
    data: TextureRawData<'a>,
}

impl<'a> TextureBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx) -> Self {
        let desc = TextureDescriptor::default();
        let data = TextureRawData::Empty;
        Self { gfx, desc, data }
    }

    pub fn from_image(mut self, image: &'a [u8]) -> Self {
        self.data = TextureRawData::Image(image);
        self
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.desc.format = format;
        self
    }

    pub fn build(self) -> Result<Texture, String> {
        let Self { gfx, desc, data } = self;
        match data {
            TextureRawData::Empty => gfx.raw.create_texture(desc, None),
            TextureRawData::Image(bytes) => {
                let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
                let rgba = img.to_rgba8();
                gfx.raw.create_texture(
                    desc,
                    Some(TextureData {
                        bytes: rgba.as_bytes(),
                        width: rgba.width(),
                        height: rgba.height(),
                    }),
                )
            }
            TextureRawData::Raw {
                bytes,
                width,
                height,
            } => gfx.raw.create_texture(
                desc,
                Some(TextureData {
                    bytes,
                    width,
                    height,
                }),
            ),
        }
    }
}

pub struct SamplerBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: SamplerDescriptor<'a>,
}

impl<'a> SamplerBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx) -> Self {
        let desc = SamplerDescriptor::default();
        Self { gfx, desc }
    }

    pub fn with_wrap_x(mut self, wrap: TextureWrap) -> Self {
        self.desc.wrap_x = wrap;
        self
    }

    pub fn with_wrap_y(mut self, wrap: TextureWrap) -> Self {
        self.desc.wrap_y = wrap;
        self
    }

    pub fn with_wrap_z(mut self, wrap: TextureWrap) -> Self {
        self.desc.wrap_z = wrap;
        self
    }

    pub fn with_min_filter(mut self, filter: TextureFilter) -> Self {
        self.desc.min_filter = filter;
        self
    }

    pub fn with_mag_filter(mut self, filter: TextureFilter) -> Self {
        self.desc.mag_filter = filter;
        self
    }

    pub fn with_mipmap_filter(mut self, filter: TextureFilter) -> Self {
        self.desc.mipmap_filter = Some(filter);
        self
    }

    pub fn build(self) -> Result<Sampler, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_sampler(desc)
    }
}

pub struct BindGroupBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: BindGroupDescriptor<'a>,
}

impl<'a> BindGroupBuilder<'a> {
    fn new(gfx: &'a mut Gfx) -> Self {
        let desc = Default::default();
        Self { gfx, desc }
    }

    pub fn with_texture(mut self, texture: TextureBinding<'a>) -> Self {
        self.desc.entry.push(BindGroupEntry::Texture(texture));
        self
    }

    pub fn with_uniform(mut self, uniform: UniformBinding<'a>) -> Self {
        self.desc.entry.push(BindGroupEntry::Uniform(uniform));
        self
    }

    pub fn build(self) -> Result<BindGroup, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_bind_group(desc)
    }
}

pub struct BufferWriteBuilder<'a> {
    gfx: &'a mut Gfx,
    buffer: &'a Buffer,
    offset: u64,
    data: Option<&'a [u8]>,
}

impl<'a> BufferWriteBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx, buffer: &'a Buffer) -> Self {
        Self {
            gfx,
            buffer,
            offset: 0,
            data: None,
        }
    }

    pub fn with_data<D: bytemuck::Pod>(mut self, data: &'a [D]) -> Self {
        self.data = Some(bytemuck::cast_slice(data));
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }

    pub fn build(mut self) -> Result<(), String> {
        let Self {
            gfx,
            buffer,
            offset,
            data,
        } = self;

        if buffer.is_static() {
            return Err("Cannot write data to a Static Buffer".to_string());
        }

        gfx.raw.write_buffer(buffer, offset, data.unwrap_or(&[]))
    }
}
