use std::collections::HashMap;

use crate::{
    frame_graph::{AnyFGResource, FGResource, ImportedResource, Resource},
    gfx_base::{
        device::Device,
        handle::{RawTypeHandle, TypeHandle},
        transient_resource_cache::TransientResourceCache,
    },
};

use super::AnyFGResourceDescriptor;

///用于渲染的资源表
#[derive(Default, Debug)]
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
            .map(|any| ResourceType::borrow_resource(any))
    }

    pub fn get_resource_mut<ResourceType: FGResource>(
        &mut self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&mut ResourceType> {
        self.resources
            .get_mut(&handle.raw_handle())
            .map(|any| ResourceType::borrow_resource_mut(any))
    }

    pub fn request_resources(
        &mut self,
        resource: &Resource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let handle = resource.get_info().handle.raw_handle();
        let resource = if let Some(resource) = resource.get_imported_resource() {
            match resource {
                ImportedResource::Texture(resource) => {
                    AnyFGResource::ImportedTexture(resource.clone())
                }
            }
        } else {
            let desc = resource.get_desc();

            transient_resource_cache
                .get_resource(&desc)
                .unwrap_or_else(|| device.create(desc))
        };

        self.resources.insert(handle, resource);
    }

    pub fn release_resources(self, transient_resource_cache: &mut TransientResourceCache) {
        for resource in self.resources.into_values() {
            match resource {
                AnyFGResource::OwnedSwapChain(swap_chain) => {
                    swap_chain.present();
                }

                AnyFGResource::OwnedTexture(texture) => {
                    transient_resource_cache.insert_resource(
                        AnyFGResourceDescriptor::Texture(texture.get_desc().clone()),
                        AnyFGResource::OwnedTexture(texture),
                    );
                }
                _ => {}
            }
        }
    }
}
