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

        self.cull();
    }

    //清除掉不需要使用的资源和pass
    pub fn cull(&mut self) {
        //初始化pass_node的引用，和资源的reader_count数目
        for pass_node in self.pass_nodes.iter_mut() {
            pass_node.ref_count = pass_node.writes.len() as u32;

            for index in pass_node.reads.iter() {
                let index = *index as usize;

                self.resource_nodes[index].reader_count += 1;
            }
        }

        let mut stack: Vec<usize> = Vec::with_capacity(self.resource_nodes.len());

        //添加只有写入，没有读取的资源节点
        for (index, resource_node) in self.resource_nodes.iter().enumerate() {
            if resource_node.reader_count == 0 && resource_node.writer.is_some() {
                stack.push(index);
            }
        }

        while !stack.is_empty() {
            let index = stack.pop().unwrap();

            let writer_pass_node = &mut self.pass_nodes[index];

            //去除资源的引用
            writer_pass_node.ref_count -= 1;

            if writer_pass_node.ref_count == 0 {
                for resource_index in writer_pass_node.reads.iter() {
                    let resource_index = *resource_index as usize;

                    self.resource_nodes[resource_index].reader_count -= 1;

                    //添加节点去除后，需要再次剔除的资源节点
                    if self.resource_nodes[resource_index].reader_count == 0 {
                        stack.push(resource_index);
                    }
                }
            }
        }
    }

    pub fn sort(&mut self) {
        self.pass_nodes.sort();
    }

    ///execute阶段
    pub fn execute(&mut self) {}
}
