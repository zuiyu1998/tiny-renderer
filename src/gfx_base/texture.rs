use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_frame_graph_type};
use std::fmt::Debug;

define_atomic_id!(TextureId);

pub trait TextureTrait: 'static + Clone + Debug {}

pub trait ErasedTextureTrait: 'static + Downcast + Debug {}

impl<T: TextureTrait> ErasedTextureTrait for T {}

define_gfx_frame_graph_type!(
    Texture,
    TextureId,
    TextureTrait,
    ErasedTextureTrait,
    TextureInfo
);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureInfo {}
