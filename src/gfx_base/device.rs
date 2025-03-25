use std::fmt::Debug;

use crate::frame_graph::{AnyFGResource, AnyFGResourceDescriptor, SwapChain, SwapChainDescriptor};

use super::{
    command_buffer::CommandBuffer,
    pipeline::{RenderPipeline, RenderPipelineDescriptorState},
    pipeline_layout::{PipelineLayout, PipelineLayoutDescriptor},
    render_pass::{RenderPass, RenderPassDescriptor},
    shader_module::{ShaderModule, ShaderModuleDescriptor},
};

pub trait DeviceTrait: 'static + Sync + Send + Debug {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptorState) -> RenderPipeline;

    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule;

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);
}

#[derive(Debug)]
pub struct Device(Box<dyn DeviceTrait>);

impl Device {
    pub fn new<T: DeviceTrait>(device: T) -> Self {
        Device(Box::new(device))
    }

    pub fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
        match desc {
            AnyFGResourceDescriptor::SwapChain(desc) => {
                AnyFGResource::OwnedSwapChain(self.create_swap_chain(desc))
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain {
        self.0.create_swap_chain(desc)
    }

    pub fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        self.0.create_render_pass(desc)
    }

    pub fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        self.0.submit(command_buffers);
    }

    pub fn create_render_pipeline(&self, state: RenderPipelineDescriptorState) -> RenderPipeline {
        self.0.create_render_pipeline(state)
    }

    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.0.create_command_buffer()
    }

    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
        self.0.create_shader_module(desc)
    }

    pub fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout {
        self.0.create_pipeline_layout(desc)
    }
}
