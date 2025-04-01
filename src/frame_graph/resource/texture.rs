use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};
use std::fmt::Debug;

use crate::frame_graph::{
    AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor,
};

define_atomic_id!(TextureId);

pub trait TextureTrait: 'static + Debug {}

pub trait ErasedTextureTrait: 'static + Downcast + Debug {}

impl<T: TextureTrait> ErasedTextureTrait for T {}

define_gfx_type!(Texture, TextureId, TextureTrait, ErasedTextureTrait);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureDescriptor {}

impl FGResourceDescriptor for TextureDescriptor {
    type Resource = Texture;
}

impl From<TextureDescriptor> for AnyFGResourceDescriptor {
    fn from(value: TextureDescriptor) -> Self {
        AnyFGResourceDescriptor::Texture(value)
    }
}

impl FGResource for Texture {
    type Descriptor = TextureDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match &res {
            AnyFGResource::OwnedTexture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn get_desc(&self) -> &Self::Descriptor {
        todo!()
    }

    fn borrow_resource_mut(_res: &mut AnyFGResource) -> &mut Self {
        todo!()
    }
}
