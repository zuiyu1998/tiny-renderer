use wgpu::CommandEncoder;

use super::{AnyRenderResource, GraphResourceCreateInfo, RenderResource, VirtualResourceHandle};
use crate::{
    error::{Kind, Result},
    renderer::resource::SwapchainImages,
};

//根据资源节点信息生成的资源
pub enum RegistryResource {
    Created(GraphResourceCreateInfo),
    Resource(AnyRenderResource),
}

pub struct RenderContext {
    pub resources: Vec<RegistryResource>,
    pub encoder: CommandEncoder,
}

impl RenderContext {
    pub fn initialize_swap_images(&mut self, mut swapchain_images: SwapchainImages) {
        for resource in self.resources.iter_mut() {
            match resource {
                RegistryResource::Resource(resource) => match resource {
                    AnyRenderResource::Pending => {
                        if let Some(swapchain_image) = swapchain_images.images.pop_front() {
                            *resource = AnyRenderResource::SwapchainImage(swapchain_image);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn begin_render_pass<'encoder>(
        &'encoder mut self,
        desc: &wgpu::RenderPassDescriptor<'_>,
    ) -> wgpu::RenderPass<'encoder> {
        self.encoder.begin_render_pass(desc)
    }

    pub fn get_render_resource<ResType: RenderResource>(
        &self,
        handle: &VirtualResourceHandle,
    ) -> Result<&ResType> {
        let registry_resource = &self.resources[handle.id as usize];

        match registry_resource {
            RegistryResource::Created(_) => Err(Kind::ResourceUninitialized.into()),

            RegistryResource::Resource(reource) => Ok(ResType::borrow_resource(reource)),
        }
    }
}
