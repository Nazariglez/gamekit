use crate::buffer::{IndexFormat, VertexLayout};
use crate::consts::{MAX_BIND_GROUPS_PER_PIPELINE, MAX_VERTEX_BUFFERS};
use crate::{BindGroup, BindGroupEntry, BlendMode, Color};
use arrayvec::ArrayVec;

pub trait GKRenderPipeline {}

// https://github.com/floooh/sokol/blob/master/sokol_gfx.h#L2213

#[derive(Default, Clone)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub shader: &'a str,
    pub depth_stencil: Option<DepthStencil>,
    pub vertex_layout: ArrayVec<VertexLayout, MAX_VERTEX_BUFFERS>,
    pub primitive: Primitive,
    pub index_format: IndexFormat,
    pub bind_group_layout: ArrayVec<&'a BindGroup, MAX_BIND_GROUPS_PER_PIPELINE>,
    pub blend_mode: Option<BlendMode>,
    pub cull_mode: Option<CullMode>,
    pub vs_entry: Option<&'a str>,
    pub fs_entry: Option<&'a str>,
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

#[derive(Debug, Copy, Clone)]
pub enum CullMode {
    Front,
    Back,
}

/// Clear options to use at the beginning of the frame
#[derive(Default, Debug, Clone, Copy)]
pub struct ClearOptions {
    pub color: Option<Color>,
    pub depth: Option<f32>,
    pub stencil: Option<u32>,
}
