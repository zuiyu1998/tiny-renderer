use std::sync::Arc;

use crate::gfx_base::pipeline::PipelineCache;
use crate::gfx_base::{device::Device, transient_resource_cache::TransientResourceCache};

use crate::frame_graph::{
    CompiledFrameGraph, ExecutingFrameGraph, FrameGraph, SwapChain, SwapChainDescriptor,
};

pub struct Renderer {
    compiled_fg: Option<CompiledFrameGraph>,
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
    pipeline_cache: PipelineCache,
    swap_chain: Option<Arc<SwapChain>>,
}

impl Renderer {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            compiled_fg: None,
            transient_resource_cache: Default::default(),
            swap_chain: None,
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

        let mut executing_rg: ExecutingFrameGraph;
        {
            executing_rg = fg.begin_execute(
                &self.device,
                &mut self.transient_resource_cache,
                &mut self.pipeline_cache,
            );

            executing_rg.execute(&self.device, &self.pipeline_cache);
        }

        if let Some(swap_chain) = self.swap_chain.take() {
            swap_chain.present();
        }
    }

    pub fn prepare_frame<PrepareFrameGraphFn>(&mut self, prepare_render_graph: PrepareFrameGraphFn)
    where
        PrepareFrameGraphFn: FnOnce(&mut FrameGraph),
    {
        let mut frame_graph = FrameGraph::default();

        let desc = SwapChainDescriptor {};
        let swap_chain = Arc::new(self.device.create_swap_chain(desc.clone()));

        frame_graph.import("swap_chain", swap_chain.clone(), desc);

        self.swap_chain = Some(swap_chain);

        prepare_render_graph(&mut frame_graph);

        self.compiled_fg = frame_graph.compile();
    }
}
