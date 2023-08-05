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

#[derive(Debug, Default, Copy, Clone)]
pub struct TextureData<'a> {
    pub bytes: &'a [u8],
    pub width: u32,
    pub height: u32,
}
