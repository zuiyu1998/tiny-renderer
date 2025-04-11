use std::{ops::Range, sync::Arc};

use crate::{
    error::{RendererError, Result},
    frame_graph::Resource,
    gfx_base::{
        buffer::Buffer,
        command_buffer::CommandBuffer,
        device::Device,
        pipeline::{CachedRenderPipelineId, PipelineCache},
    },
};

use super::{GpuRead, ResourceNodeRef, ResourceTable, TransientResourceCache};

pub type DynRenderFn = dyn FnOnce(&mut RenderContext) -> Result<(), RendererError>;

///资源上下文
pub struct RenderContext<'a> {
    ///资源表
    pub resource_table: ResourceTable,
    pub device: &'a Arc<Device>,
    pub pipeline_cache: &'a PipelineCache,
    cb: Option<CommandBuffer>,
    pub transient_resource_cache: &'a mut TransientResourceCache,
}

impl<'a> RenderContext<'a> {
    pub fn set_cb(&mut self, cb: CommandBuffer) {
        self.cb = Some(cb);
    }

    pub fn take_cb(&mut self) -> Option<CommandBuffer> {
        self.cb.take()
    }

    pub fn set_render_pipeline(&mut self, id: &CachedRenderPipelineId) {
        if let Some(pipeline) = self.pipeline_cache.get_render_pipeline(id) {
            if let Some(cb) = self.cb.as_mut() {
                cb.set_render_pipeline(pipeline);
            }
        }
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, handle: ResourceNodeRef<Buffer, GpuRead>) {
        if let Some(buffer) = self.resource_table.get_resource(&handle.resource_handle()) {
            if let Some(cb) = self.cb.as_mut() {
                cb.set_vertex_buffer(slot, buffer);
            }
        }
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        if let Some(cb) = self.cb.as_mut() {
            cb.draw(vertices, instances);
        }
    }

    pub fn device(&self) -> &Device {
        self.device
    }

    pub fn new(
        device: &'a Arc<Device>,
        pipeline_cache: &'a PipelineCache,
        transient_resource_cache: &'a mut TransientResourceCache,
    ) -> Self {
        Self {
            resource_table: Default::default(),
            device,
            pipeline_cache,
            cb: None,
            transient_resource_cache,
        }
    }

    pub fn get_resource<ResourceType: Resource>(
        &self,
        handle: &ResourceNodeRef<ResourceType, GpuRead>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(&handle.resource_handle())
    }
}
