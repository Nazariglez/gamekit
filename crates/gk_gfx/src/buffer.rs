#[derive(Default, Debug, Clone)]
pub struct VertexLayout {
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

impl VertexLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_step_mode(mut self, step_mode: VertexStepMode) -> Self {
        self.step_mode = step_mode;
        self
    }

    pub fn with_attr(mut self, location: u64, format: VertexFormat) -> Self {
        self.attributes.push(VertexAttribute { location, format });
        self
    }
}

#[derive(Default, Debug, Clone)]
pub enum VertexStepMode {
    #[default]
    Vertex,
    Instance,
}

#[derive(Default, Debug, Clone)]
pub struct VertexAttribute {
    pub location: u64,
    pub format: VertexFormat,
}

#[derive(Default, Debug, Clone)]
pub enum VertexFormat {
    #[default]
    Float32x2,
    Float32x3,
}
