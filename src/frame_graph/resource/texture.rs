use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};
use std::fmt::Debug;

use crate::frame_graph::{AnyResource, AnyResourceDescriptor, Resource, ResourceDescriptor};

define_atomic_id!(TextureId);

pub trait TextureTrait: 'static {}

pub trait ErasedTextureTrait: 'static + Downcast {}

impl<T: TextureTrait> ErasedTextureTrait for T {}

define_gfx_type!(Texture, TextureId, TextureTrait, ErasedTextureTrait);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureDescriptor {}

impl ResourceDescriptor for TextureDescriptor {
    type Resource = Texture;
}

impl From<TextureDescriptor> for AnyResourceDescriptor {
    fn from(value: TextureDescriptor) -> Self {
        AnyResourceDescriptor::Texture(value)
    }
}

impl Resource for Texture {
    type Descriptor = TextureDescriptor;

    fn borrow_resource(res: &AnyResource) -> &Self {
        match &res {
            AnyResource::OwnedTexture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}
