pub mod schedule;
pub mod renderer;

use renderer::Renderer;
pub use schedule::*;

use std::sync::Arc;

use wgpu::BufferUsages;

use crate::gfx_base::buffer::BufferInitInfo;
use crate::gfx_base::device::Device;

use crate::frame_graph::{FrameGraph, RenderContext, TransientResourceCache};
use crate::gfx_base::ColorAttachmentInfo;
use crate::gfx_base::pipeline::PipelineCache;
use crate::gfx_base::texture_view::TextureView;
use crate::graphic_context::MeshMaterial;

pub struct FrameGraphContext<'a> {
    pub device: &'a Device,
    pub camera: &'a RenderCamera,
    pub frame_graph: &'a mut FrameGraph,
}

pub enum RenderTarget {
    Window(Arc<TextureView>),
}

pub struct RenderCamera {
    pub render_target: RenderTarget,
}

impl RenderCamera {
    pub fn get_texture_view(&self) -> Arc<TextureView> {
        match &self.render_target {
            RenderTarget::Window(texture_view) => texture_view.clone(),
        }
    }
}

pub struct WorldRenderer {
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
    schedules: RenderSchedules,
}

fn prepare(
    device: &Device,
    camera: &RenderCamera,
    frame_graph: &mut FrameGraph,
    mesh_material: &MeshMaterial,
    pipeline_cache: &mut PipelineCache,
) {
    if mesh_material.id.is_none() {
        return;
    }
    let pipeline_id = mesh_material.id.clone().unwrap();

    if pipeline_cache.get_render_pipeline(&pipeline_id).is_none() {
        return;
    }

    let buffer = device.create_buffer_init(BufferInitInfo {
        label: Some("vertex_buffer".into()),
        usage: BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(&mesh_material.vertex_buffers),
    });

    let vertex_buffer = Arc::new(buffer);

    let mut builder = frame_graph.create_pass_node_builder(2, "vertex");

    let swap_chain_handle = builder.import("swap_chain", camera.get_texture_view());
    let swap_chain_read = builder.read(swap_chain_handle);

    let vertex_buffer_handle = builder.import("vertex_buffer", vertex_buffer);
    let vertex_buffer_read = builder.read(vertex_buffer_handle);

    builder.add_attachment_info(ColorAttachmentInfo::SwapChain(swap_chain_read));

    builder.render(move |render_context| {
        render_context.set_render_pipeline(&pipeline_id);
        render_context.set_vertex_buffer(0, vertex_buffer_read);
        render_context.draw(0..3, 0..1);

        Ok(())
    });
}

impl WorldRenderer {
    pub fn new(device: Arc<Device>) -> Self {
        WorldRenderer {
            device,
            transient_resource_cache: TransientResourceCache::default(),
            schedules: RenderSchedules::default(),
        }
    }

    pub fn render(
        &mut self,
        pipeline_cache: &mut PipelineCache,
        cameras: &[RenderCamera],
        mesh_material: &MeshMaterial,
    ) {
        for camera in cameras.iter() {
            let mut frame_graph = FrameGraph::default();

            prepare(
                &self.device,
                camera,
                &mut frame_graph,
                mesh_material,
                pipeline_cache,
            );

            let mut context = FrameGraphContext {
                device: &self.device,
                camera,
                frame_graph: &mut frame_graph,
            };

            self.schedules.prepare(&mut context);

            frame_graph.compile();

            let mut render_context = RenderContext::new(
                &self.device,
                pipeline_cache,
                &mut self.transient_resource_cache,
            );

            frame_graph.execute(&mut render_context);
        }
    }
}
