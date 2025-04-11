use crate::frame_graph::{GpuRead, ResourceNodeRef, SwapChain};

#[derive(Clone)]
pub enum ColorAttachmentInfo {
    SwapChain(ResourceNodeRef<SwapChain, GpuRead>),
}

impl ColorAttachmentInfo {
    pub fn swap_chain(handle: ResourceNodeRef<SwapChain, GpuRead>) -> Self {
        ColorAttachmentInfo::SwapChain(handle)
    }
}
