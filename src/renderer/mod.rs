pub mod resource;

use resource::{Buffer, BufferDescriptor, BufferUsages, SwapchainImage, SwapchainImageDescriptor};

use crate::frame_graph::TemporalFrameGraph;
use crate::render_backend::RenderBackend;
use crate::windows::Windows;

pub struct WorldRenderer {}

impl WorldRenderer {
    pub fn prepare(&self, frame_graph: &mut TemporalFrameGraph) {
        let mut builder = frame_graph.add_pass_node("world pass node", None);

        let swap_image_handle = builder.create(SwapchainImageDescriptor {});

        let vertex = builder
            .put_buffer(
                "world renderer vertex",
                BufferDescriptor {
                    label: "world renderer vertex".to_string(),
                    size: 50,
                    usage: BufferUsages::VERTEX,
                    mapped_at_creation: false,
                },
            )
            .unwrap();

        let vertex_writer = builder.write(&vertex);

        let vertex_reader = builder.read(&vertex);
        let swap_image_ref = builder.read(&swap_image_handle);

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

            api.render_context
                .backend
                .queue
                .write_buffer(&vertex.render_buffer, 0, &[]);

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

            render_pass.set_vertex_buffer(0, &vertex_reader);

            Ok(())
        });
    }
}

pub struct Renderer {
    backend: RenderBackend,
    world_renderer: WorldRenderer,
    frame_graph: TemporalFrameGraph,
}

impl Renderer {
    pub fn new(backend: RenderBackend) -> Self {
        let frame_graph = TemporalFrameGraph::new(backend.clone());

        Self {
            world_renderer: WorldRenderer {},
            backend,
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
