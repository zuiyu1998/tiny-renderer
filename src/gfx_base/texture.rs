use std::fmt::Debug;

use crate::gfx_base::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

pub trait TextureTrait: 'static + Debug {}

#[derive(Debug)]
pub struct Texture(Box<dyn TextureTrait>);

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
            AnyFGResource::OwnedTexture(res) => &res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn get_desc(&self) -> &Self::Descriptor {
        todo!()
    }

    fn borrow_resource_mut(res: &mut AnyFGResource) -> &mut Self {
        todo!()
    }
}
