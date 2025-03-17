use downcast::{Any, downcast};
use std::fmt::Debug;

use crate::frame_graph::{
    AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor,
};

pub trait TextureTrait: 'static + Debug + Any + Debug {}

#[derive(Debug)]
pub struct Texture(Box<dyn TextureTrait>);

downcast!(dyn TextureTrait);

impl Texture {
    pub fn new<T: TextureTrait>(view: T) -> Self {
        Texture(Box::new(view))
    }

    pub fn downcast<T: TextureTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.0.downcast::<T>().ok();
        value
    }

    pub fn downcast_ref<T: TextureTrait>(&self) -> Option<&T> {
        let value: Option<&T> = self.0.downcast_ref::<T>().ok();
        value
    }
}

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
