mod swap_chain;
mod texture;
mod buffer;

use std::sync::Arc;

pub use swap_chain::*;
pub use texture::*;

use super::{AnyFGResourceDescriptor, FGResource, pass_node::PassNode};
use crate::gfx_base::{buffer::Buffer, handle::TypeHandle};

pub enum ImportedVirtualResource{
    Texture(Arc<Texture>),
    Buffer(Arc<Buffer>),
}

pub struct VirtualResource {
    pub state: ResourceState,
    pub info: ResourceInfo,
}

impl VirtualResource {
    pub fn new_setuped<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<VirtualResource>,
        desc: ResourceType::Descriptor,
    ) -> VirtualResource {
        let info = ResourceInfo::new(name, handle);

        VirtualResource {
            state: ResourceState::Setup(desc.into()),
            info,
        }
    }

    pub fn new_imported<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<VirtualResource>,
        desc: ResourceType::Descriptor,
        imported_resource: ImportedVirtualResource,
    ) -> VirtualResource {
        let info = ResourceInfo::new(name, handle);

        VirtualResource {
            state: ResourceState::Imported(ImportedResourceState {
                desc: desc.into(),
                resource: imported_resource,
            }),
            info,
        }
    }
}

///记录资源被使用的必要信息
#[derive(Clone)]
pub struct ResourceInfo {
    ///唯一的资源名称
    pub name: String,
    ///资源索引
    pub handle: TypeHandle<VirtualResource>,
    /// 资源版本
    version: u32,
    ///首次使用此资源的渲染节点
    pub first_pass_node_handle: Option<TypeHandle<PassNode>>,
    ///最后使用此资源的渲染节点
    pub last_pass_node_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceInfo {
    pub fn new(name: &str, handle: TypeHandle<VirtualResource>) -> Self {
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
            self.first_pass_node_handle = Some(handle);
        }

        self.last_pass_node_handle = Some(handle)
    }
}

pub struct ImportedResourceState {
    pub desc: AnyFGResourceDescriptor,
    pub resource: ImportedVirtualResource,
}

pub enum ResourceState {
    Setup(AnyFGResourceDescriptor),
    Imported(ImportedResourceState),
}
