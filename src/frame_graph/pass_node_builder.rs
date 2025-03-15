use std::sync::Arc;

use crate::{FGResource, FGResourceDescriptor, RendererError, TypeEquals, handle::TypeHandle};

use super::{FrameGraph, GraphResourceHandle, PassNode, ResourceNode, ResourceTable};

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
    pub fn render(
        mut self,
        render: impl (FnOnce(&mut ResourceTable) -> Result<(), RendererError>) + 'static,
    ) {
        let prev = self
            .pass_node
            .as_mut()
            .unwrap()
            .render_fn
            .replace(Box::new(render));

        assert!(prev.is_none());
    }

    pub fn new(insert_point: u32, name: &str, graph: &'a mut FrameGraph) -> Self {
        let handle = TypeHandle::new(graph.pass_nodes.len());
        Self {
            graph,
            pass_node: Some(PassNode::new(insert_point, name, handle)),
        }
    }

    fn build(&mut self) {
        let pass_node = self.pass_node.take().unwrap();
        self.graph.pass_nodes.push(pass_node);
    }

    pub fn imported<ResourceType>(
        &mut self,
        name: &str,
        resouce: Arc<ResourceType>,
    ) -> GraphResourceHandle
    where
        ResourceType: FGResource,
    {
        self.graph.imported(name, resouce)
    }

    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> GraphResourceHandle
    where
    DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn read(&mut self, input_handle: TypeHandle<ResourceNode>) -> GraphResourceHandle {
        self.pass_node
            .as_mut()
            .unwrap()
            .read(&self.graph, input_handle)
    }

    pub fn write(&mut self, out_handle: TypeHandle<ResourceNode>) -> GraphResourceHandle {
        self.pass_node
            .as_mut()
            .unwrap()
            .write(&mut self.graph, out_handle)
    }
}
