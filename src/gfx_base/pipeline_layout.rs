use std::fmt::Debug;

use downcast::Any;
use wgpu::PushConstantRange;

use crate::define_atomic_id;

define_atomic_id!(PipelineLayoutId);

pub trait PipelineLayoutTrait: ErasedPipelineLayoutTrait + Clone {
    fn clone_value(&self) -> Box<dyn ErasedPipelineLayoutTrait> {
        Box::new(self.clone())
    }
}

pub trait ErasedPipelineLayoutTrait: 'static + Any + Debug + Sync + Send {
    fn clone_value(&self) -> Box<dyn ErasedPipelineLayoutTrait>;
}

impl<T> ErasedPipelineLayoutTrait for T
where
    T: PipelineLayoutTrait,
{
    fn clone_value(&self) -> Box<dyn ErasedPipelineLayoutTrait> {
        PipelineLayoutTrait::clone_value(self)
    }
}

use downcast::downcast;

use super::bind_group_layout::BindGroupLayout;

#[derive(Debug)]
pub struct PipelineLayout {
    id: PipelineLayoutId,
    instance: Box<dyn ErasedPipelineLayoutTrait>,
}

pub struct PipelineLayoutDescriptor {
    pub bind_group_layouts: Vec<BindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
}

impl Clone for PipelineLayout {
    fn clone(&self) -> Self {
        PipelineLayout {
            id: self.id,
            instance: self.instance.clone_value(),
        }
    }
}

downcast!(dyn ErasedPipelineLayoutTrait);

impl PartialEq for PipelineLayout {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PipelineLayout {
    pub fn new<T: PipelineLayoutTrait>(instance: T) -> Self {
        PipelineLayout {
            instance: Box::new(instance),
            id: PipelineLayoutId::new(),
        }
    }

    pub fn id(&self) -> PipelineLayoutId {
        self.id
    }

    pub fn downcast<T: PipelineLayoutTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.instance.downcast::<T>().ok();
        value
    }

    pub fn downcast_ref<T: PipelineLayoutTrait>(&self) -> Option<&T> {
        let value: Option<&T> = self.instance.downcast_ref::<T>().ok();
        value
    }
}
