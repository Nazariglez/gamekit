use crate::buffer::{IndexFormat, VertexLayout};
use crate::{BindGroup, BindGroupEntry};

pub trait GKRenderPipeline {}

// https://github.com/floooh/sokol/blob/master/sokol_gfx.h#L2213

#[derive(Default, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
    pub vertex_layout: Option<VertexLayout>,
    pub primitive: Primitive,
    pub index_format: IndexFormat,
    pub bind_group_layout: Option<&'a BindGroup>,
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
