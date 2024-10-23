use std::{mem::swap, ops::Range};

use wgpu::{CommandEncoder, RenderPass, RenderPipeline};

use super::{
    AnyRenderResource, AnyRenderResourceDescriptor, GpuSrv, GraphResourceCreateInfo, Ref,
    RenderResource, TransientResourceCache, VirtualResourceHandle,
};
use crate::{
    error::{Kind, Result},
    render_backend::RenderBackend,
    renderer::resource::{Buffer, SwapchainImages},
};

//根据资源节点信息生成的资源
pub enum RegistryResource {
    Created(GraphResourceCreateInfo),
    Resource(AnyRenderResource),
}

impl RegistryResource {
    pub fn request(&mut self, cache: &mut TransientResourceCache) {
        match self {
            RegistryResource::Created(GraphResourceCreateInfo { desciptor }) => match desciptor {
                AnyRenderResourceDescriptor::Buffer(buffer) => {
                    let buffer = cache.request_buffer(buffer);
                    *self = RegistryResource::Resource(AnyRenderResource::OwnedBuffer(buffer));
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn release(&mut self, cache: &mut TransientResourceCache) {
        let mut registry_resource = RegistryResource::Resource(AnyRenderResource::Release);
        swap(self, &mut registry_resource);

        match registry_resource {
            RegistryResource::Resource(resource) => match resource {
                AnyRenderResource::SwapchainImage(image) => {
                    image.release(cache);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub struct TrackedRenderPass<'a> {
    pub render_pass: RenderPass<'a>,
    pub render_context: &'a RenderContext,
}

impl<'a> TrackedRenderPass<'a> {
    pub fn set_vertex_buffer(&mut self, slot: u32, handle: &Ref<Buffer, GpuSrv>) {
        let buffer: &Buffer = self
            .render_context
            .get_render_resource(&handle.handle)
            .unwrap();

        self.render_pass
            .set_vertex_buffer(slot, buffer.render_buffer.slice(0..));
    }

    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.render_pass.set_pipeline(pipeline);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw(vertices, instances);
    }
}

pub struct RenderApi {
    pub render_context: RenderContext,
    pub encoder: CommandEncoder,
}

impl RenderApi {
    pub fn begin_render_pass<'encoder>(
        &'encoder mut self,
        desc: &wgpu::RenderPassDescriptor<'_>,
    ) -> TrackedRenderPass<'encoder> {
        let render_pass = self.encoder.begin_render_pass(desc);

        TrackedRenderPass {
            render_pass,
            render_context: &self.render_context,
        }
    }
}

pub struct RenderContext {
    pub resources: Vec<RegistryResource>,
    pub backend: RenderBackend,
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
