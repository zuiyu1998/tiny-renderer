use std::sync::Mutex;

use crate::{
    frame_graph::{SwapChain, SwapChainDescriptor},
    gfx_base::{
        command_buffer::CommandBuffer,
        device::DeviceTrait,
        pipeline::{RenderPipeline, RenderPipelineDescriptorState},
        pipeline_layout::{PipelineLayout, PipelineLayoutDescriptor},
        render_pass::{RenderPass, RenderPassDescriptor},
        shader_module::{ShaderModule, ShaderModuleDescriptor},
    },
    gfx_wgpu::WgpuBindGroupLayout,
};

use super::{
    WgpuCommandBuffer, WgpuPipelineLayout, WgpuRenderPass, WgpuRenderPipeline, WgpuShaderModule,
    WgpuSwapChain, WgpuTextView,
};

#[derive(Debug)]
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
            render_pass,
            encoder,
        })
    }

    fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        let command_buffers = command_buffers
            .into_iter()
            .map(|command_buffer| {
                let wgpu_command_buffer: Box<WgpuCommandBuffer> =
                    command_buffer.downcast().unwrap();
                wgpu_command_buffer.0.unwrap()
            })
            .collect::<Vec<wgpu::CommandBuffer>>();

        self.queue.submit(command_buffers);
    }

    fn create_render_pipeline(&self, state: RenderPipelineDescriptorState) -> RenderPipeline {
        let vertex_module = state
            .vertex_module
            .downcast_ref::<WgpuShaderModule>()
            .unwrap();

        let vertex_buffer_layouts = state
            .desc
            .vertex
            .buffers
            .iter()
            .map(|layout| wgpu::VertexBufferLayout {
                array_stride: layout.array_stride,
                attributes: &layout.attributes,
                step_mode: layout.step_mode,
            })
            .collect::<Vec<_>>();

        let vertex_state: wgpu::VertexState = wgpu::VertexState {
            module: vertex_module.shader_module(),
            entry_point: Some(&state.desc.vertex.entry_point),
            compilation_options: Default::default(),
            buffers: &vertex_buffer_layouts,
        };

        let fragment_state = state.fragment_module.as_ref().map(|fragment_module| {
            let fragment_module = fragment_module.downcast_ref::<WgpuShaderModule>().unwrap();
            let fragment = state.desc.fragment.as_ref().unwrap();

            wgpu::FragmentState {
                module: fragment_module.shader_module(),
                entry_point: Some(&fragment.entry_point),
                compilation_options: Default::default(),
                targets: &fragment.targets,
            }
        });

        let layout = state.layout.as_ref().map(|layout| {
            layout
                .downcast_ref::<WgpuPipelineLayout>()
                .unwrap()
                .pipeline_layout()
        });

        let label = state.desc.label.as_ref().map(|label| label.to_string());

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: label.as_deref(),
                layout,
                vertex: vertex_state,
                fragment: fragment_state,
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
                multiview: None,
                cache: None,
            });

        RenderPipeline::new(WgpuRenderPipeline::new(render_pipeline))
    }

    fn create_command_buffer(&self) -> CommandBuffer {
        CommandBuffer::new(WgpuCommandBuffer(None))
    }

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
        let label = desc.label.as_ref().map(|label| label.to_string());

        let source =
            wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(desc.source.source.as_str()));

        let shader_module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: label.as_deref(),
                source,
            });

        ShaderModule::new(WgpuShaderModule::new(shader_module))
    }

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout {
        let bind_group_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|layout| {
                let layout = layout
                    .downcast_ref::<WgpuBindGroupLayout>()
                    .unwrap()
                    .bind_group_layout();
                layout
            })
            .collect::<Vec<_>>();

        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &desc.push_constant_ranges,
            });

        PipelineLayout::new(WgpuPipelineLayout::new(layout))
    }
}
