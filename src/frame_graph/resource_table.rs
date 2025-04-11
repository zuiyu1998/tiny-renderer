use std::collections::HashMap;

use crate::{
    frame_graph::{AnyResource, ImportedVirtualResource, Resource, VirtualResource},
    gfx_base::{
        device::Device,
        handle::{RawTypeHandle, TypeHandle},
    },
};

use super::{ResourceState, TransientResourceCache};

///用于渲染的资源表
#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<RawTypeHandle, AnyResource>,
}

impl ResourceTable {
    pub fn get_resource<ResourceType: Resource>(
        &self,
        handle: &TypeHandle<VirtualResource>,
    ) -> Option<&ResourceType> {
        self.resources
            .get(&handle.raw_handle())
            .map(|any| ResourceType::borrow_resource(any))
    }

    pub fn request_resources(
        &mut self,
        resource: &VirtualResource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let handle = resource.info.handle.raw_handle();
        let resource = match &resource.state {
            ResourceState::Imported(state) => match &state.resource {
                ImportedVirtualResource::Texture(resource) => {
                    AnyResource::ImportedTexture(resource.clone())
                }
                ImportedVirtualResource::Buffer(resource) => {
                    AnyResource::ImportedBuffer(resource.clone())
                }
            },
            ResourceState::Setup(desc) => {
                let desc = desc.clone();
                transient_resource_cache
                    .get_resource(&desc)
                    .unwrap_or_else(|| device.create(desc))
            }
        };

        self.resources.insert(handle, resource);
    }

    pub fn release_resources(self, _transient_resource_cache: &mut TransientResourceCache) {
        for resource in self.resources.into_values() {
            match resource {
                AnyResource::OwnedSwapChain(swap_chain) => {
                    swap_chain.present();
                }

                // AnyResource::OwnedTexture(texture) => {
                //     transient_resource_cache.insert_resource(
                //         AnyResourceDescriptor::Texture(texture.get_desc().clone()),
                //         AnyResource::OwnedTexture(texture),
                //     );
                // }
                _ => {}
            }
        }
    }
}
