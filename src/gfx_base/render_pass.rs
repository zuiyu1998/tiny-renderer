use std::fmt::Debug;

use super::{
    color_attachment::ColorAttachment, command_buffer::CommandBuffer, render_context::RenderContext,
};

#[derive(Debug, Default)]
pub struct RenderPassDescriptor {
    pub color_attachments: Vec<ColorAttachment>,
}

impl RenderPassDescriptor {
    pub fn initialization(&mut self, resource_context: &mut RenderContext) {
        for color_attachment in self.color_attachments.iter_mut() {
            color_attachment.initialization(resource_context);
        }
    }
}

pub trait RenderPassTrait: 'static + Debug {
    fn finish(&mut self) -> CommandBuffer;
}

pub struct RenderPass(Box<dyn RenderPassTrait>);

impl RenderPass {
    pub fn new<T: RenderPassTrait>(render_pass: T) -> Self {
        RenderPass(Box::new(render_pass))
    }

    pub fn finish(&mut self) -> CommandBuffer {
        self.0.finish()
    }
}
