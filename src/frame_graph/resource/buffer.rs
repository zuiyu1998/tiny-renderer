use crate::{
    frame_graph::{
        AnyResource, AnyResourceDescriptor, ImportToFrameGraph, Resource, ResourceDescriptor,
    },
    gfx_base::buffer::{Buffer, BufferInfo},
};

use super::ImportedVirtualResource;

impl ResourceDescriptor for BufferInfo {
    type Resource = Buffer;
}

impl From<BufferInfo> for AnyResourceDescriptor {
    fn from(value: BufferInfo) -> Self {
        AnyResourceDescriptor::Buffer(value)
    }
}

impl ImportToFrameGraph for Buffer {
    fn import(self: std::sync::Arc<Self>) -> ImportedVirtualResource {
        ImportedVirtualResource::Buffer(self)
    }
}

impl Resource for Buffer {
    type Descriptor = BufferInfo;

    fn borrow_resource(res: &AnyResource) -> &Self {
        match &res {
            AnyResource::OwnedBuffer(res) => res,
            AnyResource::ImportedBuffer(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
    
    fn get_desc(&self) -> &Self::Descriptor {
        self.get_desc()
    }
}
