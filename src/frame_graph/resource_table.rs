use std::collections::HashMap;

use crate::{
    frame_graph::{AnyResource, ImportedVirtualResource, Resource, VirtualResource},
    gfx_base::{device::Device, handle::TypeHandle},
};

use super::{ResourceState, TransientResourceCache};

///用于渲染的资源表
#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<TypeHandle<VirtualResource>, AnyResource>,
}

impl ResourceTable {
    pub fn get_resource<ResourceType: Resource>(
        &self,
        handle: &TypeHandle<VirtualResource>,
    ) -> Option<&ResourceType> {
        self.resources
            .get(&handle)
            .map(|any| ResourceType::borrow_resource(any))
    }

    pub fn request_resource(
        &mut self,
        resource: &VirtualResource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let handle = resource.info.handle.clone();
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

    pub fn release_resource(
        &mut self,
        handle: &TypeHandle<VirtualResource>,
        _transient_resource_cache: &mut TransientResourceCache,
    ) {
        if let Some(resource) = self.resources.remove(handle) {
            match resource {
                _ => {}
            }
        }
    }

    pub fn release_resources(self, _transient_resource_cache: &mut TransientResourceCache) {
        for resource in self.resources.into_values() {
            match resource {
                _ => {}
            }
        }
    }
}
