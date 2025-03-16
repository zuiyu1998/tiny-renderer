use std::collections::HashMap;

use crate::{
    frame_graph::{AnyFGResource, FGResource, ImportedResource, Resource},
    gfx_base::{
        device::Device,
        handle::{RawTypeHandle, TypeHandle},
        transient_resource_cache::TransientResourceCache,
    },
};

///用于渲染的资源表
#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<RawTypeHandle, AnyFGResource>,
}

impl ResourceTable {
    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&ResourceType> {
        self.resources
            .get(&handle.raw_handle())
            .and_then(|any| Some(ResourceType::borrow_resource(any)))
    }

    pub fn get_resouce_mut<ResourceType: FGResource>(
        &mut self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&mut ResourceType> {
        self.resources
            .get_mut(&handle.raw_handle())
            .and_then(|any| Some(ResourceType::borrow_resource_mut(any)))
    }

    pub fn release_resources(
        &mut self,
        resource: &Resource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let handle = resource.get_info().handle.raw_handle();
        let resource = if let Some(resource) = resource.get_imported_resource() {
            let resource = match resource {
                ImportedResource::SwapChain(resource) => {
                    AnyFGResource::ImportedSwapChain(resource.clone())
                }
            };
            resource
        } else {
            let desc = resource.get_desc();

            let resource = transient_resource_cache
                .get_resource(&desc)
                .unwrap_or_else(|| device.create(desc));
            resource
        };

        self.resources.insert(handle, resource);
    }
}
