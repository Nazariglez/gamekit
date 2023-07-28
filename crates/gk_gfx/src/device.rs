use raw_window_handle::HasRawWindowHandle;

pub trait GKDevice<RP: GKRenderPipeline> {
    fn init_context<H: HasRawWindowHandle>(&mut self, id: &H) -> Result<(), String>;
    fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<RP, String>;
}

pub trait GKRenderPipeline {}

#[derive(Default, Copy, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
}

#[derive(Default, Copy, Clone)]
pub struct DepthStencil;