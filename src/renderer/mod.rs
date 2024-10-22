pub mod resource;

use resource::{Buffer, BufferDescriptor, BufferUsages};

use crate::frame_graph::TemporalFrameGraph;
use crate::render_backend::RenderBackend;

pub struct WorldRenderer {}

impl WorldRenderer {
    pub fn prepare(&self, frame_graph: &mut TemporalFrameGraph) {
        let mut builder = frame_graph.add_pass_node("world pass node", None);

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

        builder.read(&vertex);

        builder.render(move |render_context| {
            println!("resources len: {}", render_context.resources.len());

            // let mut render_pass = render_context.begin_render_pass(&wgpu::RenderPassDescriptor {
            //     label: Some("Render Pass"),
            //     // color_attachments: &[
            //     //     // This is what @location(0) in the fragment shader targets
            //     //     Some(wgpu::RenderPassColorAttachment {
            //     //         view: &view,
            //     //         resolve_target: None,
            //     //         ops: wgpu::Operations {
            //     //             load: wgpu::LoadOp::Clear(wgpu::Color {
            //     //                 r: 0.1,
            //     //                 g: 0.2,
            //     //                 b: 0.3,
            //     //                 a: 1.0,
            //     //             }),
            //     //             store: wgpu::StoreOp::Store,
            //     //         },
            //     //     }),
            //     // ],
            //     depth_stencil_attachment: None,
            //     ..Default::default()
            // });

            // let vertex: &Buffer = render_context.get_render_resource(&vertex_writer.handle)?;

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

    pub fn render(&mut self) {
        println!("renderer render");

        self.prepare();

        self.frame_graph.compile();

        self.frame_graph.execute();
    }
}
