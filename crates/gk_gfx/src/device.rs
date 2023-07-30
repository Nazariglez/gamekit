use crate::attrs::GfxAttributes;
use crate::renderer::Renderer;
use gk_app::window::{GKWindow, GKWindowId};

pub trait GKDevice<RP: GKRenderPipeline, B: GKBuffer> {
    fn new(attrs: GfxAttributes) -> Result<Self, String>
    where
        Self: Sized;
    fn init_context<W: GKWindow>(&mut self, win: &W) -> Result<(), String>;
    fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<RP, String>;
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<B, String>;
    fn resize(&mut self, id: GKWindowId, width: u32, height: u32);
    fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String>;
}

pub trait GKRenderPipeline {}
pub trait GKBuffer {}

#[derive(Default, Copy, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
}

#[derive(Default, Copy, Clone)]
pub struct BufferDescriptor<'a> {
    pub label: Option<&'a str>,
    pub usage: BufferUsage,
    pub content: &'a [u8],
}

#[derive(Default, Copy, Clone)]
pub enum BufferUsage {
    #[default]
    Vertex,
    Index,
    Uniform,
}

#[derive(Default, Copy, Clone)]
pub struct DepthStencil;