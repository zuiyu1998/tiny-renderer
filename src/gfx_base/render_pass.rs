use std::{fmt::Debug, ops::Range};

use crate::frame_graph::RenderContext;
use downcast::{Any, downcast};

use super::{
    color_attachment::{ColorAttachment, ColorAttachmentView},
    pipeline::RenderPipeline,
};

#[derive(Debug, Default)]
pub struct RenderPassDescriptor {
    pub color_attachments: Vec<ColorAttachment>,
}

impl RenderPassDescriptor {
    pub fn initialization(&mut self, resource_context: &mut RenderContext) {
        for color_attachment in self.color_attachments.iter_mut() {
            let view = match &color_attachment.view {
                ColorAttachmentView::Initialization(_) => {
                    continue;
                }
                ColorAttachmentView::Uninitialization(handle) => {
                    resource_context.get_texture_view_with_swap_chain(handle)
                }
            };

            color_attachment.view = ColorAttachmentView::Initialization(view);
        }
    }
}

pub trait RenderPassTrait: 'static + Debug + Any {
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);
}

pub struct RenderPass(Box<dyn RenderPassTrait>);

downcast!(dyn RenderPassTrait);

impl RenderPass {
    pub fn new<T: RenderPassTrait>(render_pass: T) -> Self {
        RenderPass(Box::new(render_pass))
    }

    pub fn downcast<T: RenderPassTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.0.downcast::<T>().ok();
        value
    }

    pub fn downcast_ref<T: RenderPassTrait>(&self) -> Option<&T> {
        let value: Option<&T> = self.0.downcast_ref::<T>().ok();
        value
    }

    pub fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        self.0.set_render_pipeline(render_pipeline);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.0.draw(vertices, instances);
    }
}
