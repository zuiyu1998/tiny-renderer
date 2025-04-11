use std::sync::Arc;

use crate::{
    frame_graph::{
        AnyResource, AnyResourceDescriptor, ImportToFrameGraph, Resource, ResourceDescriptor,
    },
    gfx_base::texture_view::{TextureView, TextureViewInfo},
};

use super::ImportedVirtualResource;

impl ResourceDescriptor for TextureViewInfo {
    type Resource = TextureView;
}

impl From<TextureViewInfo> for AnyResourceDescriptor {
    fn from(value: TextureViewInfo) -> Self {
        AnyResourceDescriptor::TextureView(value)
    }
}

impl ImportToFrameGraph for TextureView {
    fn import(self: Arc<Self>) -> ImportedVirtualResource {
        ImportedVirtualResource::TextureView(self)
    }
}

impl Resource for TextureView {
    type Descriptor = TextureViewInfo;

    fn borrow_resource(res: &AnyResource) -> &Self {
        match &res {
            AnyResource::ImportedTextureView(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
    
    fn get_desc(&self) -> &Self::Descriptor {
        self.get_desc()
    }
}
