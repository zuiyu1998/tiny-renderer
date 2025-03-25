pub mod bind_group_layout;
pub mod device;
pub mod pipeline_layout;
pub mod render_pipeline;
pub mod shader_module;

pub use bind_group_layout::*;
pub use device::*;
pub use pipeline_layout::*;
pub use render_pipeline::*;
pub use shader_module::*;

use std::{ops::Range, sync::Mutex};
use wgpu::SurfaceTexture;

use crate::{
    frame_graph::SwapChainTrait,
    gfx_base::{
        command_buffer::CommandBufferTrait,
        pipeline::RenderPipeline,
        render_pass::{RenderPass, RenderPassTrait},
        texture_view::{TextureView, TextureViewTrait},
    },
};

#[derive(Debug)]
pub struct WgpuTextView(wgpu::TextureView);

impl TextureViewTrait for WgpuTextView {}

pub struct WgpuCommandBuffer(Option<wgpu::CommandBuffer>);

impl CommandBufferTrait for WgpuCommandBuffer {
    fn finish(&mut self, render_pass: RenderPass) {
        let render_pass: Box<WgpuRenderPass> = render_pass.downcast().unwrap();

        drop(render_pass.render_pass);

        let command_buffer = render_pass.encoder.finish();

        self.0 = Some(command_buffer);
    }
}

#[derive(Debug)]
pub struct WgpuRenderPass {
    render_pass: wgpu::RenderPass<'static>,
    encoder: wgpu::CommandEncoder,
}

impl RenderPassTrait for WgpuRenderPass {
    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        let render_pipeline = render_pipeline
            .downcast_ref::<WgpuRenderPipeline>()
            .unwrap();

        self.render_pass.set_pipeline(&render_pipeline.0);
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw(vertices, instances);
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
