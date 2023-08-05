use crate::attrs::GfxAttributes;
use crate::buffer::{BufferDescriptor, GKBuffer, VertexLayout};
use crate::pipeline::{GKRenderPipeline, RenderPipelineDescriptor};
use crate::renderer::Renderer;
use crate::texture::{GKTexture, TextureDescriptor};
use crate::TextureData;
use gk_app::window::{GKWindow, GKWindowId};

pub trait GKDevice<RP: GKRenderPipeline, B: GKBuffer, T: GKTexture> {
    fn new(attrs: GfxAttributes) -> Result<Self, String>
    where
        Self: Sized;
    fn init_surface<W: GKWindow>(&mut self, win: &W) -> Result<(), String>;
    fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<RP, String>;
    fn create_buffer(&mut self, desc: BufferDescriptor) -> Result<B, String>;
    fn create_texture(
        &mut self,
        desc: TextureDescriptor,
        data: Option<TextureData>,
    ) -> Result<T, String>;
    fn resize(&mut self, id: GKWindowId, width: u32, height: u32);
    fn render(&mut self, window: GKWindowId, renderer: &Renderer) -> Result<(), String>;
}
