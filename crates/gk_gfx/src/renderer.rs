use crate::color::Color;
use crate::consts::{
    MAX_BIND_GROUPS_PER_PIPELINE, MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE, MAX_VERTEX_BUFFERS,
};
use crate::{BindGroup, Buffer, RenderPipeline};
use arrayvec::ArrayVec;
use std::ops::Range;

// TODO gfx works with RenderPass, then we have Render2D, and Render3D
// for things like the old notan draw, or a new 3d API

const MAX_BUFFERS: usize = MAX_VERTEX_BUFFERS + MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE + 1;

#[derive(Default)]
pub struct RenderPass<'a> {
    pub(crate) pipeline: Option<&'a RenderPipeline>,
    pub(crate) buffers: ArrayVec<&'a Buffer, MAX_BUFFERS>,
    pub(crate) color: Color,
    pub(crate) vertices: Range<u32>,
    pub(crate) instances: Option<u32>,
    pub(crate) bind_groups: ArrayVec<&'a BindGroup, MAX_BIND_GROUPS_PER_PIPELINE>,
}

#[derive(Default)]
pub struct Renderer<'a> {
    pub(crate) passes: Vec<RenderPass<'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn begin(&mut self, color: Color, width: u32, height: u32) {
        self.passes.push(RenderPass {
            color,
            ..Default::default()
        });
    }

    pub fn apply_pipeline(&mut self, pip: &'a RenderPipeline) {
        if let Some(rp) = self.passes.last_mut() {
            rp.pipeline = Some(pip);
        }
    }

    pub fn apply_buffers(&mut self, buffers: &[&'a Buffer]) {
        if let Some(rp) = self.passes.last_mut() {
            rp.buffers = ArrayVec::new();
            rp.buffers.try_extend_from_slice(buffers).unwrap();
        }
    }

    pub fn apply_bindings(&mut self, groups: &[&'a BindGroup]) {
        if let Some(rp) = self.passes.last_mut() {
            rp.bind_groups = ArrayVec::new();
            rp.bind_groups.try_extend_from_slice(groups).unwrap();
        }
    }

    pub fn draw(&mut self, vertices: Range<u32>) {
        if let Some(rp) = self.passes.last_mut() {
            rp.vertices = vertices;
        }
    }

    pub fn draw_instanced(&mut self, vertices: Range<u32>, instances: u32) {
        if let Some(rp) = self.passes.last_mut() {
            rp.vertices = vertices;
            rp.instances = Some(instances);
        }
    }
}
