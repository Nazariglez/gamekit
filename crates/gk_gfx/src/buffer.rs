pub trait GKBuffer {
    fn usage(&self) -> BufferUsage;
}

#[derive(Default, Debug, Copy, Clone)]
pub struct BufferDescriptor<'a> {
    pub label: Option<&'a str>,
    pub usage: BufferUsage,
    pub content: &'a [u8],
}

#[derive(Default, Debug, Copy, Clone)]
pub enum BufferUsage {
    #[default]
    Vertex,
    Index,
    Uniform,
}

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

#[derive(Default, Debug, Copy, Clone)]
pub enum VertexStepMode {
    #[default]
    Vertex,
    Instance,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct VertexAttribute {
    pub location: u64,
    pub format: VertexFormat,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum VertexFormat {
    #[default]
    UInt8x2,
    UInt8x4,
    Int8x2,
    Int8x4,
    U8x2norm,
    U8x4norm,
    I8x2norm,
    I8x4norm,
    UInt16x2,
    UInt16x4,
    Int16x2,
    Int16x4,
    U16x2norm,
    U16x4norm,
    Int16x2norm,
    Int16x4norm,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    UInt32,
    UInt32x2,
    UInt32x3,
    UInt32x4,
    Int32,
    Int32x2,
    Int32x3,
    Int32x4,
}
