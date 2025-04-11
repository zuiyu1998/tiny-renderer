use std::sync::Mutex;
use wgpu::SurfaceTexture;

use crate::{frame_graph::SwapChainTrait, gfx_base::texture_view::TextureView};

use super::WgpuTextureView;

#[derive(Debug)]
pub struct WgpuSwapChain {
    pub surface_texture: Mutex<Option<SurfaceTexture>>,
    pub surface_format: wgpu::TextureFormat,
}

impl SwapChainTrait for WgpuSwapChain {
    fn present(&self) {
        let mut guard = self.surface_texture.lock().unwrap();

        if let Some(surface_texture) = guard.take() {
            surface_texture.present();
        }
    }

    fn get_texture_view(&self) -> TextureView {
        let guard = self.surface_texture.lock().unwrap();
        let view = guard
            .as_ref()
            .unwrap()
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        TextureView::new(WgpuTextureView(view))
    }
}
