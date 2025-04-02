use std::sync::{Arc, mpsc::Receiver};

use fyrox_core::parking_lot::Mutex;
use fyrox_resource::event::ResourceEvent;
use wgpu::{BufferUsages, ColorTargetState, TextureFormat};

use crate::{
    build_in::get_test,
    frame_graph::SwapChainDescriptor,
    gfx_base::{
        buffer::{Buffer, BufferInitDescriptor},
        color_attachment::ColorAttachment,
        device::Device,
        pipeline::{FragmentState, RenderPipelineDescriptor, VertexBufferLayout, VertexState},
        shader::Shader,
    },
    renderer::Renderer,
};

pub struct InitializationGraphicContext {
    renderer: Renderer,
    params: GraphicContextParams,
    shader_event_receiver: Receiver<ResourceEvent>,
    format: TextureFormat,
    vertex_buffer: Mutex<Arc<Buffer>>,
}

impl InitializationGraphicContext {
    pub fn new(
        device: Arc<Device>,
        params: GraphicContextParams,
        shader_event_receiver: Receiver<ResourceEvent>,
        format: TextureFormat,
    ) -> Self {
        let renderer = Renderer::new(device.clone());

        let vertex_buffers = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let buffer = device.create_buffer_init(BufferInitDescriptor {
            label: Some("vertex_buffer".into()),
            usage: BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&vertex_buffers),
        });

        let vertex_buffer = Mutex::new(Arc::new(buffer));

        InitializationGraphicContext {
            renderer,
            params,
            shader_event_receiver,
            format,
            vertex_buffer,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl InitializationGraphicContext {
    fn update_pipeline_cache(&mut self, dt: f32) {
        while let Ok(event) = self.shader_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource)
            | ResourceEvent::Reloaded(resource)
            | ResourceEvent::Added(resource) = event
            {
                if let Some(shader) = resource.try_cast::<Shader>() {
                    self.renderer.pipeline_cache_mut().remove(&shader);
                    self.renderer.pipeline_cache_mut().set_shader(&shader);
                }
            }
        }

        self.renderer.pipeline_cache_mut().update(dt);
    }

    fn render(&mut self, dt: f32) {
        let vertex_buffer_layout = VertexBufferLayout {
            array_stride: core::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: core::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let test_desc = RenderPipelineDescriptor {
            label: Some("test".into()),
            vertex: VertexState {
                shader: get_test().clone(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: get_test().clone(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: self.format.add_srgb_suffix(),
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            layout: vec![],
            push_constant_ranges: vec![],
        };

        let vertex_buffer = self.vertex_buffer.lock().clone();

        self.renderer.prepare_frame(|fg| {
            let mut builder = fg.create_pass_node_builder(0, "final");

            let pipeline_handle = builder.register_render_pipeline(&test_desc);

            let new_swap_chain = builder.create("swap_chain", SwapChainDescriptor);

            let vertex_buffer_desc = vertex_buffer.get_desc().clone();

            let vertex_buffer_handle =
                builder.import("vertex_buffer", vertex_buffer, vertex_buffer_desc);

            let swap_chain_read_ref = builder.read(new_swap_chain);

            let vertex_buffe_read_ref = builder.read(vertex_buffer_handle);

            builder.add_attachment(ColorAttachment::SwapChain(swap_chain_read_ref));

            builder.render(move |context| {
                context.set_render_pipeline(&pipeline_handle);

                context.set_vertex_buffer(0, vertex_buffe_read_ref);

                context.draw(0..3, 0..1);

                Ok(())
            });
        });

        self.update_pipeline_cache(dt);

        self.renderer.draw_frame();
    }
}

#[derive(Debug, Clone)]
pub struct GraphicContextParams {}

pub enum GraphicContext {
    Initialization(Box<InitializationGraphicContext>),
    Uninitialization(GraphicContextParams),
}

impl GraphicContext {
    pub fn get_params(&self) -> &GraphicContextParams {
        match &self {
            GraphicContext::Uninitialization(params) => params,
            GraphicContext::Initialization(init) => &init.params,
        }
    }

    pub fn initialization(
        &mut self,
        device: Arc<Device>,
        shader_event_receiver: Receiver<ResourceEvent>,
        format: TextureFormat,
    ) {
        *self = GraphicContext::Initialization(Box::new(InitializationGraphicContext::new(
            device,
            self.get_params().clone(),
            shader_event_receiver,
            format,
        )));
    }

    pub fn render(&mut self, dt: f32) {
        if let GraphicContext::Initialization(context) = self {
            context.render(dt)
        }
    }
}
