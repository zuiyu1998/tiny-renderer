use std::sync::Arc;

use crate::{
    error::RendererError,
    gfx_base::{device::Device, handle::TypeHandle, render_pass::RenderPassDescriptor},
};

use super::{
    DynRenderFn, FrameGraph, PassNode, RenderContext, ResourceTable, TransientResourceCache,
    VirtualResource,
};

#[derive(Default)]
pub struct DevicePass {
    logic_pass: LogicPass,
    render_pass_desc: RenderPassDescriptor,
}

impl DevicePass {
    pub fn extra(&mut self, graph: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = graph.get_pass_node(&handle);

        let resource_request_array = pass_node
            .resource_request_array
            .iter()
            .map(|handle| graph.get_resource(handle).clone())
            .collect();

        let resource_release_array = pass_node.resource_release_array.clone();

        let pass_node = graph.get_pass_node_mut(&handle);

        let render_fn = pass_node.render_fn.take();

        let logic_pass = LogicPass {
            render_fn,
            resource_request_array,
            resource_release_array,
            name: pass_node.name.clone(),
        };

        self.logic_pass = logic_pass;

        self.render_pass_desc
            .color_attachments
            .append(&mut pass_node.color_attachments);
    }

    pub fn begin(&self, render_context: &mut RenderContext) -> Result<(), RendererError> {
        self.logic_pass.request_resources(
            render_context.device,
            render_context.transient_resource_cache,
            &mut render_context.resource_table,
        );

        let mut command_buffer = render_context.device().create_command_buffer();

        let mut render_pass = render_context
            .device()
            .create_render_pass(self.render_pass_desc.clone());

        render_pass.do_init(render_context)?;

        command_buffer.begin_render_pass(render_context.device(), render_pass);
        render_context.set_cb(command_buffer);

        Ok(())
    }

    pub fn end(&self, render_context: &mut RenderContext) {
        if let Some(mut command_buffer) = render_context.take_cb() {
            command_buffer.end_render_pass();
            render_context.device().submit(vec![command_buffer]);
        }

        self.logic_pass.release_resources(
            render_context.transient_resource_cache,
            &mut render_context.resource_table,
        );
    }

    pub fn execute(&mut self, render_context: &mut RenderContext) -> Result<(), RendererError> {
        self.begin(render_context)?;

        if let Some(render_fn) = self.logic_pass.render_fn.take() {
            render_fn(render_context)?;
        }

        self.end(render_context);

        Ok(())
    }
}

#[derive(Default)]
pub struct LogicPass {
    pub render_fn: Option<Box<DynRenderFn>>,
    pub resource_release_array: Vec<TypeHandle<VirtualResource>>,
    pub resource_request_array: Vec<VirtualResource>,
    pub name: String,
}

impl LogicPass {
    pub fn request_resources(
        &self,
        device: &Arc<Device>,
        transient_resource_cache: &mut TransientResourceCache,
        resource_table: &mut ResourceTable,
    ) {
        for resource in self.resource_request_array.iter() {
            resource_table.request_resource(resource, device, transient_resource_cache);
        }
    }

    pub fn release_resources(
        &self,
        transient_resource_cache: &mut TransientResourceCache,
        resource_table: &mut ResourceTable,
    ) {
        for handle in self.resource_release_array.iter() {
            resource_table.release_resource(handle, transient_resource_cache);
        }
    }
}
