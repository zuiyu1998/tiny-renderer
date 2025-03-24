use std::fmt::Debug;

use downcast::Any;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(TextureViewId);
pub trait TextureViewTrait: 'static + Any + Debug {}

define_gfx_type!(TextureView, TextureViewId, TextureViewTrait);
