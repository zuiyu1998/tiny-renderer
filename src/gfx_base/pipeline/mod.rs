mod pipeline_cache;
mod render_pipeline;
mod shader_cache;

use fyrox_resource::Resource;
pub use pipeline_cache::*;
pub use render_pipeline::*;
pub use shader_cache::*;
use wgpu::{BufferAddress, ColorTargetState, PushConstantRange, VertexAttribute, VertexStepMode};

use std::borrow::Cow;

use super::{
    bind_group_layout::BindGroupLayout,
    pipeline_layout::PipelineLayout,
    shader::{Shader, ShaderDefVal},
    shader_module::ShaderModule,
};

#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexBufferLayout {
    /// The stride, in bytes, between elements of this buffer.
    pub array_stride: BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RenderPipelineDescriptor {
    pub label: Option<Cow<'static, str>>,
    /// The layout of bind groups for this pipeline.
    pub layout: Vec<BindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
    pub vertex: VertexState,
    pub fragment: Option<FragmentState>,
}

pub struct RenderPipelineDescriptorState<'a> {
    pub vertex_module: &'a ShaderModule,
    pub fragment_module: Option<&'a ShaderModule>,
    pub layout: Option<&'a PipelineLayout>,
    pub desc: RenderPipelineDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexState {
    pub shader: Resource<Shader>,
    pub shader_defs: Vec<ShaderDefVal>,
    pub entry_point: Cow<'static, str>,
    pub buffers: Vec<VertexBufferLayout>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FragmentState {
    /// The compiled shader module for this stage.
    pub shader: Resource<Shader>,
    pub shader_defs: Vec<ShaderDefVal>,
    /// The name of the entry point in the compiled shader. There must be a
    /// function with this name in the shader.
    pub entry_point: Cow<'static, str>,
    /// The color state of the render targets.
    pub targets: Vec<Option<ColorTargetState>>,
}
