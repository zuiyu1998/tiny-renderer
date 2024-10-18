pub mod resource;

use crate::frame_graph::FrameGraph;
use crate::render_backend::RenderBackend;

pub struct WorldRenderer {}

pub struct Renderer {
    backend: RenderBackend,
    world_renderer: WorldRenderer,
    fg: FrameGraph,
}

impl Renderer {
    pub fn new(backend: RenderBackend) -> Self {
        Self {
            world_renderer: WorldRenderer {},
            backend,
            fg: FrameGraph::default(),
        }
    }

    pub fn prepare(&mut self) {}

    pub fn render(&mut self) {
        self.prepare();

        self.fg.compile();

        self.fg.execute();
    }
}
