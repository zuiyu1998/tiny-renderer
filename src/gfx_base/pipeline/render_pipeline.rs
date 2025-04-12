use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(RenderPipelineId);

pub trait RenderPipelineTrait: 'static + Debug + Sync + Send {}

pub trait ErasedRenderPipelineTrait: 'static + Sync + Send + Debug + Downcast {}

impl<T: RenderPipelineTrait> ErasedRenderPipelineTrait for T {}

define_gfx_type!(
    RenderPipeline,
    RenderPipelineId,
    RenderPipelineTrait,
    ErasedRenderPipelineTrait
);
