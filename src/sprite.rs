use gk_gfx::{GKSampler, GKTexture, Sampler, SamplerId, Texture, TextureId};
use gk_math::Vec2;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpriteId {
    texture_id: u64,
    sampler_id: u64,
}

#[derive(Clone)]
pub struct Sprite {
    id: SpriteId,
    texture: Texture,
    sampler: Sampler,
    size: Vec2,
}

impl Sprite {
    pub fn new(texture: Texture, sampler: Sampler) -> Self {
        let id = SpriteId {
            texture_id: texture.id().into(),
            sampler_id: sampler.id().into(),
        };
        let size = texture.size().into();
        Self {
            id,
            texture,
            sampler,
            size,
        }
    }

    fn id(&self) -> SpriteId {
        self.id
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn width(&self) -> f32 {
        self.size.x
    }

    fn height(&self) -> f32 {
        self.size.y
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
