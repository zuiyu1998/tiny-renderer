use crate::gfx_base::{color_attachment::ColorAttachment, handle::TypeHandle};

use super::{
    DynRenderFn, FrameGraph, GpuRead, GpuWrite, LogicPass, VirtualResource, ResourceNode,
    ResourceNodeHandle, ResourceNodeRef,
};

pub struct PassNode {
    pub name: String,
    pub handle: TypeHandle<PassNode>,
    pub render_fn: Option<Box<DynRenderFn>>,
    pub writes: Vec<TypeHandle<ResourceNode>>,
    pub reads: Vec<TypeHandle<ResourceNode>>,
    pub insert_point: u32,
    pub resource_request_array: Vec<TypeHandle<VirtualResource>>,
    pub resource_release_array: Vec<TypeHandle<VirtualResource>>,
    pub color_attachments: Vec<ColorAttachment>,
}

impl PassNode {
    pub(crate) fn take(&mut self) -> LogicPass {
        let resource_release_array = self.resource_release_array.clone();
        let render_fn = self.render_fn.take();
        LogicPass {
            render_fn,
            resource_release_array,
        }
    }

    pub fn add_attachment(&mut self, color_attachment: ColorAttachment) {
        self.color_attachments.push(color_attachment);
    }

    pub fn write<ResourceType>(
        &mut self,
        graph: &mut FrameGraph,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuWrite> {
        let resource_handle = graph
            .get_resource_node(&resource_node_handle.resource_node_handle())
            .resource_handle;
        let resource = graph.get_resource_mut(&resource_handle);
        resource.info.new_version();

        let resource_info = resource.info.clone();
        let new_resource_node_handle = graph.create_resource_node(resource_info);
        let new_resource_node = graph.get_resource_node_mut(&new_resource_node_handle);
        new_resource_node.pass_node_writer_handle = Some(self.handle);

        self.writes.push(new_resource_node_handle);

        ResourceNodeRef::new(ResourceNodeHandle::new(
            new_resource_node_handle,
            resource_handle,
        ))
    }

    pub fn read_from_board<ResourceType>(
        &mut self,
        graph: &FrameGraph,
        name: &str,
    ) -> Option<ResourceNodeRef<ResourceType, GpuRead>> {
        if let Some(handle) = graph.get_resource_board().get(name) {
            if !self.reads.contains(&handle.resource_node_handle()) {
                self.reads.push(handle.resource_node_handle());
            }

            Some(ResourceNodeRef::new(handle.clone().into()))
        } else {
            None
        }
    }

    pub fn read<ResourceType>(
        &mut self,
        graph: &FrameGraph,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuRead> {
        let resource_node_handle = resource_node_handle.resource_node_handle();

        if !self.reads.contains(&resource_node_handle) {
            self.reads.push(resource_node_handle);
        }

        let resource_handle = graph
            .get_resource_node(&resource_node_handle)
            .resource_handle;

        ResourceNodeRef::new(ResourceNodeHandle::new(
            resource_node_handle,
            resource_handle,
        ))
    }

    pub fn new(insert_point: u32, name: &str, handle: TypeHandle<PassNode>) -> Self {
        PassNode {
            name: name.to_string(),
            handle,
            render_fn: None,
            writes: vec![],
            reads: vec![],
            insert_point,
            resource_request_array: vec![],
            resource_release_array: vec![],
            color_attachments: vec![],
        }
    }
}
