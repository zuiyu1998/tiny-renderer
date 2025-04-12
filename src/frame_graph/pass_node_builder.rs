use std::sync::Arc;

use crate::{
    error::RendererError,
    gfx_base::{BindGroupRef, color_attachment::ColorAttachmentInfo, handle::TypeHandle},
};

use super::{
    FrameGraph, GpuRead, GpuWrite, ImportToFrameGraph, PassNode, RenderContext, Resource,
    ResourceDescriptor, ResourceNodeHandle, ResourceNodeRef, TypeEquals,
};

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
    pub fn add_bind_group(&mut self, bind_group: BindGroupRef) {
        self.pass_node.as_mut().unwrap().add_bind_group(bind_group);
    }

    pub fn add_attachment_info(&mut self, color_attachment: ColorAttachmentInfo) {
        self.pass_node
            .as_mut()
            .unwrap()
            .add_attachment(color_attachment);
    }

    pub fn render(
        mut self,
        render: impl (FnOnce(&mut RenderContext) -> Result<(), RendererError>) + 'static,
    ) {
        self.pass_node
            .as_mut()
            .unwrap()
            .render_fn
            .replace(Box::new(render));
    }

    pub fn new(insert_point: u32, name: &str, graph: &'a mut FrameGraph) -> Self {
        let handle = TypeHandle::new(graph.pass_nodes.len());
        Self {
            graph,
            pass_node: Some(PassNode::new(insert_point, name, handle)),
        }
    }

    fn build(&mut self) {
        assert!(self.pass_node.as_ref().unwrap().render_fn.is_some());

        let pass_node = self.pass_node.take().unwrap();
        self.graph.pass_nodes.push(pass_node);
    }

    pub fn import<ResourceType>(
        &mut self,
        name: &str,
        resource: Arc<ResourceType>,
    ) -> ResourceNodeHandle<ResourceType>
    where
        ResourceType: ImportToFrameGraph,
    {
        let desc = resource.get_desc().clone();
        self.graph.import(name, resource, desc)
    }

    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> ResourceNodeHandle<DescriptorType::Resource>
    where
        DescriptorType: ResourceDescriptor
            + TypeEquals<
                Other = <<DescriptorType as ResourceDescriptor>::Resource as Resource>::Descriptor,
            >,
    {
        self.graph.create(name, desc)
    }

    pub fn read_from_board<ResourceType>(
        &mut self,
        name: &str,
    ) -> Option<ResourceNodeRef<ResourceType, GpuRead>> {
        self.pass_node
            .as_mut()
            .unwrap()
            .read_from_board(self.graph, name)
    }

    pub fn read<ResourceType>(
        &mut self,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuRead> {
        self.pass_node
            .as_mut()
            .unwrap()
            .read(self.graph, resource_node_handle)
    }

    pub fn write<ResourceType>(
        &mut self,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuWrite> {
        self.pass_node
            .as_mut()
            .unwrap()
            .write(self.graph, resource_node_handle)
    }
}
