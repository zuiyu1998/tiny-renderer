use crate::frame_graph::{GpuRead, ResourceNodeRef, SwapChain};

#[derive(Clone)]
pub enum ColorAttachment {
    SwapChain(ResourceNodeRef<SwapChain, GpuRead>),
}

impl ColorAttachment {
    pub fn swap_chain(handle: ResourceNodeRef<SwapChain, GpuRead>) -> Self {
        ColorAttachment::SwapChain(handle)
    }
}
