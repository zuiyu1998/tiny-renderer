use super::{
    AnyRenderResource, AnyRenderResourceDescriptor, AnyRenderResourceRef, RenderResource,
    RenderResourceDescriptor,
};

pub struct Image {}

impl RenderResource for Image {
    type Descriptor = ImageDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self {
        match res.borrow() {
            AnyRenderResourceRef::Image(image) => image,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageDescriptor {}

impl From<ImageDescriptor> for AnyRenderResourceDescriptor {
    fn from(value: ImageDescriptor) -> Self {
        AnyRenderResourceDescriptor::Image(value)
    }
}
impl RenderResourceDescriptor for ImageDescriptor {
    type Resource = Image;
}
