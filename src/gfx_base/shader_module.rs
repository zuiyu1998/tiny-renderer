use std::{borrow::Cow, fmt::Debug};

use downcast::Any;

use crate::{define_atomic_id, define_gfx_type};

use super::shader::ShaderDefinition;

define_atomic_id!(ShaderModuleId);

pub trait ShaderModuleTrait: 'static + Any + Debug + Sync + Send {}

define_gfx_type!(ShaderModule, ShaderModuleId, ShaderModuleTrait);

pub struct ShaderModuleDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub source: ShaderDefinition,
}
