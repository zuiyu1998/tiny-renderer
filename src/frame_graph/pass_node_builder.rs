use std::sync::Arc;

use crate::{
    error::RendererError,
    gfx_base::{
        color_attachment::ColorAttachment,
        handle::TypeHandle,
        pipeline::{RenderPipeline, RenderPipelineDescriptor},
    },
};

use super::{
    FGResource, FGResourceDescriptor, FrameGraph, GpuRead, GpuWrite, ImportToFrameGraph, PassNode,
    RenderContext, ResourceNodeHandle, ResourceRef, TypeEquals,
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
    pub fn register_render_pipeline(
        &mut self,
        desc: &RenderPipelineDescriptor,
    ) -> TypeHandle<RenderPipeline> {
        self.graph.register_render_pipeline(desc.clone())
    }

    pub fn add_attachment(&mut self, color_attachment: ColorAttachment) {
        self.pass_node
            .as_mut()
            .unwrap()
            .add_attachment(color_attachment);
    }

    pub fn render(
        mut self,
        render: impl (FnOnce(&mut RenderContext) -> Result<(), RendererError>) + 'static,
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

    pub fn import<ResourceType>(
        &mut self,
        name: &str,
        resource: Arc<ResourceType>,
        desc: ResourceType::Descriptor,
    ) -> ResourceNodeHandle<ResourceType>
    where
        ResourceType: ImportToFrameGraph,
    {
        self.graph.import(name, resource, desc)
    }

    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> ResourceNodeHandle<DescriptorType::Resource>
    where
    DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn read_from_board<ResourceType>(
        &mut self,
        name: &str,
    ) -> Option<ResourceRef<ResourceType, GpuRead>> {
        self.pass_node
            .as_mut()
            .unwrap()
            .read_from_board(self.graph, name)
    }

    pub fn read<ResourceType>(
        &mut self,
        input_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, GpuRead> {
        self.pass_node
            .as_mut()
            .unwrap()
            .read(self.graph, input_handle)
    }

    pub fn write<ResourceType>(
        &mut self,
        out_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, GpuWrite> {
        self.pass_node
            .as_mut()
            .unwrap()
            .write(self.graph, out_handle)
    }
}
