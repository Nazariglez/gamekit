use crate::buffer::VertexLayout;

pub trait GKRenderPipeline {}

#[derive(Default, Debug, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
    pub vertex_layout: Option<VertexLayout>,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct DepthStencil;
