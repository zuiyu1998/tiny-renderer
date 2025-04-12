use std::ops::Range;

use crate::{
    gfx_base::{
        BindGroup, Buffer, command_buffer::CommandBufferTrait, device::Device,
        pipeline::RenderPipeline, render_pass::RenderPass,
    },
    gfx_wgpu::{WgpuDevice, WgpuRenderPipeline, WgpuTextureView, render_pass::WgpuRenderPass},
};

use super::{WgpuBindGroup, WgpuBuffer};

#[derive(Debug, Default)]
pub struct WgpuCommandBuffer {
    encoder: Option<wgpu::CommandEncoder>,
    render_pass: Option<wgpu::RenderPass<'static>>,
    pub command_buffer: Option<wgpu::CommandBuffer>,
}

impl CommandBufferTrait for WgpuCommandBuffer {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass) {
        let device = device.downcast_ref::<WgpuDevice>().unwrap();

        let mut render_pass = render_pass.downcast::<WgpuRenderPass>().unwrap();

        let mut color_attachments = vec![];

        let texture_views = render_pass.texture_views.take().unwrap();

        for texture_view in texture_views.iter() {
            let texture_view = texture_view.downcast_ref::<WgpuTextureView>().unwrap();

            color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                view: &texture_view.0,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            }));
        }

        let mut encoder = device.device.create_command_encoder(&Default::default());
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &color_attachments,
            ..Default::default()
        });

        let render_pass = render_pass.forget_lifetime();

        self.encoder = Some(encoder);
        self.render_pass = Some(render_pass);
    }

    fn end_render_pass(&mut self) {
        let render_pass = self.render_pass.take().unwrap();
        let encoder = self.encoder.take().unwrap();

        drop(render_pass);

        let command_buffer = encoder.finish();

        self.command_buffer = Some(command_buffer);
    }

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        let render_pipeline = render_pipeline
            .downcast_ref::<WgpuRenderPipeline>()
            .unwrap();

        if let Some(render_pass) = self.render_pass.as_mut() {
            render_pass.set_pipeline(&render_pipeline.0);
        }
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        if let Some(render_pass) = self.render_pass.as_mut() {
            render_pass.draw(vertices, instances);
        }
    }

    fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) {
        let buffer = buffer.downcast_ref::<WgpuBuffer>().unwrap();

        if let Some(render_pass) = self.render_pass.as_mut() {
            render_pass.set_vertex_buffer(slot, buffer.buffer.slice(0..));
        }
    }

    fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup) {
        let bind_group = bind_group.downcast_ref::<WgpuBindGroup>().unwrap();

        if let Some(render_pass) = self.render_pass.as_mut() {
            render_pass.set_bind_group(index, Some(&bind_group.0), &[]);
        }
    }

    fn set_index_buffer(&mut self, buffer: &Buffer, index_format: wgpu::IndexFormat) {
        let buffer = buffer.downcast_ref::<WgpuBuffer>().unwrap();

        if let Some(render_pass) = self.render_pass.as_mut() {
            render_pass.set_index_buffer(buffer.buffer.slice(0..), index_format);
        }
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        if let Some(render_pass) = self.render_pass.as_mut() {
            render_pass.draw_indexed(indices, base_vertex, instances);
        }
    }
}
