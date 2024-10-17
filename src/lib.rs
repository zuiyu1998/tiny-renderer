mod pass;
mod resource;

use std::marker::PhantomData;

use resource::{
    GraphResourceCreateInfo, RenderResource, RenderResourceDescriptor, ResourceNode,
    ResourceNodeHandle, VirtualResource,
};

use pass::PassNode;

///FrameGraph是一个有向无环图，用于渲染数据的整合，cocos的rust版本
pub struct FrameGraph {
    pass_nodes: Vec<PassNode>,
    resource_nodes: Vec<ResourceNode>,
    virtual_resources: Vec<VirtualResource>,
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}

impl FrameGraph {
    ///根据描述创建资源节点
    pub fn create<D: RenderResourceDescriptor>(
        &mut self,
        descriptor: D,
    ) -> ResourceNodeHandle<D::Resource>
    where
        D: TypeEquals<
            Other = <<D as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
        >,
    {
        let index = self.create_raw_resource_node(GraphResourceCreateInfo {
            desciptor: descriptor.clone().into(),
        });

        ResourceNodeHandle {
            index,
            descriptor: TypeEquals::same(descriptor),
            marker: PhantomData,
        }
    }

    pub fn create_raw_resource_node(&mut self, info: GraphResourceCreateInfo) -> u32 {
        let index = self.resource_nodes.len() as u32;

        let virtual_resource = VirtualResource {
            id: index,
            version: 0,
            ..Default::default()
        };
        self.virtual_resources.push(virtual_resource);

        self.resource_nodes.push(ResourceNode::created(info));

        index
    }
}

impl FrameGraph {
    ///compile阶段
    pub fn compile(&mut self) {
        //sort 对插入的pass node进行排序
        self.sort();
    }

    pub fn sort(&mut self) {
        self.pass_nodes.sort();
    }

    ///execute阶段
    pub fn execute(&mut self) {}
}
