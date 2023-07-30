/// Configuration to use with the GFX system
#[derive(Default, Debug, Copy, Clone)]
pub struct GfxAttributes {
    /// Use of the integrated gpu if possible
    pub integrated_gpu: bool,
    /// GPU limits compatible with webgl2, D3D11 and GLES-3.0
    pub compatible_mode: bool,
    /// Use VSync mode if possible
    pub vsync: bool,
    // TODO wgpu backends?
}
