use crate::consts::{MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE, MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE};
use crate::{Sampler, Texture};
use arrayvec::ArrayVec;

pub trait GKBindGroup {}

pub const MAX_BINDING_ENTRIES: usize =
    MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE + MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE;

#[derive(Default, Clone)]
pub struct BindGroupDescriptor<'a> {
    pub label: Option<&'a str>,
    pub entry: ArrayVec<BindGroupEntry<'a>, MAX_BINDING_ENTRIES>,
}

#[derive(Copy, Clone)]
pub enum BindGroupEntry<'a> {
    Texture(TextureBinding<'a>),
    Uniform,
}

#[derive(Default, Copy, Clone)]
pub struct TextureBinding<'a> {
    pub texture: Option<(u32, &'a Texture)>,
    pub sampler: Option<(u32, &'a Sampler)>,
    pub visible_fragment: bool,
    pub visible_vertex: bool,
    pub visible_compute: bool,
}

impl<'a> TextureBinding<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_texture(mut self, location: u32, texture: &'a Texture) -> Self {
        self.texture = Some((location, texture));
        self
    }

    pub fn with_sampler(mut self, location: u32, sampler: &'a Sampler) -> Self {
        self.sampler = Some((location, sampler));
        self
    }

    pub fn with_fragment_visibility(mut self, visible: bool) -> Self {
        self.visible_fragment = visible;
        self
    }

    pub fn with_vertex_visibility(mut self, visible: bool) -> Self {
        self.visible_vertex = visible;
        self
    }

    pub fn with_compute_visibility(mut self, visible: bool) -> Self {
        self.visible_compute = visible;
        self
    }
}
