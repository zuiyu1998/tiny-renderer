use std::{ops::Range, sync::Arc};

use crate::{
    error::{RendererError, Result},
    frame_graph::Resource,
    gfx_base::{
        BindGroupRef,
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

    pub fn set_bind_group(&mut self, index: u32, bind_group: &BindGroupRef) {
        if let Some(cb) = self.cb.as_mut() {
            let info = bind_group.get_info(self.device, &self.resource_table);
            let bind_group = self.device.create_bind_group(info);

            cb.set_bind_group(index, &bind_group);
        }
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

    pub fn set_index_buffer(
        &mut self,
        handle: ResourceNodeRef<Buffer, GpuRead>,
        index_format: wgpu::IndexFormat,
    ) {
        if let Some(buffer) = self.resource_table.get_resource(&handle.resource_handle()) {
            if let Some(cb) = self.cb.as_mut() {
                cb.set_index_buffer(buffer, index_format);
            }
        }
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        if let Some(cb) = self.cb.as_mut() {
            cb.draw(vertices, instances);
        }
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        if let Some(cb) = self.cb.as_mut() {
            cb.draw_indexed(indices, base_vertex, instances);
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
