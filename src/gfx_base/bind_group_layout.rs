use std::fmt::Debug;

use downcast::Any;

use crate::define_atomic_id;

define_atomic_id!(BindGroupLayoutId);

pub trait BindGroupLayoutTrait: ErasedBindGroupLayoutTrait + Clone {
    fn clone_value(&self) -> Box<dyn ErasedBindGroupLayoutTrait> {
        Box::new(self.clone())
    }
}

pub trait ErasedBindGroupLayoutTrait: 'static + Any + Debug + Sync + Send {
    fn clone_value(&self) -> Box<dyn ErasedBindGroupLayoutTrait>;
}

impl<T> ErasedBindGroupLayoutTrait for T
where
    T: BindGroupLayoutTrait,
{
    fn clone_value(&self) -> Box<dyn ErasedBindGroupLayoutTrait> {
        BindGroupLayoutTrait::clone_value(self)
    }
}

use downcast::downcast;

#[derive(Debug)]
pub struct BindGroupLayout {
    id: BindGroupLayoutId,
    value: Box<dyn ErasedBindGroupLayoutTrait>,
}

impl Clone for BindGroupLayout {
    fn clone(&self) -> Self {
        BindGroupLayout {
            id: self.id,
            value: self.value.clone_value(),
        }
    }
}

downcast!(dyn ErasedBindGroupLayoutTrait);

impl PartialEq for BindGroupLayout {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BindGroupLayout {
    pub fn new<T: BindGroupLayoutTrait>(value: T) -> Self {
        BindGroupLayout {
            value: Box::new(value),
            id: BindGroupLayoutId::new(),
        }
    }

    pub fn id(&self) -> BindGroupLayoutId {
        self.id
    }

    pub fn downcast<T: BindGroupLayoutTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.value.downcast::<T>().ok();
        value
    }

    pub fn downcast_ref<T: BindGroupLayoutTrait>(&self) -> Option<&T> {
        let value: Option<&T> = self.value.downcast_ref::<T>().ok();
        value
    }
}
