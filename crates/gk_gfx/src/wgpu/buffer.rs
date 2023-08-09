use crate::{BufferUsage, GKBuffer};
use wgpu::Buffer as RawBuffer;

pub struct Buffer {
    pub(crate) raw: RawBuffer,
    pub(crate) usage: BufferUsage,
    pub(crate) is_static: bool,
}

impl GKBuffer for Buffer {
    fn usage(&self) -> BufferUsage {
        self.usage
    }

    fn is_static(&self) -> bool {
        self.is_static
    }
}
