use std::sync::Mutex;

use wgpu::SurfaceTexture;

use crate::{
    frame_graph::{SwapChain, SwapChainDescriptor, SwapChainTrait},
    gfx_base::{
        command_buffer::{CommandBuffer, CommandBufferTrait},
        device::DeviceTrait,
        render_pass::{RenderPass, RenderPassDescriptor, RenderPassTrait},
    },
};

pub struct WgpuCommandBuffer(wgpu::CommandBuffer);

impl CommandBufferTrait for WgpuCommandBuffer {}

#[derive(Debug)]
pub struct WgpuRenderPass {
    render_pass: Option<wgpu::RenderPass<'static>>,
    encoder: Option<wgpu::CommandEncoder>,
}

impl RenderPassTrait for WgpuRenderPass {
    fn finish(&mut self) -> CommandBuffer {
        let render_pass = self.render_pass.take().unwrap();
        drop(render_pass);

        let encoder = self.encoder.take().unwrap();
        CommandBuffer::new(WgpuCommandBuffer(encoder.finish()))
    }
}

pub struct WgpuDevice {
    device: wgpu::Device,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    queue: wgpu::Queue,
}

impl WgpuDevice {
    pub fn new(
        device: wgpu::Device,
        surface: wgpu::Surface<'static>,
        surface_format: wgpu::TextureFormat,
        queue: wgpu::Queue,
    ) -> Self {
        WgpuDevice {
            device,
            surface,
            surface_format,
            queue,
        }
    }
}

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

    fn get_texture_view(&self) -> wgpu::TextureView {
        let guard = self.surface_texture.lock().unwrap();
        guard
            .as_ref()
            .unwrap()
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            })
    }
}

impl DeviceTrait for WgpuDevice {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let swap_chain = WgpuSwapChain {
            surface_texture: Mutex::new(Some(surface_texture)),
            surface_format: self.surface_format,
        };

        SwapChain::new(desc, swap_chain)
    }

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        let mut color_attachments = vec![];

        for color_attachment in desc.color_attachments.iter() {
            color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                view: color_attachment.view.get_texture_view(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            }));
        }

        let mut encoder = self.device.create_command_encoder(&Default::default());
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let render_pass = render_pass.forget_lifetime();

        RenderPass::new(WgpuRenderPass {
            render_pass: Some(render_pass),
            encoder: Some(encoder),
        })
    }

    fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        let command_buffers = command_buffers
            .into_iter()
            .map(|command_buffer| {
                let wgpu_command_buffer: Box<WgpuCommandBuffer> =
                    command_buffer.downcast().unwrap();

                wgpu_command_buffer.0
            })
            .collect::<Vec<wgpu::CommandBuffer>>();

        self.queue.submit(command_buffers);
    }
}
