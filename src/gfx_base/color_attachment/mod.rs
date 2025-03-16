use wgpu::TextureView;

use crate::frame_graph::{Resource, SwapChain};

use super::{handle::TypeHandle, render_context::RenderContext};

#[derive(Debug, Clone)]
pub enum ColorAttachmentView {
    Uninitialization(TypeHandle<Resource>),
    Initialization(TextureView),
}

impl ColorAttachmentView {
    pub fn get_texture_view(&self) -> &TextureView {
        match self {
            ColorAttachmentView::Uninitialization(_) => {
                unimplemented!()
            }
            ColorAttachmentView::Initialization(view) => view,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorAttachment {
    pub view: ColorAttachmentView,
}

impl ColorAttachment {
    pub fn initialization(&mut self, resource_context: &mut RenderContext) {
        let handle = match &self.view {
            ColorAttachmentView::Uninitialization(handle) => handle.clone(),
            ColorAttachmentView::Initialization(_) => {
                return;
            }
        };

        let swap_chain = resource_context.get_resource::<SwapChain>(&handle).unwrap();

        let view = swap_chain.get_texture_view();

        self.view = ColorAttachmentView::Initialization(view)
    }
}
