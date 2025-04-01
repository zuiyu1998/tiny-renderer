use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(BindGroupLayoutId);

pub trait BindGroupLayoutTrait: ErasedBindGroupLayoutTrait + Clone {
    fn clone_value(&self) -> Box<dyn ErasedBindGroupLayoutTrait> {
        Box::new(self.clone())
    }
}

pub trait ErasedBindGroupLayoutTrait: 'static + Downcast + Debug + Sync + Send {
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

define_gfx_type!(
    BindGroupLayout,
    BindGroupLayoutId,
    BindGroupLayoutTrait,
    ErasedBindGroupLayoutTrait
);

impl Clone for BindGroupLayout {
    fn clone(&self) -> Self {
        BindGroupLayout {
            id: self.id,
            value: self.value.clone_value(),
        }
    }
}


impl BindGroupLayout {
    pub fn id(&self) -> BindGroupLayoutId {
        self.id
    }
}