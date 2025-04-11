use crate::{define_atomic_id, define_gfx_type, error::RendererError};
use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::frame_graph::RenderContext;

use super::color_attachment::ColorAttachmentInfo;

define_atomic_id!(RenderPassId);

#[derive(Default, Clone)]
pub struct RenderPassDescriptor {
    pub color_attachments: Vec<ColorAttachmentInfo>,
}

impl RenderPassDescriptor {}

pub trait RenderPassTrait: 'static {
    fn do_init(&mut self, render_context: &RenderContext) -> Result<(), RendererError>;
}

pub trait ErasedRenderPassTrait: 'static + Downcast {
    fn do_init(&mut self, render_context: &RenderContext) -> Result<(), RendererError>;
}

impl<T: RenderPassTrait> ErasedRenderPassTrait for T {
    fn do_init(&mut self, render_context: &RenderContext) -> Result<(), RendererError> {
        <T as RenderPassTrait>::do_init(self, render_context)
    }
}

define_gfx_type!(
    RenderPass,
    RenderPassId,
    RenderPassTrait,
    ErasedRenderPassTrait
);

impl RenderPass {
    pub fn do_init(&mut self, render_context: &RenderContext) -> Result<(), RendererError> {
        self.value.do_init(render_context)
    }
}
