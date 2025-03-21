use crate::frame_graph::{AnyFGResource, AnyFGResourceDescriptor, SwapChain, SwapChainDescriptor};

use super::{
    command_buffer::CommandBuffer,
    pipeline::{RenderPipeline, RenderPipelineDescriptor},
    render_pass::{RenderPass, RenderPassDescriptor},
};

pub trait DeviceTrait: 'static + Sync + Send {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;

    fn create_render_pipeline(&self, desc: RenderPipelineDescriptor) -> RenderPipeline;

    fn create_command_buffer(&self) -> CommandBuffer;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);
}

pub struct Device(Box<dyn DeviceTrait>);

impl Device {
    pub fn new<T: DeviceTrait>(device: T) -> Self {
        Device(Box::new(device))
    }

    pub fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
        match desc {
            AnyFGResourceDescriptor::SwapChain(_desc) => {
                unimplemented!()
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

    pub fn create_render_pipeline(&self, desc: RenderPipelineDescriptor) -> RenderPipeline {
        self.0.create_render_pipeline(desc)
    }

    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.0.create_command_buffer()
    }
}
