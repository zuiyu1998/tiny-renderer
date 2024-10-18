use super::{
    AnyRenderResource, AnyRenderResourceDescriptor, AnyRenderResourceRef, RenderResource,
    RenderResourceDescriptor,
};

use crate::renderer::resource::{Image, ImageDescriptor};

impl RenderResource for Image {
    type Descriptor = ImageDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self {
        match res.borrow() {
            AnyRenderResourceRef::Image(image) => image,
            _ => unimplemented!(),
        }
    }
}

impl From<ImageDescriptor> for AnyRenderResourceDescriptor {
    fn from(value: ImageDescriptor) -> Self {
        AnyRenderResourceDescriptor::Image(value)
    }
}
impl RenderResourceDescriptor for ImageDescriptor {
    type Resource = Image;
}
