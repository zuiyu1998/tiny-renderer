use crate::{
    frame_graph::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor, ImportToFrameGraph},
    gfx_base::buffer::{Buffer, BufferDescriptor},
};

impl FGResourceDescriptor for BufferDescriptor {
    type Resource = Buffer;
}

impl From<BufferDescriptor> for AnyFGResourceDescriptor {
    fn from(value: BufferDescriptor) -> Self {
        AnyFGResourceDescriptor::Buffer(value)
    }
}

impl ImportToFrameGraph for Buffer {
    fn import(self: std::sync::Arc<Self>) -> super::ImportedResource {
        super::ImportedResource::Buffer(self)
    }
}

impl FGResource for Buffer {
    type Descriptor = BufferDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match &res {
            AnyFGResource::OwnedBuffer(res) => res,
            AnyFGResource::ImportedBuffer(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn get_desc(&self) -> &Self::Descriptor {
        todo!()
    }

    fn borrow_resource_mut(_res: &mut AnyFGResource) -> &mut Self {
        todo!()
    }
}
