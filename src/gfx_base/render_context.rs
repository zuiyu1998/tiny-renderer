use std::sync::Arc;

use crate::{
    RendererError,
    frame_graph::{
        CompiledPipelines, FGResource, GpuViewType, Resource, ResourceBoard, ResourceRef,
    },
    gfx_base::{device::Device, handle::TypeHandle},
};

use super::{
    pipeline::{PipelineCache, RenderPipeline},
    render_pass::RenderPass,
    resource_table::ResourceTable,
};

pub type DynRenderFn = dyn FnOnce(&mut RenderApi) -> Result<(), RendererError>;

pub struct RenderApi<'a, 'b> {
    context: &'a mut RenderContext<'b>,
    pass: &'a mut RenderPass,
}

impl<'a, 'b> RenderApi<'a, 'b>
where
    'b: 'a,
{
    pub fn get_render_pipeline(&self, handle: &TypeHandle<RenderPipeline>) -> Arc<RenderPipeline> {
        self.context.get_render_pipeline(handle)
    }

    pub fn get_render_pass_mut(&mut self) -> &mut RenderPass {
        self.pass
    }

    pub fn device(&self) -> &Device {
        self.context.device()
    }

    pub fn new(context: &'a mut RenderContext<'b>, pass: &'a mut RenderPass) -> Self {
        Self { context, pass }
    }

    pub fn get_resource<ResourceType: FGResource, ViewType>(
        &self,
        handle: &ResourceRef<ResourceType, ViewType>,
    ) -> Option<&ResourceType>
    where
        ViewType: GpuViewType,
    {
        if !ViewType::IS_WRITABLE {
            self.context.get_resource(&handle.handle().resource_handle)
        } else {
            None
        }
    }

    pub fn get_resource_mut<ResourceType: FGResource, ViewType>(
        &mut self,
        handle: &ResourceRef<ResourceType, ViewType>,
    ) -> Option<&mut ResourceType>
    where
        ViewType: GpuViewType,
    {
        if ViewType::IS_WRITABLE {
            self.context
                .get_resource_mut(&handle.handle().resource_handle)
        } else {
            None
        }
    }
}

pub struct RenderContext<'a> {
    resource_table: &'a mut ResourceTable,
    device: &'a Device,
    resource_board: &'a ResourceBoard,
    pipeline_cache: &'a PipelineCache,
    pipelines: &'a CompiledPipelines,
}

impl<'a> RenderContext<'a> {
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

    pub fn get_resource_from_name<ResourceType: FGResource>(
        &self,
        name: &str,
    ) -> Option<&ResourceType> {
        if let Some(handle) = self.resource_board.get(name) {
            self.resource_table.get_resource(&handle.resource_handle)
        } else {
            None
        }
    }

    pub fn get_resource_mut_from_name<ResourceType: FGResource>(
        &mut self,
        name: &str,
    ) -> Option<&mut ResourceType> {
        if let Some(handle) = self.resource_board.get(name) {
            self.resource_table
                .get_resource_mut(&handle.resource_handle)
        } else {
            None
        }
    }

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(handle)
    }

    pub fn get_resource_mut<ResourceType: FGResource>(
        &mut self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&mut ResourceType> {
        self.resource_table.get_resource_mut(handle)
    }
}
