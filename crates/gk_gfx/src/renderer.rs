use crate::color::Color;
use crate::RenderPipeline;
use std::ops::Range;

#[derive(Default)]
pub struct RenderPass<'a> {
    pub(crate) pipeline: Option<&'a RenderPipeline>,
    pub(crate) color: Color,
    pub(crate) vertices: Range<u32>,
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

    pub fn apply_bindings(&mut self) {
        // todo
    }

    pub fn draw(&mut self, vertices: Range<u32>) {
        if let Some(rp) = self.passes.last_mut() {
            rp.vertices = vertices;
        }
    }
}
