use std::sync::Arc;

use wgpu::BufferUsages;

use crate::gfx_base::buffer::BufferInitInfo;
use crate::gfx_base::color_attachment::ColorAttachmentInfo;
use crate::gfx_base::device::Device;

use crate::frame_graph::{FrameGraph, RenderContext, SwapChainInfo, TransientResourceCache};
use crate::gfx_base::pipeline::PipelineCache;
use crate::graphic_context::Vertex;

pub struct WorldRenderer {
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
}

fn prepare(device: &Device, vertex_buffers: &Vec<Vertex>, frame_graph: &mut FrameGraph) {
    let buffer = device.create_buffer_init(BufferInitInfo {
        label: Some("vertex_buffer".into()),
        usage: BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(&vertex_buffers),
    });

    let desc = buffer.get_desc().clone();
    let vertex_buffer = Arc::new(buffer);

    let mut builder = frame_graph.create_pass_node_builder(1, "vertex");

    let swap_chain_handle = builder.create("swap_chain", SwapChainInfo);

    let swap_chain_read = builder.read(swap_chain_handle);

    builder.add_attachment_info(ColorAttachmentInfo::SwapChain(swap_chain_read));

    builder.render(|_render_context| Ok(()));
}

impl WorldRenderer {
    pub fn new(device: Arc<Device>) -> Self {
        WorldRenderer {
            device,
            transient_resource_cache: TransientResourceCache::default(),
        }
    }

    pub fn render(&mut self, pipeline_cache: &mut PipelineCache, vertex: &Vec<Vertex>) {
        let mut frame_graph = FrameGraph::default();

        prepare(&self.device, vertex, &mut frame_graph);

        frame_graph.compile();

        let mut render_context = RenderContext::new(
            &self.device,
            pipeline_cache,
            &mut self.transient_resource_cache,
        );

        frame_graph.execute(&mut render_context);
    }
}
