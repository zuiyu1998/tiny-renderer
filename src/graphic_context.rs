use std::sync::Arc;

use crate::{
    gfx_base::{
        color_attachment::{ColorAttachment, ColorAttachmentView},
        device::Device,
    },
    renderer::Renderer,
};

pub struct InitializationGraphicContext {
    renderer: Renderer,
    device: Arc<Device>,
}

impl InitializationGraphicContext {
    fn render(&mut self) {
        self.renderer.prepare_frame(|fg| {
            let mut builder = fg.create_pass_node_builder(0, "final");

            let new_swap_chain = builder.read_from_board("swap_chain").unwrap();

            builder.add_attachment(ColorAttachment {
                view: ColorAttachmentView::Uninitialization(new_swap_chain.resource_handle.clone()),
            });
        });
        self.renderer.draw_frame();
    }
}

pub enum GraphicContext {
    Initialization(InitializationGraphicContext),
    Uninitialization,
}

impl GraphicContext {
    pub fn initialization(&mut self, device: Arc<Device>, renderer: Renderer) {
        *self = GraphicContext::Initialization(InitializationGraphicContext { renderer, device });
    }

    pub fn render(&mut self) {
        if let GraphicContext::Initialization(context) = self {
            context.render()
        }
    }
}
