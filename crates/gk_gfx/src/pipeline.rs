use crate::buffer::VertexLayout;

pub trait GKRenderPipeline {}

#[derive(Default, Debug, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
    pub vertex_layout: Option<VertexLayout>,
    pub primitive: Primitive,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct DepthStencil;

#[derive(Default, Debug, Copy, Clone)]
pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    #[default]
    Triangles,
    TriangleStrip,
}
