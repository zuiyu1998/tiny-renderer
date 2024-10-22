use std::{marker::PhantomData, sync::Arc};

use super::{
    AnyRenderResource, AnyRenderResourceDescriptor, AnyRenderResourceRef, GraphResourceImportInfo,
    ImportToFrameGraph, RawResourceNodeHandle, RenderResource, RenderResourceDescriptor,
    ResourceNode, ResourceNodeHandle, VirtualResource,
};

use crate::{
    frame_graph::FrameGraph,
    render_backend::RenderDevice,
    renderer::resource::{Image, ImageDescriptor},
};

impl ImportToFrameGraph for Image {
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
            .push(ResourceNode::imported(GraphResourceImportInfo::Image {
                resource: self,
            }));

        ResourceNodeHandle {
            raw,
            descriptor,
            marker: PhantomData,
        }
    }
}

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

    fn create_resource(&self, _device: &RenderDevice) -> Self::Resource {
        todo!()
    }
}
