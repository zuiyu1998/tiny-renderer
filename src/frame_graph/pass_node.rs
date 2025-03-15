use crate::gfx_base::handle::TypeHandle;

use super::{DynRenderFn, FrameGraph, GraphResourceHandle, LogicPass, Resource, ResourceNode};

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

    pub fn write(
        &mut self,
        graph: &mut FrameGraph,
        out_handle: TypeHandle<ResourceNode>,
    ) -> GraphResourceHandle {
        let resource_handle = graph.get_resource_node(&out_handle).resource_handle.clone();
        let resource = graph.get_resource_mut(&resource_handle);
        resource.get_info_mut().new_version();

        let resource_info = resource.get_info().clone();
        let new_resource_node_handle = graph.create_resource_node(resource_info);
        let new_resource_node = graph.get_resource_node_mut(&new_resource_node_handle);

        new_resource_node.pass_node_writer_handle = Some(self.handle.clone());

        self.writes.push(new_resource_node_handle.clone());

        GraphResourceHandle {
            resource_node_handle: new_resource_node_handle,
            resource_handle,
        }
    }

    pub fn read(
        &mut self,
        graph: &FrameGraph,
        input_handle: TypeHandle<ResourceNode>,
    ) -> GraphResourceHandle {
        if !self.reads.contains(&input_handle) {
            self.reads.push(input_handle.clone());
        }

        let resource_handle = graph
            .get_resource_node(&input_handle)
            .resource_handle
            .clone();

        GraphResourceHandle {
            resource_node_handle: input_handle,
            resource_handle,
        }
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
        }
    }
}
