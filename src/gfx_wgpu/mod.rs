mod render_pipeline;

pub use render_pipeline::*;

use std::{ops::Range, sync::Mutex};
use wgpu::SurfaceTexture;

use crate::{
    frame_graph::{SwapChain, SwapChainDescriptor, SwapChainTrait},
    gfx_base::{
        command_buffer::{CommandBuffer, CommandBufferTrait},
        device::DeviceTrait,
        pipeline::{RenderPipeline, RenderPipelineDescriptor},
        render_pass::{RenderPass, RenderPassDescriptor, RenderPassTrait},
        texture_view::{TextureView, TextureViewTrait},
    },
};

#[derive(Debug)]
pub struct WgpuTextView(wgpu::TextureView);

impl TextureViewTrait for WgpuTextView {}

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
            let texture_view = color_attachment
                .view
                .get_texture_view()
                .downcast_ref::<WgpuTextView>()
                .unwrap();

            color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                view: &texture_view.0,
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

    fn create_render_pipeline(&self, _desc: RenderPipelineDescriptor) -> RenderPipeline {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.surface_format.add_srgb_suffix(),
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
                cache: None,
            });

        RenderPipeline::new(WgpuRenderPipeline::new(render_pipeline))
    }
}
