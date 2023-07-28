use std::ops::Range;
use crate::Pipeline;
use crate::color::Color;

#[derive(Default)]
struct RenderPass<'a> {
    pipeline: Option<&'a Pipeline>,
    color: Color,
    vertices: Range<u32>,
}

#[derive(Default)]
pub struct Renderer<'a> {
    passes: Vec<RenderPass<'a>>,
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

    pub fn apply_pipeline(&mut self, pip: &'a Pipeline) {
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
