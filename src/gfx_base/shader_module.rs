use std::{borrow::Cow, fmt::Debug};

use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};

use super::shader::ShaderDefinition;

define_atomic_id!(ShaderModuleId);

pub trait ShaderModuleTrait: 'static + Debug + Sync + Send {}
pub trait ErasedShaderModuleTrait: 'static + Debug + Sync + Send + Downcast {}

impl<T: ShaderModuleTrait> ErasedShaderModuleTrait for T {}

define_gfx_type!(
    ShaderModule,
    ShaderModuleId,
    ShaderModuleTrait,
    ErasedShaderModuleTrait
);

pub struct ShaderModuleDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub source: ShaderDefinition,
}
