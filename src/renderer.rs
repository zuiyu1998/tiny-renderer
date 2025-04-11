use std::sync::Arc;

use crate::gfx_base::device::Device;

use crate::frame_graph::{FrameGraph, RenderContext, TransientResourceCache};
use crate::gfx_base::pipeline::PipelineCache;

pub struct WorldRenderer {
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
}

impl WorldRenderer {
    pub fn new(device: Arc<Device>) -> Self {
        WorldRenderer {
            device,
            transient_resource_cache: TransientResourceCache::default(),
        }
    }

    pub fn render(&mut self, pipeline_cache: &mut PipelineCache) {
        let mut frame_graph = FrameGraph::default();

        frame_graph.compile();

        let mut render_context = RenderContext::new(
            &self.device,
            pipeline_cache,
            &mut self.transient_resource_cache,
        );

        frame_graph.execute(&mut render_context);
    }
}
