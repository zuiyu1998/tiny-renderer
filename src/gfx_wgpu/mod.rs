pub mod bind_group_layout;
pub mod command_buffer;
pub mod device;
pub mod pipeline_layout;
pub mod render_pass;
pub mod render_pipeline;
pub mod shader_module;
pub mod buffer;

pub use bind_group_layout::*;
pub use command_buffer::*;
pub use device::*;
pub use pipeline_layout::*;
pub use render_pipeline::*;
pub use shader_module::*;
pub use buffer::*;

use std::sync::Mutex;
use wgpu::SurfaceTexture;

use crate::{
    frame_graph::SwapChainTrait,
    gfx_base::texture_view::{TextureView, TextureViewTrait},
};

#[derive(Debug)]
pub struct WgpuTextView(wgpu::TextureView);

impl TextureViewTrait for WgpuTextView {}

#[derive(Debug)]
pub struct WgpuSwapChain {
    surface_texture: Mutex<Option<SurfaceTexture>>,
    surface_format: wgpu::TextureFormat,
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

        TextureView::new(WgpuTextView(view))
    }
}
