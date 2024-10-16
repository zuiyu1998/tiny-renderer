use super::{AnyRenderResource, AnyRenderResourceRef, RenderResource, RenderResourceDescriptor};

pub struct Image {}

pub struct ImageDescriptor {}

impl RenderResource for Image {
    type Descriptor = ImageDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self {
        match res.borrow() {
            AnyRenderResourceRef::Image(image) => image,
            _ => unimplemented!(),
        }
    }
}

impl RenderResourceDescriptor for ImageDescriptor {
    type Resource = Image;
}
