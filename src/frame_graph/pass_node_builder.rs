use crate::{FGResource, FGResourceDescriptor, TypeEquals, handle::TypeHandle};

use super::{FrameGraph, PassNode, ResourceNode};

pub struct PassNodeBuilder<'a> {
    graph: &'a mut FrameGraph,
    pass_node: Option<PassNode>,
}

impl Drop for PassNodeBuilder<'_> {
    fn drop(&mut self) {
        self.build();
    }
}

impl<'a> PassNodeBuilder<'a> {
    pub fn new(insert_point: u32, name: &str, graph: &'a mut FrameGraph) -> Self {
        let handle = TypeHandle::new(graph.pass_nodes.len());
        Self {
            graph,
            pass_node: Some(PassNode::new(insert_point, name, handle)),
        }
    }

    fn build(&mut self) {
        let pass_node = self.pass_node.take().unwrap();

        assert!(pass_node.render_fn.is_some());

        self.graph.pass_nodes.push(pass_node);
    }


    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> TypeHandle<ResourceNode>
    where
    DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn read(&mut self, input_handle: TypeHandle<ResourceNode>) -> TypeHandle<ResourceNode> {
        self.pass_node.as_mut().unwrap().read(input_handle)
    }

    pub fn write(&mut self, out_handle: TypeHandle<ResourceNode>) -> TypeHandle<ResourceNode> {
        self.pass_node
            .as_mut()
            .unwrap()
            .write(&mut self.graph, out_handle)
    }
}
