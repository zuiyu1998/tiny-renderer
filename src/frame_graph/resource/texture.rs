use crate::{
    frame_graph::{AnyResource, AnyResourceDescriptor, Resource, ResourceDescriptor},
    gfx_base::texture::{TextureInfo, Texture},
};

impl ResourceDescriptor for TextureInfo {
    type Resource = Texture;
}

impl From<TextureInfo> for AnyResourceDescriptor {
    fn from(value: TextureInfo) -> Self {
        AnyResourceDescriptor::Texture(value)
    }
}

impl Resource for Texture {
    type Descriptor = TextureInfo;

    fn borrow_resource(res: &AnyResource) -> &Self {
        match &res {
            AnyResource::OwnedTexture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
    
    fn get_desc(&self) -> &Self::Descriptor {
        self.get_desc()
    }
}
