use crate::{define_atomic_id, define_gfx_type};
use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::frame_graph::{AnyResource, AnyResourceDescriptor};

use super::{
    BindGroup, BindGroupInfo, BindGroupLayout, BindGroupLayoutInfo, PipelineLayout,
    PipelineLayoutDescriptor, RenderPass, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptorState, Sample, SampleInfo, ShaderModule, ShaderModuleDescriptor,
    Texture, TextureInfo,
    buffer::{Buffer, BufferInfo, BufferInitInfo},
    command_buffer::CommandBuffer,
};

define_atomic_id!(DeviceId);

pub trait DeviceTrait: 'static + Sync + Send + Debug {
    fn create_bind_group_layout(&self, desc: BindGroupLayoutInfo) -> BindGroupLayout;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline;

    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule;

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout;

    fn create_buffer(&self, desc: BufferInfo) -> Buffer;

    fn create_texture(&self, desc: TextureInfo) -> Texture;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);

    fn create_buffer_init(&self, desc: BufferInitInfo) -> Buffer;

    fn create_bind_group(&self, desc: BindGroupInfo) -> BindGroup;

    fn create_sampler(&self, desc: SampleInfo) -> Sample;
}

pub trait ErasedDeviceTrait: 'static + Sync + Send + Debug + Downcast {
    fn create_sampler(&self, desc: SampleInfo) -> Sample;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline;

    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule;

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);

    fn create_buffer(&self, desc: BufferInfo) -> Buffer;

    fn create_texture(&self, desc: TextureInfo) -> Texture;

    fn create_buffer_init(&self, desc: BufferInitInfo) -> Buffer;

    fn create_bind_group_layout(&self, desc: BindGroupLayoutInfo) -> BindGroupLayout;

    fn create_bind_group(&self, desc: BindGroupInfo) -> BindGroup;
}

impl<T: DeviceTrait> ErasedDeviceTrait for T {
    fn create_sampler(&self, desc: SampleInfo) -> Sample {
        <T as DeviceTrait>::create_sampler(self, desc)
    }

    fn create_bind_group(&self, desc: BindGroupInfo) -> BindGroup {
        <T as DeviceTrait>::create_bind_group(self, desc)
    }

    fn create_bind_group_layout(&self, desc: BindGroupLayoutInfo) -> BindGroupLayout {
        <T as DeviceTrait>::create_bind_group_layout(self, desc)
    }

    fn create_buffer(&self, desc: BufferInfo) -> Buffer {
        <T as DeviceTrait>::create_buffer(self, desc)
    }

    fn create_texture(&self, desc: TextureInfo) -> Texture {
        <T as DeviceTrait>::create_texture(self, desc)
    }

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        <T as DeviceTrait>::create_render_pass(self, desc)
    }

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline {
        <T as DeviceTrait>::create_render_pipeline(self, desc)
    }

    fn create_command_buffer(&self) -> CommandBuffer {
        <T as DeviceTrait>::create_command_buffer(self)
    }

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
        <T as DeviceTrait>::create_shader_module(self, desc)
    }

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout {
        <T as DeviceTrait>::create_pipeline_layout(self, desc)
    }

    fn create_buffer_init(&self, desc: BufferInitInfo) -> Buffer {
        <T as DeviceTrait>::create_buffer_init(self, desc)
    }

    fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        <T as DeviceTrait>::submit(self, command_buffers)
    }
}

define_gfx_type!(Device, DeviceId, DeviceTrait, ErasedDeviceTrait);

impl Device {
    pub fn create(&self, desc: AnyResourceDescriptor) -> AnyResource {
        match desc {
            AnyResourceDescriptor::Buffer(desc) => {
                AnyResource::OwnedBuffer(self.create_buffer(desc))
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        self.value.create_render_pass(desc)
    }

    pub fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        self.value.submit(command_buffers);
    }

    pub fn create_bind_group(&self, desc: BindGroupInfo) -> BindGroup {
        self.value.create_bind_group(desc)
    }

    pub fn create_render_pipeline(&self, state: RenderPipelineDescriptorState) -> RenderPipeline {
        self.value.create_render_pipeline(state)
    }

    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.value.create_command_buffer()
    }

    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
        self.value.create_shader_module(desc)
    }

    pub fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout {
        self.value.create_pipeline_layout(desc)
    }

    pub fn create_buffer(&self, desc: BufferInfo) -> Buffer {
        self.value.create_buffer(desc)
    }

    pub fn create_texture(&self, desc: TextureInfo) -> Texture {
        self.value.create_texture(desc)
    }

    pub fn create_buffer_init(&self, desc: BufferInitInfo) -> Buffer {
        self.value.create_buffer_init(desc)
    }

    pub fn create_bind_group_layout(&self, desc: BindGroupLayoutInfo) -> BindGroupLayout {
        self.value.create_bind_group_layout(desc)
    }

    pub fn create_sampler(&self, desc: SampleInfo) -> Sample {
        self.value.create_sampler(desc)
    }
}
