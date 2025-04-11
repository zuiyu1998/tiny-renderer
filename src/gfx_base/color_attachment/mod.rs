use crate::frame_graph::{GpuRead, ResourceNodeRef};

use super::texture_view::TextureView;

#[derive(Clone)]
pub enum ColorAttachmentInfo {
    SwapChain(ResourceNodeRef<TextureView, GpuRead>),
}

impl ColorAttachmentInfo {
    pub fn swap_chain(handle: ResourceNodeRef<TextureView, GpuRead>) -> Self {
        ColorAttachmentInfo::SwapChain(handle)
    }
}
