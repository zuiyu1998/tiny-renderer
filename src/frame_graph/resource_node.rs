use std::marker::PhantomData;

use crate::gfx_base::handle::TypeHandle;

use super::{PassNode, Resource};

pub struct ResourceNodeHandle<ResourceType> {
    handle: TypeHandle<ResourceNode>,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType> ResourceNodeHandle<ResourceType> {
    pub fn handle(&self) -> &TypeHandle<ResourceNode> {
        &self.handle
    }

    pub fn new(handle: TypeHandle<ResourceNode>) -> ResourceNodeHandle<Resource> {
        ResourceNodeHandle {
            handle,
            _marker: PhantomData,
        }
    }
}

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
