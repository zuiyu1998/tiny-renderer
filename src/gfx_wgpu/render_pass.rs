use crate::{
    error::RendererError,
    frame_graph::RenderContext,
    gfx_base::{
        ColorAttachmentInfo,
        render_pass::{RenderPassDescriptor, RenderPassTrait},
        texture_view::TextureView,
    },
};

pub struct WgpuRenderPass {
    desc: RenderPassDescriptor,
    pub texture_views: Option<Vec<TextureView>>,
}

impl WgpuRenderPass {
    pub fn new(desc: RenderPassDescriptor) -> Self {
        WgpuRenderPass {
            desc,
            texture_views: None,
        }
    }
}

impl RenderPassTrait for WgpuRenderPass {
    fn do_init(&mut self, render_context: &RenderContext) -> Result<(), RendererError> {
        let mut texture_views = vec![];

        for color_attachment in self.desc.color_attachments.iter() {
            match color_attachment {
                ColorAttachmentInfo::SwapChain(handle) => {
                    if let Some(resource) = render_context.get_resource(&handle) {
                        texture_views.push(resource.clone());
                    } else {
                        return Err(RendererError::ResourceNotFound {
                            resource_index: handle.resource_handle().index(),
                        });
                    }
                }
            }
        }

        self.texture_views = Some(texture_views);

        Ok(())
    }
}
