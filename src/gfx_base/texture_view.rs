use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::{define_atomic_id, define_gfx_frame_graph_type};

define_atomic_id!(TextureViewId);

pub trait TextureViewTrait: 'static + Debug + Clone {}

pub trait ErasedTextureViewTrait: 'static + Downcast + Debug {
    fn clone_value(&self) -> Box<dyn ErasedTextureViewTrait>;
}

impl<T: TextureViewTrait> ErasedTextureViewTrait for T {
    fn clone_value(&self) -> Box<dyn ErasedTextureViewTrait> {
        Box::new(self.clone())
    }
}

define_gfx_frame_graph_type!(
    TextureView,
    TextureViewId,
    TextureViewTrait,
    ErasedTextureViewTrait,
    TextureViewInfo
);

impl Clone for TextureView {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            value: self.value.clone_value(),
            desc: self.desc.clone()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureViewInfo {}
