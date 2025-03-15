use wgpu::SurfaceTexture;

use crate::{
    device::DeviceTrait,
    render_pass::RenderPassDescriptor,
    swap_chain::{SwapChain, SwapChainDescriptor, SwapChainTrait},
};

pub struct WgpuDevice {
    device: wgpu::Device,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl WgpuDevice {
    pub fn create_render_pass(&self, desc: RenderPassDescriptor) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }

    pub fn new(
        device: wgpu::Device,
        surface: wgpu::Surface<'static>,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        WgpuDevice {
            device,
            surface,
            surface_format,
        }
    }
}

#[derive(Debug)]
pub struct WgpuSwapChain {
    surface_texture: Option<SurfaceTexture>,
}

impl SwapChainTrait for WgpuSwapChain {
    fn present(&mut self) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
    }
}

impl DeviceTrait for WgpuDevice {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let swap_chain = WgpuSwapChain {
            surface_texture: Some(surface_texture),
        };

        SwapChain::new(desc, swap_chain)
    }
}
