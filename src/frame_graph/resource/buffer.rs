use std::{marker::PhantomData, sync::Arc};

use crate::{
    frame_graph::FrameGraph,
    render_backend::RenderDevice,
    renderer::resource::{Buffer, BufferDescriptor},
};

use super::{
    AnyRenderResource, AnyRenderResourceDescriptor, AnyRenderResourceRef, GraphResourceImportInfo,
    ImportToFrameGraph, RawResourceNodeHandle, RenderResource, RenderResourceDescriptor,
    ResourceNode, ResourceNodeHandle, VirtualResource,
};

impl ImportToFrameGraph for Buffer {
    fn import(self: Arc<Self>, fg: &mut FrameGraph) -> ResourceNodeHandle<Self> {
        let raw = RawResourceNodeHandle {
            index: fg.resource_nodes.len() as u32,
        };

        let res = VirtualResource {
            id: fg.resource_nodes.len() as u32,
            imported: true,
            ..Default::default()
        };

        let descriptor = self.descriptor.clone();

        fg.virtual_resources.push(res);
        fg.resource_nodes
            .push(ResourceNode::imported(GraphResourceImportInfo::Buffer {
                resource: self,
            }));

        ResourceNodeHandle {
            raw,
            descriptor,
            marker: PhantomData,
        }
    }
}

impl RenderResource for Buffer {
    type Descriptor = BufferDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self {
        match res.borrow() {
            AnyRenderResourceRef::Buffer(buffer) => buffer,
            _ => unimplemented!(),
        }
    }
}

impl From<BufferDescriptor> for AnyRenderResourceDescriptor {
    fn from(value: BufferDescriptor) -> Self {
        AnyRenderResourceDescriptor::Buffer(value)
    }
}

impl RenderResourceDescriptor for BufferDescriptor {
    type Resource = Buffer;

    fn create_resource(&self, device: &RenderDevice) -> Self::Resource {
        let buffer = device.create_render_buffer(self);
        Buffer::new(buffer, self.clone())
    }
}
