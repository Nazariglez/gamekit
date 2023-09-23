use crate::attrs::GfxAttributes;
use crate::buffer::{BufferDescriptor, GKBuffer};
use crate::frame::GKDrawFrame;
use crate::pipeline::{GKRenderPipeline, RenderPipelineDescriptor};
use crate::renderer::Renderer;
use crate::texture::{GKSampler, GKTexture, SamplerDescriptor, TextureData, TextureDescriptor};
use crate::{BindGroupDescriptor, GKBindGroup, GKBindGroupLayoutRef};
use gk_sys::window::{GKWindow, WindowId};

pub trait GKDevice<
    DF: GKDrawFrame,
    RP: GKRenderPipeline,
    B: GKBuffer,
    T: GKTexture,
    S: GKSampler,
    BG: GKBindGroup,
    BGL: GKBindGroupLayoutRef,
>
{
    fn new(attrs: GfxAttributes) -> Result<Self, String>
    where
        Self: Sized;
    fn create_frame(&mut self, window_id: WindowId) -> Result<DF, String>;
    fn present(&mut self, frame: &DF) -> Result<(), String>;
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
    fn size(&self, id: WindowId) -> (u32, u32);
    fn render(&mut self, window: WindowId, renderer: &Renderer) -> Result<(), String>;
}
