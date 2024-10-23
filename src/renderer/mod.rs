pub mod resource;

use resource::{Buffer, BufferDescriptor, BufferUsages, SwapchainImage, SwapchainImageDescriptor};
use wgpu::TextureFormat;

use crate::frame_graph::TemporalFrameGraph;
use crate::render_backend::RenderBackend;
use crate::windows::{WindowState, Windows};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct WorldRenderer {
    surface_format: TextureFormat,
    vertex_data: Vec<Vertex>,
}

impl WorldRenderer {
    pub fn prepare(&self, frame_graph: &mut TemporalFrameGraph) {
        let mut builder = frame_graph.add_pass_node("world pass node", None);

        let swap_image_handle = builder.create(SwapchainImageDescriptor {});

        let vertex = builder
            .put_buffer(
                "world renderer vertex",
                BufferDescriptor {
                    label: "world renderer vertex".to_string(),
                    size: 500,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                },
            )
            .unwrap();

        let vertex_writer = builder.write(&vertex);

        let vertex_reader = builder.read(&vertex);
        let swap_image_ref = builder.read(&swap_image_handle);

        let surface_format = self.surface_format;

        let vertex_data = self.vertex_data.clone();

        builder.render(move |api| {
            let swap_image: &SwapchainImage = api
                .render_context
                .get_render_resource(&swap_image_ref.handle)?;

            let swap_image_view =
                swap_image
                    .texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor {
                        ..Default::default()
                    });

            let vertex: &Buffer = api
                .render_context
                .get_render_resource(&vertex_writer.handle)?;

            api.render_context.backend.queue.write_buffer(
                &vertex.render_buffer,
                0,
                bytemuck::cast_slice(&vertex_data),
            );

            let shader = api.render_context.backend.device.create_shader_module(
                wgpu::ShaderModuleDescriptor {
                    label: Some("Shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
                },
            );

            let render_pipeline_layout = api.render_context.backend.device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                },
            );

            let render_pipeline = api.render_context.backend.device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                        compilation_options: Default::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: surface_format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: Default::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                        // or Features::POLYGON_MODE_POINT
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
                    // Useful for optimizing shader compilation on Android
                    cache: None,
                },
            );

            let mut render_pass = api.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &swap_image_view,
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
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            render_pass.set_pipeline(&render_pipeline);

            render_pass.set_vertex_buffer(0, &vertex_reader);

            render_pass.draw(0..3, 0..1);
            Ok(())
        });
    }
}

pub struct Renderer {
    world_renderer: WorldRenderer,
    frame_graph: TemporalFrameGraph,
}

impl Renderer {
    pub fn new(backend: RenderBackend, window: &WindowState) -> Self {
        let frame_graph = TemporalFrameGraph::new(backend.clone());

        Self {
            world_renderer: WorldRenderer {
                surface_format: window.surface_format,
                vertex_data: vec![
                    Vertex {
                        position: [-0.0868241, 0.49240386, 0.0],
                        color: [0.5, 0.0, 0.5],
                    }, // A
                    Vertex {
                        position: [-0.49513406, 0.06958647, 0.0],
                        color: [0.5, 0.0, 0.5],
                    }, // B
                    Vertex {
                        position: [-0.21918549, -0.44939706, 0.0],
                        color: [0.5, 0.0, 0.5],
                    }, // C
                    Vertex {
                        position: [0.35966998, -0.3473291, 0.0],
                        color: [0.5, 0.0, 0.5],
                    }, // D
                    Vertex {
                        position: [0.44147372, 0.2347359, 0.0],
                        color: [0.5, 0.0, 0.5],
                    }, // E
                ],
            },
            frame_graph,
        }
    }

    fn prepare(&mut self) {
        self.world_renderer.prepare(&mut self.frame_graph);
    }

    pub fn render(&mut self, windows: &mut Windows) {
        println!("renderer render");

        self.prepare();

        self.frame_graph.compile();

        let swapchain_images = windows.get_current_swapchain_images();
        self.frame_graph.execute(swapchain_images);
    }
}
