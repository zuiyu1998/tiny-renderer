use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_frame_graph_type};
use std::{borrow::Cow, fmt::Debug};

use super::{TextureView, TextureViewInfo};

define_atomic_id!(TextureId);

pub trait TextureTrait: 'static + Clone + Debug + Sync + Send {
    fn write_texture(&self, bytes: &[u8], info: &TextureInfo);

    fn get_texture_view(&self, desc: TextureViewInfo) -> TextureView;
}

pub trait ErasedTextureTrait: 'static + Downcast + Debug + Sync + Send {
    fn write_texture(&self, bytes: &[u8], info: &TextureInfo);

    fn get_texture_view(&self, desc: TextureViewInfo) -> TextureView;
}

impl<T: TextureTrait> ErasedTextureTrait for T {
    fn write_texture(&self, bytes: &[u8], info: &TextureInfo) {
        <T as TextureTrait>::write_texture(self, bytes, info);
    }

    fn get_texture_view(&self, desc: TextureViewInfo) -> TextureView {
        <T as TextureTrait>::get_texture_view(self, desc)
    }
}

define_gfx_frame_graph_type!(
    Texture,
    TextureId,
    TextureTrait,
    ErasedTextureTrait,
    TextureInfo
);

impl Texture {
    pub fn write_texture(&self, bytes: &[u8]) {
        self.value.write_texture(bytes, &self.desc);
    }

    pub fn get_texture_view(&self, desc: TextureViewInfo) -> TextureView {
        self.value.get_texture_view(desc)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureInfo {
    pub dimension: wgpu::TextureDimension,
    pub format: wgpu::TextureFormat,
    pub usage: wgpu::TextureUsages,
    pub size: wgpu::Extent3d,
    pub label: Option<Cow<'static, str>>,
}
