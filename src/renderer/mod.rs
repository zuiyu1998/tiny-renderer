pub mod resource;

use crate::frame_graph::TemporalFrameGraph;
use crate::render_backend::RenderBackend;

pub struct WorldRenderer {}

pub struct Renderer {
    backend: RenderBackend,
    world_renderer: WorldRenderer,
    frame_graph: TemporalFrameGraph,
}

impl Renderer {
    pub fn new(backend: RenderBackend) -> Self {
        Self {
            world_renderer: WorldRenderer {},
            backend,
            frame_graph: Default::default(),
        }
    }

    pub fn prepare(&mut self) {}

    pub fn render(&mut self) {
        self.prepare();

        self.frame_graph.compile();

        self.frame_graph.execute();
    }
}
