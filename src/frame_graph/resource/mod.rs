mod swap_chain;
mod texture;

use std::sync::Arc;

pub use swap_chain::*;
pub use texture::*;

use super::{AnyFGResourceDescriptor, FGResource, pass_node::PassNode};
use crate::gfx_base::handle::TypeHandle;

pub enum ImportedResource {
    SwapChain(Arc<SwapChain>),
}

pub struct Resource {
    state: ResourceState,
    info: ResourceInfo,
}

impl Resource {
    pub fn get_imported_resource(&self) -> Option<&ImportedResource> {
        match &self.state {
            ResourceState::Created(_) => None,
            ResourceState::Imported(imported) => Some(&imported.resource),
        }
    }

    pub fn new_created<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<Resource>,
        desc: ResourceType::Descriptor,
    ) -> Resource {
        let info = ResourceInfo::new(name, handle);

        Resource {
            state: ResourceState::Created(desc.into()),
            info,
        }
    }

    pub fn new_imported<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<Resource>,
        desc: ResourceType::Descriptor,
        imported_resource: ImportedResource,
    ) -> Resource {
        let info = ResourceInfo::new(name, handle);

        Resource {
            state: ResourceState::Imported(ImportedResourceState {
                desc: desc.into(),
                resource: imported_resource,
            }),
            info,
        }
    }

    pub fn get_info(&self) -> &ResourceInfo {
        &self.info
    }

    pub fn get_info_mut(&mut self) -> &mut ResourceInfo {
        &mut self.info
    }

    pub fn get_desc(&self) -> AnyFGResourceDescriptor {
        match &self.state {
            ResourceState::Created(desc) => desc.clone(),
            ResourceState::Imported(imported) => imported.desc.clone(),
        }
    }
}

///记录资源被使用的必要信息
#[derive(Clone)]
pub struct ResourceInfo {
    ///唯一的资源名称
    pub name: String,
    ///资源索引
    pub handle: TypeHandle<Resource>,
    /// 资源版本
    version: u32,
    ///首次使用此资源的渲染节点
    pub first_pass_node_handle: Option<TypeHandle<PassNode>>,
    ///最后使用此资源的渲染节点
    pub last_pass_node_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceInfo {
    pub fn new(name: &str, handle: TypeHandle<Resource>) -> Self {
        ResourceInfo {
            name: name.to_string(),
            handle,
            version: 0,
            first_pass_node_handle: None,
            last_pass_node_handle: None,
        }
    }
}

impl ResourceInfo {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn new_version(&mut self) {
        self.version += 1
    }

    pub fn update_lifetime(&mut self, handle: TypeHandle<PassNode>) {
        if self.first_pass_node_handle.is_none() {
            self.first_pass_node_handle = Some(handle.clone());
        }

        self.last_pass_node_handle = Some(handle)
    }
}

pub struct ImportedResourceState {
    desc: AnyFGResourceDescriptor,
    resource: ImportedResource,
}

pub enum ResourceState {
    Created(AnyFGResourceDescriptor),
    Imported(ImportedResourceState),
}
