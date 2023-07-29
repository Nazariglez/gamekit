use crate::GKBuffer;
use wgpu::Buffer as RawBuffer;

pub struct Buffer {
    pub(crate) raw: RawBuffer,
}

impl GKBuffer for Buffer {}
