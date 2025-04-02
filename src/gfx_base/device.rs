use crate::{define_atomic_id, define_gfx_type};
use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::frame_graph::{AnyFGResource, AnyFGResourceDescriptor, SwapChain, SwapChainDescriptor};

use super::{
    buffer::{Buffer, BufferDescriptor, BufferInitDescriptor},
    command_buffer::CommandBuffer,
    pipeline::{RenderPipeline, RenderPipelineDescriptorState},
    pipeline_layout::{PipelineLayout, PipelineLayoutDescriptor},
    render_pass::{RenderPass, RenderPassDescriptor},
    shader_module::{ShaderModule, ShaderModuleDescriptor},
};

define_atomic_id!(DeviceId);

pub trait DeviceTrait: 'static + Sync + Send + Debug {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline;

    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule;

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout;

    fn create_buffer(&self, desc: BufferDescriptor) -> Buffer;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);

    fn create_buffer_init(&self, desc: BufferInitDescriptor) -> Buffer;
}

pub trait ErasedDeviceTrait: 'static + Sync + Send + Debug + Downcast {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline;

    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule;

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);

    fn create_buffer(&self, desc: BufferDescriptor) -> Buffer;

    fn create_buffer_init(&self, desc: BufferInitDescriptor) -> Buffer;
}

impl<T: DeviceTrait> ErasedDeviceTrait for T {
    fn create_buffer(&self, desc: BufferDescriptor) -> Buffer {
        <T as DeviceTrait>::create_buffer(&self, desc)
    }

    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain {
        <T as DeviceTrait>::create_swap_chain(&self, desc)
    }

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        <T as DeviceTrait>::create_render_pass(&self, desc)
    }

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline {
        <T as DeviceTrait>::create_render_pipeline(&self, desc)
    }

    fn create_command_buffer(&self) -> CommandBuffer {
        <T as DeviceTrait>::create_command_buffer(&self)
    }

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
        <T as DeviceTrait>::create_shader_module(&self, desc)
    }

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout {
        <T as DeviceTrait>::create_pipeline_layout(&self, desc)
    }

    fn create_buffer_init(&self, desc: BufferInitDescriptor) -> Buffer {
        <T as DeviceTrait>::create_buffer_init(&self, desc)
    }

    fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        <T as DeviceTrait>::submit(&self, command_buffers)
    }
}

define_gfx_type!(Device, DeviceId, DeviceTrait, ErasedDeviceTrait);

impl Device {
    pub fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
        match desc {
            AnyFGResourceDescriptor::SwapChain(desc) => {
                AnyFGResource::OwnedSwapChain(self.create_swap_chain(desc))
            }
            AnyFGResourceDescriptor::Buffer(desc) => {
                AnyFGResource::OwnedBuffer(self.create_buffer(desc))
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain {
        self.value.create_swap_chain(desc)
    }

    pub fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        self.value.create_render_pass(desc)
    }

    pub fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        self.value.submit(command_buffers);
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

    pub fn create_buffer(&self, desc: BufferDescriptor) -> Buffer {
        self.value.create_buffer(desc)
    }

    pub fn create_buffer_init(&self, desc: BufferInitDescriptor) -> Buffer {
        self.value.create_buffer_init(desc)
    }
}
