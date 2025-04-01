use std::ops::Range;

use crate::{
    error::{RendererError, Result},
    frame_graph::{CompiledPipelines, FGResource, ResourceBoard},
    gfx_base::{
        command_buffer::CommandBuffer,
        device::Device,
        handle::TypeHandle,
        pipeline::{PipelineCache, RenderPipeline},
        texture_view::TextureView,
    },
};

use super::{GpuRead, GpuWrite, Resource, ResourceRef, ResourceTable, SwapChain};

pub type DynRenderFn = dyn FnOnce(&mut RenderContext) -> Result<(), RendererError>;

///资源上下文
pub struct RenderContext<'a> {
    ///资源表
    resource_table: &'a mut ResourceTable,
    device: &'a Device,
    ///只读资源的全局索引
    resource_board: &'a ResourceBoard,
    pipeline_cache: &'a PipelineCache,
    pipelines: &'a CompiledPipelines,
    cb: Option<CommandBuffer>,
}

impl<'a> RenderContext<'a> {
    pub fn set_cb(&mut self, cb: CommandBuffer) {
        self.cb = Some(cb);
    }

    pub fn take_cb(&mut self) -> Option<CommandBuffer> {
        self.cb.take()
    }

    pub fn get_texture_view_with_swap_chain(&self, handle: &TypeHandle<Resource>) -> TextureView {
        let swap_chain: &SwapChain = self.resource_table.get_resource(handle).unwrap();
        swap_chain.get_texture_view()
    }

    pub fn set_render_pipeline(&mut self, handle: &TypeHandle<RenderPipeline>) {
        let handle = self.pipelines.render_pipeline_ids[handle.index()];
        if let Some(render_pipeline) = self.pipeline_cache.get_render_pipeline(&handle) {
            if let Some(cb) = self.cb.as_mut() {
                cb.set_render_pipeline(render_pipeline);
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
        resource_table: &'a mut ResourceTable,
        device: &'a Device,
        resource_board: &'a ResourceBoard,
        pipeline_cache: &'a PipelineCache,
        pipelines: &'a CompiledPipelines,
    ) -> Self {
        Self {
            resource_table,
            device,
            resource_board,
            pipeline_cache,
            pipelines,
            cb: None,
        }
    }

    pub fn get_resource_from_board<ResourceType: FGResource>(
        &self,
        name: &str,
    ) -> Option<&ResourceType> {
        if let Some(handle) = self.resource_board.get(name) {
            let handle: ResourceRef<ResourceType, GpuRead> =
                ResourceRef::new(handle.clone().into());
            self.get_resource(&handle)
        } else {
            None
        }
    }

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &ResourceRef<ResourceType, GpuRead>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(&handle.resource_handle())
    }

    pub fn get_resource_mut<ResourceType: FGResource>(
        &mut self,
        handle: &ResourceRef<ResourceType, GpuWrite>,
    ) -> Option<&mut ResourceType> {
        self.resource_table
            .get_resource_mut(&handle.resource_handle())
    }
}
