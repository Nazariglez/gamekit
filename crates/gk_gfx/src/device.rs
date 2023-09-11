use crate::attrs::GfxAttributes;
use crate::buffer::{BufferDescriptor, GKBuffer};
use crate::pipeline::{GKRenderPipeline, RenderPipelineDescriptor};
use crate::renderer::Renderer;
use crate::texture::{GKSampler, GKTexture, SamplerDescriptor, TextureData, TextureDescriptor};
use crate::{BindGroupDescriptor, GKBindGroup};
use gk_sys::window::{GKWindow, WindowId};

pub trait GKDevice<RP: GKRenderPipeline, B: GKBuffer, T: GKTexture, S: GKSampler, BG: GKBindGroup> {
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
    fn write_buffer(&mut self, buffer: &B, offset: u64, data: &[u8]) -> Result<(), String>;
    fn create_sampler(&mut self, desc: SamplerDescriptor) -> Result<S, String>;
    fn create_bind_group(&mut self, desc: BindGroupDescriptor) -> Result<BG, String>;
    fn resize(&mut self, id: WindowId, width: u32, height: u32) -> Result<(), String>;
    fn render(&mut self, window: WindowId, renderer: &Renderer) -> Result<(), String>;
}
