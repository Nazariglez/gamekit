use crate::renderer::Renderer;
use crate::{Device, Pipeline};
use crate::{GKDevice, RenderPipelineDescriptor};
use gk_app::window::{GKWindow, GKWindowId};
use gk_app::Plugin;

pub struct Gfx {
    pub(crate) raw: Device,
}

impl Plugin for Gfx {}

impl Gfx {
    pub fn new() -> Result<Self, String> {
        let raw = Device::new()?;
        Ok(Self { raw })
    }

    pub fn init_context<W: GKWindow>(&mut self, win: &W) -> Result<(), String> {
        self.raw.init_context(win)
    }

    pub fn create_render_pipeline<'a>(&'a mut self, shader: &'a str) -> RenderPipelineBuilder {
        RenderPipelineBuilder::new(self, shader)
    }

    pub fn resize(&mut self, id: GKWindowId, width: u32, height: u32) {
        self.raw.resize(id, width, height);
    }

    pub fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String> {
        self.raw.render(window, renderer)
    }
}

pub struct RenderPipelineBuilder<'a> {
    desc: RenderPipelineDescriptor<'a>,
    gfx: &'a mut Gfx,
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

    pub fn build(self) -> Result<Pipeline, String> {
        let Self { desc, gfx } = self;
        gfx.raw.create_render_pipeline(desc)
    }
}
