use crate::{AnyFGResourceDescriptor, FGResource, handle::TypeHandle};

use super::pass_node::PassNode;

///frame_graph 自动管理的资源
pub trait ResourceTrait: 'static {
    fn get_info(&self) -> &ResourceInfo;
    fn get_info_mut(&mut self) -> &mut ResourceInfo;
    fn get_desc(&self) -> AnyFGResourceDescriptor;
}

pub struct Resource(Box<dyn ResourceTrait>);

impl Resource {
    pub fn new<T: ResourceTrait>(base_resouce: T) -> Resource {
        Resource(Box::new(base_resouce))
    }

    pub fn get_info(&self) -> &ResourceInfo {
        self.0.get_info()
    }

    pub fn get_info_mut(&mut self) -> &mut ResourceInfo {
        self.0.get_info_mut()
    }

    pub fn get_desc(&self) -> AnyFGResourceDescriptor {
        self.0.get_desc()
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

pub struct ResourceEntry<ResourceType: FGResource> {
    desc: ResourceType::Descriptor,
    info: ResourceInfo,
}

impl<ResourceType: FGResource> ResourceEntry<ResourceType> {
    pub fn new(name: &str, handle: TypeHandle<Resource>, desc: ResourceType::Descriptor) -> Self {
        ResourceEntry {
            desc,
            info: ResourceInfo::new(name, handle),
        }
    }
}

impl<ResourceType> ResourceTrait for ResourceEntry<ResourceType>
where
    ResourceType: FGResource,
{
    fn get_info(&self) -> &ResourceInfo {
        &self.info
    }

    fn get_info_mut(&mut self) -> &mut ResourceInfo {
        &mut self.info
    }

    fn get_desc(&self) -> AnyFGResourceDescriptor {
        self.desc.clone().into()
    }
}
