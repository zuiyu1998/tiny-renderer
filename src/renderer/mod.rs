pub mod resource;

use crate::frame_graph::TemporalFrameGraph;
use crate::render_backend::RenderBackend;

pub struct WorldRenderer {}

impl WorldRenderer {
    pub fn prepare(&self, frame_graph: &mut TemporalFrameGraph) {
        let builder = frame_graph.add_pass_node("world renderer", None);

        // let d = builder.put_buffer("world renderer vetex", descriptor);
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
