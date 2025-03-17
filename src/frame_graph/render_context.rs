use std::sync::Arc;

use crate::{
    RendererError,
    frame_graph::{CompiledPipelines, FGResource, ResourceBoard},
    gfx_base::{
        color_attachment::ColorAttachmentView,
        device::Device,
        handle::TypeHandle,
        pipeline::{PipelineCache, RenderPipeline},
        render_pass::{RenderPass, RenderPassDescriptor},
        resource_table::ResourceTable,
        texture_view::TextureView,
    },
};

use super::{GpuRead, GpuWrite, Resource, ResourceRef, SwapChain};

pub type DynRenderFn = dyn FnOnce(&mut RenderPass, &mut RenderContext) -> Result<(), RendererError>;

///资源上下文
pub struct RenderContext<'a> {
    ///资源表
    resource_table: &'a mut ResourceTable,
    device: &'a Device,
    ///只读资源的全局索引
    resource_board: &'a ResourceBoard,
    pipeline_cache: &'a PipelineCache,
    pipelines: &'a CompiledPipelines,
}

impl<'a> RenderContext<'a> {
    pub fn initialization_render_pass_descriptor(&self, desc: &mut RenderPassDescriptor) {
        for color_attachment in desc.color_attachments.iter_mut() {
            let view = match &color_attachment.view {
                ColorAttachmentView::Initialization(_) => {
                    continue;
                }
                ColorAttachmentView::Uninitialization(handle) => {
                    self.get_texture_view_with_swap_chain(handle)
                }
            };

            color_attachment.view = ColorAttachmentView::Initialization(view);
        }
    }

    pub fn get_texture_view_with_swap_chain(&self, handle: &TypeHandle<Resource>) -> TextureView {
        let swap_chain: &SwapChain = self.resource_table.get_resource(handle).unwrap();
        swap_chain.get_texture_view()
    }

    pub fn get_render_pipeline(&self, handle: &TypeHandle<RenderPipeline>) -> Arc<RenderPipeline> {
        let handle = self.pipelines.render_pipelines[handle.index()];
        self.pipeline_cache.get_render_pipeline(&handle)
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
