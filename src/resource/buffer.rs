use super::{
    AnyRenderResource, AnyRenderResourceDescriptor, AnyRenderResourceRef, RenderResource,
    RenderResourceDescriptor,
};

pub struct Buffer {}

impl RenderResource for Buffer {
    type Descriptor = BufferDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self {
        match res.borrow() {
            AnyRenderResourceRef::Buffer(buffer) => buffer,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub struct BufferDescriptor {}

impl From<BufferDescriptor> for AnyRenderResourceDescriptor {
    fn from(value: BufferDescriptor) -> Self {
        AnyRenderResourceDescriptor::Buffer(value)
    }
}

impl RenderResourceDescriptor for BufferDescriptor {
    type Resource = Buffer;
}
