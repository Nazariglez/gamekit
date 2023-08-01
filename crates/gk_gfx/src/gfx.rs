use crate::renderer::Renderer;
use crate::{
    Buffer, BufferDescriptor, BufferUsage, Device, GfxAttributes, Primitive, RenderPipeline,
    VertexLayout,
};
use crate::{GKDevice, RenderPipelineDescriptor};
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;

pub struct Gfx {
    pub(crate) raw: Device,
}

impl Plugin for Gfx {}

impl Gfx {
    pub fn new(attrs: GfxAttributes) -> Result<Self, String> {
        let raw = Device::new(attrs)?;
        Ok(Self { raw })
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

    // TODO IndexFormat!
    pub fn create_index_buffer<'a, D: bytemuck::Pod>(&'a mut self, data: &'a [D]) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Index, data)
    }

    pub fn create_uniform_buffer<'a, D: bytemuck::Pod>(
        &'a mut self,
        data: &'a [D],
    ) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Uniform, data)
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

    pub fn with_primitive(mut self, primitive: Primitive) -> Self {
        self.desc.primitive = primitive;
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

    pub fn build(self) -> Result<Buffer, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_buffer(desc)
    }
}
