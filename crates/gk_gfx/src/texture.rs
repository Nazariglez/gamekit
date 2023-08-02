pub trait GKTexture {}

#[derive(Debug, Default, Copy, Clone)]
pub struct TextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub format: TextureFormat,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureFormat {
    #[default]
    Rgba8UnormSrgb,
}
