use crate::{BufferUsage, GKBuffer};
use wgpu::Buffer as RawBuffer;

pub struct Buffer {
    pub(crate) raw: RawBuffer,
    pub(crate) usage: BufferUsage,
}

impl GKBuffer for Buffer {
    fn usage(&self) -> BufferUsage {
        self.usage
    }
}
