use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(SampleId);

pub trait SampleTrait: 'static + Debug + Sync + Send {}
pub trait ErasedSampleTrait: 'static + Debug + Sync + Send + Downcast {}

impl<T: SampleTrait> ErasedSampleTrait for T {}

define_gfx_type!(Sample, SampleId, SampleTrait, ErasedSampleTrait);

#[derive(Clone)]
pub struct SampleInfo {}
