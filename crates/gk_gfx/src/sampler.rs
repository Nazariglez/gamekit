pub enum SamplerWrap {
    Clamp,
    Repeat,
    MirrorRepeat,
}

// TODO move all of this to texture.rs and call it textureWhatwerver?
pub struct SamplerDescriptor {
   wrap_x: SamplerWrap,
    wrap_y: SamplerWrap,
    wrap_z: SamplerWrap,
    mag_filter: SamplerFilter,
    min_filter: SamplerFilter,
    mipmap_filter: Option<SamplerFilter>
}