use std::marker::PhantomData;

use super::{
    resource_registry::RenderContext, GpuSrv, GpuUav, PassNode, PassResourceRef, PutResourceNode,
    Ref, RenderResource, RenderResourceDescriptor, ResourceNodeHandle, TemporalFrameGraph,
    TemporalResourceKey, TypeEquals,
};

use crate::{
    error::Result,
    renderer::resource::{BufferDescriptor, ImageDescriptor},
};

pub struct PassBuilder<'a> {
    pub frame_graph: &'a mut TemporalFrameGraph,
    pub pass_node: Option<PassNode>,
}

impl<'s> Drop for PassBuilder<'s> {
    fn drop(&mut self) {
        self.frame_graph
            .frame_graph
            .add_pass_node(self.pass_node.take().unwrap())
    }
}

impl<'a> PassBuilder<'a> {
    pub fn new(
        frame_graph: &'a mut TemporalFrameGraph,
        name: &str,
        insert_point: Option<u32>,
    ) -> Self {
        let id = frame_graph.frame_graph.pass_nodes.len() as u32;

        let insert_point = insert_point.unwrap_or(id);

        let pass_node = PassNode::new(id, insert_point, name);

        Self {
            frame_graph,
            pass_node: Some(pass_node),
        }
    }

    pub fn render(mut self, render: impl (FnOnce(&mut RenderContext) -> Result<()>) + 'static) {
        let prev = self
            .pass_node
            .as_mut()
            .unwrap()
            .render_fn
            .replace(Box::new(render));

        assert!(prev.is_none());
    }

    pub fn put_buffer(
        &mut self,
        key: impl Into<TemporalResourceKey>,
        descriptor: BufferDescriptor,
    ) -> Result<ResourceNodeHandle<<BufferDescriptor as RenderResourceDescriptor>::Resource>> {
        self.frame_graph.put(key, descriptor)
    }

    pub fn put_image(
        &mut self,
        key: impl Into<TemporalResourceKey>,
        descriptor: ImageDescriptor,
    ) -> Result<ResourceNodeHandle<<ImageDescriptor as RenderResourceDescriptor>::Resource>> {
        self.frame_graph.put(key, descriptor)
    }

    pub fn create<D: RenderResourceDescriptor>(
        &mut self,
        descriptor: D,
    ) -> ResourceNodeHandle<D::Resource>
    where
        D: TypeEquals<
            Other = <<D as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
        >,
    {
        self.frame_graph.frame_graph.create(descriptor)
    }

    pub fn write<ResType: RenderResource>(
        &mut self,
        handle: &ResourceNodeHandle<ResType>,
    ) -> Ref<ResType, GpuUav> {
        let pass = self.pass_node.as_mut().unwrap();

        let resource =
            &mut self.frame_graph.frame_graph.virtual_resources[handle.raw.index as usize];

        resource.new_version();

        let resource_handle = resource.get_handle();

        pass.writes.push(PassResourceRef {
            handle: resource_handle,
        });

        Ref {
            descriptor: handle.descriptor.clone(),
            handle: resource_handle,
            marker: PhantomData,
        }
    }

    pub fn read<ResType: RenderResource>(
        &mut self,
        handle: &ResourceNodeHandle<ResType>,
    ) -> Ref<ResType, GpuSrv> {
        let pass = self.pass_node.as_mut().unwrap();

        let resource =
            &mut self.frame_graph.frame_graph.virtual_resources[handle.raw.index as usize];

        resource.new_version();

        let resource_handle = resource.get_handle();

        pass.reads.push(PassResourceRef {
            handle: resource_handle,
        });

        Ref {
            descriptor: handle.descriptor.clone(),
            handle: resource_handle,
            marker: PhantomData,
        }
    }
}
