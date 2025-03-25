use std::sync::Arc;

use crate::gfx_base::pipeline::PipelineCache;
use crate::gfx_base::{device::Device, transient_resource_cache::TransientResourceCache};

use crate::frame_graph::{CompiledFrameGraph, FrameGraph};

#[derive(Debug)]
pub struct Renderer {
    compiled_fg: Option<CompiledFrameGraph>,
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
    pipeline_cache: PipelineCache,
}

impl Renderer {
    pub fn pipeline_cache_mut(&mut self) -> &mut PipelineCache {
        &mut self.pipeline_cache
    }

    pub fn new(device: Arc<Device>) -> Self {
        Self {
            compiled_fg: None,
            transient_resource_cache: Default::default(),
            pipeline_cache: PipelineCache::new(device.clone()),
            device,
        }
    }

    pub fn draw_frame(&mut self) {
        let fg = match self.compiled_fg.take() {
            Some(fg) => fg,
            None => {
                return;
            }
        };

        let executing_rg = fg.begin_execute(&self.device, &mut self.transient_resource_cache);

        let retired_frame_graph = executing_rg.execute(&self.device, &self.pipeline_cache);

        retired_frame_graph.release_resources(&mut self.transient_resource_cache);
    }

    pub fn prepare_frame<PrepareFrameGraphFn>(&mut self, prepare_render_graph: PrepareFrameGraphFn)
    where
        PrepareFrameGraphFn: FnOnce(&mut FrameGraph),
    {
        let mut frame_graph = FrameGraph::default();

        prepare_render_graph(&mut frame_graph);

        self.compiled_fg = frame_graph.compile(&mut self.pipeline_cache);
    }
}
