use std::fmt::Debug;

use downcast_rs::Downcast;
use wgpu::PushConstantRange;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(PipelineLayoutId);

pub trait PipelineLayoutTrait: ErasedPipelineLayoutTrait + Clone {
    fn clone_value(&self) -> Box<dyn ErasedPipelineLayoutTrait> {
        Box::new(self.clone())
    }
}

pub trait ErasedPipelineLayoutTrait: 'static + Downcast + Debug + Sync + Send {
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

use super::bind_group_layout::BindGroupLayout;

define_gfx_type!(
    PipelineLayout,
    PipelineLayoutId,
    PipelineLayoutTrait,
    ErasedPipelineLayoutTrait
);

pub struct PipelineLayoutDescriptor {
    pub bind_group_layouts: Vec<BindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
}

impl Clone for PipelineLayout {
    fn clone(&self) -> Self {
        PipelineLayout {
            id: self.id,
            value: self.value.clone_value(),
        }
    }
}
