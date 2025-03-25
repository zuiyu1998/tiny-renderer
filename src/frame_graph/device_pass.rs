use std::fmt::Debug;

use crate::gfx_base::{handle::TypeHandle, render_pass::RenderPassDescriptor};

use super::{DynRenderFn, FrameGraph, PassNode, RenderContext, Resource};

#[derive(Debug)]
pub struct DevicePass {
    logic_pass: LogicPass,
    render_pass_desc: Option<RenderPassDescriptor>,
}

impl Default for DevicePass {
    fn default() -> Self {
        DevicePass {
            logic_pass: Default::default(),
            render_pass_desc: Some(RenderPassDescriptor::default()),
        }
    }
}

impl DevicePass {
    pub fn extra(&mut self, graph: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = graph.get_pass_node_mut(&handle);
        self.logic_pass = pass_node.take();

        self.render_pass_desc
            .as_mut()
            .unwrap()
            .color_attachments
            .append(&mut pass_node.color_attachments);
    }

    pub fn execute(&mut self, render_context: &mut RenderContext) {
        let mut render_pass_desc = self.render_pass_desc.take().unwrap();

        render_context.initialization_render_pass_descriptor(&mut render_pass_desc);

        let mut render_pass = render_context.device().create_render_pass(render_pass_desc);

        if let Some(render_fn) = self.logic_pass.render_fn.take() {
            if let Err(e) = render_fn(&mut render_pass, render_context) {
                println!("render_fn error: {}", e)
            }
        }
        let mut command_buffer = render_context.device().create_command_buffer();
        command_buffer.finish(render_pass);
        render_context.device().submit(vec![command_buffer]);
    }
}

#[derive(Default)]
pub struct LogicPass {
    pub render_fn: Option<Box<DynRenderFn>>,
    pub resource_release_array: Vec<TypeHandle<Resource>>,
}

impl Debug for LogicPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogicPass")
            .field("render_fn", &self.render_fn.is_some())
            .field("resource_release_array", &self.resource_release_array)
            .finish()
    }
}
