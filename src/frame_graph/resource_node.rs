use std::marker::PhantomData;

use crate::gfx_base::handle::TypeHandle;

use super::{PassNode, Resource};

///用于控制资源是否可写
pub struct ResourceRef<ResourceType, ViewType> {
    handle: ResourceNodeHandle<ResourceType>,
    _marker: PhantomData<ViewType>,
}

impl<ResourceType, ViewType> ResourceRef<ResourceType, ViewType> {
    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.handle.resource_node_handle.clone()
    }

    pub fn resource_handle(&self) -> TypeHandle<Resource> {
        self.handle.resource_handle.clone()
    }

    pub fn new(handle: ResourceNodeHandle<ResourceType>) -> Self {
        Self {
            handle,
            _marker: PhantomData,
        }
    }
}

pub trait GpuViewType {
    const IS_WRITABLE: bool;
}

pub struct GpuRead;

impl GpuViewType for GpuRead {
    const IS_WRITABLE: bool = false;
}

pub struct GpuWrite;

impl GpuViewType for GpuWrite {
    const IS_WRITABLE: bool = true;
}

#[derive(Clone, Debug)]
pub struct RawResourceNodeHandle {
    resource_node_handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<Resource>,
}

impl<ResourceType> From<RawResourceNodeHandle> for ResourceNodeHandle<ResourceType> {
    fn from(value: RawResourceNodeHandle) -> Self {
        ResourceNodeHandle {
            resource_node_handle: value.resource_node_handle.clone(),
            resource_handle: value.resource_handle.clone(),
            _marker: PhantomData,
        }
    }
}

impl RawResourceNodeHandle {
    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.resource_node_handle.clone()
    }

    pub fn resource_handle(&self) -> TypeHandle<Resource> {
        self.resource_handle.clone()
    }
}

#[derive(Debug)]
pub struct ResourceNodeHandle<ResourceType> {
    resource_node_handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<Resource>,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType> ResourceNodeHandle<ResourceType> {
    pub fn raw(&self) -> RawResourceNodeHandle {
        RawResourceNodeHandle {
            resource_node_handle: self.resource_node_handle(),
            resource_handle: self.resource_handle(),
        }
    }

    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.resource_node_handle.clone()
    }

    pub fn resource_handle(&self) -> TypeHandle<Resource> {
        self.resource_handle.clone()
    }

    pub fn new(
        resource_node_handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<Resource>,
    ) -> Self {
        ResourceNodeHandle {
            resource_node_handle,
            resource_handle,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType> Clone for ResourceNodeHandle<ResourceType> {
    fn clone(&self) -> Self {
        ResourceNodeHandle {
            resource_node_handle: self.resource_node_handle.clone(),
            resource_handle: self.resource_handle.clone(),
            _marker: PhantomData,
        }
    }
}

impl<ResourceType> ResourceNodeHandle<ResourceType> {}

#[derive(Debug)]
pub struct ResourceNode {
    ///资源索引
    pub resource_handle: TypeHandle<Resource>,
    ///自身索引
    pub handle: TypeHandle<ResourceNode>,
    /// 资源版本
    pub version: u32,
    /// 当前写入此资源节点的渲染节点
    pub pass_node_writer_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceNode {
    pub fn new(
        handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<Resource>,
        version: u32,
    ) -> Self {
        ResourceNode {
            handle,
            version,
            pass_node_writer_handle: None,
            resource_handle,
        }
    }
}
