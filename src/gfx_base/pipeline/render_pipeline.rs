use std::fmt::Debug;

use downcast::Any;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(RenderPipelineId);

pub trait RenderPipelineTrait: 'static + Any + Debug + Sync + Send {}

define_gfx_type!(RenderPipeline, RenderPipelineId, RenderPipelineTrait);
