use std::fmt::Debug;

use crate::gfx_base::{color_attachment::ColorAttachment, handle::TypeHandle};

use super::{
    DynRenderFn, FrameGraph, GpuRead, GpuWrite, LogicPass, Resource, ResourceNode,
    ResourceNodeHandle, ResourceRef,
};

pub struct PassNode {
    ///唯一的节点名称
    pub name: String,
    pub handle: TypeHandle<PassNode>,
    pub render_fn: Option<Box<DynRenderFn>>,
    pub writes: Vec<TypeHandle<ResourceNode>>,
    pub reads: Vec<TypeHandle<ResourceNode>>,
    ///渲染节点的插入顺序
    pub insert_point: u32,
    ///使用资源的获取生命周期
    pub resource_request_array: Vec<TypeHandle<Resource>>,
    ///使用资源的释放生命周期
    pub resource_release_array: Vec<TypeHandle<Resource>>,

    //render pass 配置
    pub color_attachments: Vec<ColorAttachment>,
}

impl Debug for PassNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PassNode")
            .field("name", &self.name)
            .field("handle", &self.handle)
            .field("render_fn", &self.render_fn.is_some())
            .field("writes", &self.writes)
            .field("reads", &self.reads)
            .field("insert_point", &self.insert_point)
            .field("resource_request_array", &self.resource_request_array)
            .field("resource_release_array", &self.resource_release_array)
            .field("color_attachments", &self.color_attachments)
            .finish()
    }
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
        out_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, GpuWrite> {
        let resource_handle = graph
            .get_resource_node(&out_handle.resource_node_handle())
            .resource_handle
            .clone();
        let resource = graph.get_resource_mut(&resource_handle);
        resource.get_info_mut().new_version();

        let resource_info = resource.get_info().clone();
        let new_resource_node_handle = graph.create_resource_node(resource_info);
        let new_resource_node = graph.get_resource_node_mut(&new_resource_node_handle);

        new_resource_node.pass_node_writer_handle = Some(self.handle.clone());

        self.writes.push(new_resource_node_handle.clone());

        ResourceRef::new(ResourceNodeHandle::new(
            new_resource_node_handle,
            resource_handle,
        ))
    }

    pub fn read_from_board<ResourceType>(
        &mut self,
        graph: &FrameGraph,
        name: &str,
    ) -> Option<ResourceRef<ResourceType, GpuRead>> {
        if let Some(handle) = graph.get_resource_board().get(name) {
            if !self.reads.contains(&handle.resource_node_handle()) {
                self.reads.push(handle.resource_node_handle());
            }

            Some(ResourceRef::new(handle.clone().into()))
        } else {
            None
        }
    }

    pub fn read<ResourceType>(
        &mut self,
        graph: &FrameGraph,
        input_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, GpuRead> {
        let resource_node_handle = input_handle.resource_node_handle();

        if !self.reads.contains(&resource_node_handle) {
            self.reads.push(resource_node_handle.clone());
        }

        let resource_handle = graph
            .get_resource_node(&resource_node_handle)
            .resource_handle
            .clone();

        ResourceRef::new(ResourceNodeHandle::new(
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
