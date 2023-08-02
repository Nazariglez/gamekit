pub const MAX_VERTEX_BUFFERS: usize = 8;
pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
pub const MAX_SAMPLERS_PER_SHADER_STAGE: usize = 16;
pub const MAX_SAMPLED_TEXTURES_PER_SHADER_STAGE: usize = 16;

#[cfg(all(target_arch = "wasm32", feature = "webgl"))]
pub const MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE: usize = 11;

#[cfg(not(any(target_arch = "wasm32", feature = "webgl")))]
pub const MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE: usize = 12;
