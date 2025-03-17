use std::fmt::Debug;

use downcast::{Any, downcast};

#[derive(Debug)]
pub struct TextureView(Box<dyn TextureViewTrait>);

pub trait TextureViewTrait: 'static + Any + Debug {}

downcast!(dyn TextureViewTrait);

impl TextureView {
    pub fn new<T: TextureViewTrait>(view: T) -> Self {
        TextureView(Box::new(view))
    }

    pub fn downcast<T: TextureViewTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.0.downcast::<T>().ok();
        value
    }

    pub fn downcast_ref<T: TextureViewTrait>(&self) -> Option<&T> {
        let value: Option<&T> = self.0.downcast_ref::<T>().ok();
        value
    }
}
