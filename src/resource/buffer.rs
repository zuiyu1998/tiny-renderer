use super::{AnyRenderResource, AnyRenderResourceRef, RenderResource, RenderResourceDescriptor};

pub struct Buffer {}

pub struct BufferDescriptor {}

impl RenderResource for Buffer {
    type Descriptor = BufferDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self {
        match res.borrow() {
            AnyRenderResourceRef::Buffer(buffer) => buffer,
            _ => unimplemented!(),
        }
    }
}

impl RenderResourceDescriptor for BufferDescriptor {
    type Resource = Buffer;
}
