#[derive(Clone, Debug)]
pub struct AssetLoaded {
    pub name: String,
    pub data: Vec<u8>,
}
