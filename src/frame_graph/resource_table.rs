use std::collections::HashMap;

use crate::{
    AnyFGResource, device::Device, handle::RawTypeHandle,
    transient_resource_cache::TransientResourceCache,
};

use super::Resource;

///用于渲染的资源表
#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<RawTypeHandle, AnyFGResource>,
}

impl ResourceTable {
    #[allow(unused)]
    pub fn release_resources(
        &mut self,
        resource: &Resource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let handle = resource.get_info().handle.raw_handle();
        let desc = resource.get_desc();

        let resource = transient_resource_cache
            .get_resource(&desc)
            .unwrap_or_else(|| device.create(desc));

        self.resources.insert(handle, resource);
    }
}
