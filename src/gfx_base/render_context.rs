use crate::{
    RendererError,
    frame_graph::{Resource, ResourceBoard, FGResource},
    gfx_base::{device::Device, handle::TypeHandle},
};

use super::{render_pass::RenderPass, resource_table::ResourceTable};

pub type DynRenderFn = dyn FnOnce(&mut RenderApi) -> Result<(), RendererError>;

pub struct RenderApi<'a, 'b> {
    context: &'a mut RenderContext<'b>,
    pass: &'a mut RenderPass,
}

impl<'a, 'b> RenderApi<'a, 'b>
where
    'b: 'a,
{
    pub fn device(&self) -> &Device {
        self.context.device()
    }

    pub fn new(context: &'a mut RenderContext<'b>, pass: &'a mut RenderPass) -> Self {
        Self { context, pass }
    }

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&ResourceType> {
        self.context.get_resource(handle)
    }

    pub fn get_resouce_mut<ResourceType: FGResource>(
        &mut self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&mut ResourceType> {
        self.context.get_resouce_mut(handle)
    }
}

pub struct RenderContext<'a> {
    resource_table: &'a mut ResourceTable,
    device: &'a Device,
    resource_board: &'a ResourceBoard,
}

impl<'a> RenderContext<'a> {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn new(
        resource_table: &'a mut ResourceTable,
        device: &'a Device,
        resource_board: &'a ResourceBoard,
    ) -> Self {
        Self {
            resource_table,
            device,
            resource_board,
        }
    }

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(handle)
    }

    pub fn get_resouce_mut<ResourceType: FGResource>(
        &mut self,
        handle: &TypeHandle<Resource>,
    ) -> Option<&mut ResourceType> {
        self.resource_table.get_resouce_mut(handle)
    }
}
