use crate::gfx_base::{TextureInfo, TextureTrait, TextureView, TextureViewInfo};

use super::WgpuTextureView;

#[derive(Debug, Clone)]
pub struct WgpuTexture {
    pub texture: wgpu::Texture,
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
}

impl TextureTrait for WgpuTexture {
    fn write_texture(&self, bytes: &[u8], info: &TextureInfo) {
        self.queue.write_texture(
            self.texture.as_image_copy(),
            bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * info.size.width),
                rows_per_image: None,
            },
            info.size,
        );
    }

    fn get_texture_view(&self, desc: TextureViewInfo) -> TextureView {
        let texture_view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        TextureView::new(WgpuTextureView(texture_view), desc)
    }
}
