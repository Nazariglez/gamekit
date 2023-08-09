use crate::{BufferUsage, GKBuffer};
use wgpu::Buffer as RawBuffer;

pub struct Buffer {
    pub(crate) raw: RawBuffer,
    pub(crate) usage: BufferUsage,
    pub(crate) write: bool,
}

impl GKBuffer for Buffer {
    fn usage(&self) -> BufferUsage {
        self.usage
    }

    fn is_writable(&self) -> bool {
        self.write
    }
}
