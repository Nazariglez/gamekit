/// Configuration to use with the GFX system
#[derive(Default, Debug, Copy, Clone)]
pub struct GfxAttributes {
    /// Use VSync mode if possible
    pub vsync: bool,
    // TODO wgpu backends?
}
