pub mod resource;

use resource::{BufferDescriptor, BufferUsages};

use crate::frame_graph::TemporalFrameGraph;
use crate::render_backend::RenderBackend;

pub struct WorldRenderer {}

impl WorldRenderer {
    pub fn prepare(&self, frame_graph: &mut TemporalFrameGraph) {
        let mut builder = frame_graph.add_pass_node("world renderer", None);

        let vertex = builder
            .put_buffer(
                "world renderer vetex",
                BufferDescriptor {
                    label: "world renderer vetex".to_string(),
                    size: 50,
                    usage: BufferUsages::VERTEX,
                    mapped_at_creation: true,
                },
            )
            .unwrap();
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

    pub fn prepare(&mut self) {}

    pub fn render(&mut self) {
        self.prepare();

        self.frame_graph.compile();

        self.frame_graph.execute();
    }
}
