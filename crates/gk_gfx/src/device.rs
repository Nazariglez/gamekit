use crate::attrs::GfxAttributes;
use crate::renderer::Renderer;
use crate::VertexLayout;
use gk_app::window::{GKWindow, GKWindowId};

pub trait GKDevice<RP: GKRenderPipeline, B: GKBuffer> {
    fn new(attrs: GfxAttributes) -> Result<Self, String>
    where
        Self: Sized;
    fn init_surface<W: GKWindow>(&mut self, win: &W) -> Result<(), String>;
    fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<RP, String>;
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<B, String>;
    fn resize(&mut self, id: GKWindowId, width: u32, height: u32);
    fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String>;
}

pub trait GKRenderPipeline {}
pub trait GKBuffer {
    fn usage(&self) -> BufferUsage;
}

#[derive(Default, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
    pub vertex_layout: Option<VertexLayout>,
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
