pub mod meshes;
pub mod render_camera;
pub mod schedule;

pub use meshes::*;
pub use render_camera::*;
pub use schedule::*;

use std::sync::Arc;

use crate::gfx_base::device::Device;

use crate::frame_graph::{FrameGraph, RenderContext, TransientResourceCache};
use crate::gfx_base::pipeline::PipelineCache;

pub trait Renderer {
    fn prepare(&self, context: &mut FrameGraphContext);
}

pub struct FrameGraphContext<'a> {
    pub device: &'a Device,
    pub camera: &'a RenderCamera,
    pub frame_graph: &'a mut FrameGraph,
    pub pipeline_cache: &'a PipelineCache,
}

pub struct WorldRenderer {
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
    schedules: RenderSchedules,
}

impl WorldRenderer {
    pub fn new(pipeline_cache: &mut PipelineCache) -> Self {
        WorldRenderer {
            device: pipeline_cache.device.clone(),
            transient_resource_cache: TransientResourceCache::default(),
            schedules: RenderSchedules::new(pipeline_cache),
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

            let mut context = FrameGraphContext {
                device: &self.device,
                camera,
                frame_graph: &mut frame_graph,
                pipeline_cache,
            };

            mesh_material.prepare(&mut context);

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
