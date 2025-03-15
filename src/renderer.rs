use std::sync::Arc;

use crate::{
    device::Device,
    frame_graph::{CompiledFrameGraph, ExecutingFrameGraph, FrameGraph},
    transient_resource_cache::TransientResourceCache,
};

pub struct Renderer {
    compiled_fg: Option<CompiledFrameGraph>,
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
}

impl Renderer {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            compiled_fg: None,
            device,
            transient_resource_cache: Default::default(),
        }
    }

    pub fn draw_frame(&mut self) {
        let fg = match self.compiled_fg.take() {
            Some(fg) => fg,
            None => {
                return;
            }
        };

        let mut executing_rg: ExecutingFrameGraph;
        {
            executing_rg = fg.begin_execute(&self.device, &mut self.transient_resource_cache);

            executing_rg.execute(&self.device);
        }
    }

    pub fn prepare_frame<PrepareFrameGraphFn>(&mut self, prepare_render_graph: PrepareFrameGraphFn)
    where
        PrepareFrameGraphFn: FnOnce(&mut FrameGraph),
    {
        let mut frame_graph = FrameGraph::default();

        prepare_render_graph(&mut frame_graph);

        self.compiled_fg = frame_graph.compile();
    }
}
