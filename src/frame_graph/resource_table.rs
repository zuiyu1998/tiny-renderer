use std::collections::HashMap;

use crate::gfx_base::{
    AnyFGResource, FGResource,
    device::Device,
    handle::{RawTypeHandle, TypeHandle},
    transient_resource_cache::TransientResourceCache,
};

use super::Resource;

///用于渲染的资源表
#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<RawTypeHandle, AnyFGResource>,
}

impl ResourceTable {
    pub fn get_resouce<ResourceType: FGResource>(
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

    #[allow(unused)]
    pub fn release_resources(
        &mut self,
        resource: &Resource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        if resource.get_info().imported {
            return;
        }

        let handle = resource.get_info().handle.raw_handle();
        let desc = resource.get_desc();

        let resource = transient_resource_cache
            .get_resource(&desc)
            .unwrap_or_else(|| device.create(desc));

        self.resources.insert(handle, resource);
    }
}
