use crate::gfx_base::{handle::TypeHandle, render_pass::RenderPassDescriptor};

use super::{DynRenderFn, FrameGraph, PassNode, RenderContext, Resource};

pub struct DevicePass {
    logic_pass: LogicPass,
    render_pass_desc: RenderPassDescriptor,
}

impl Default for DevicePass {
    fn default() -> Self {
        DevicePass {
            logic_pass: Default::default(),
            render_pass_desc: RenderPassDescriptor::default(),
        }
    }
}

impl DevicePass {
    pub fn extra(&mut self, graph: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = graph.get_pass_node_mut(&handle);
        self.logic_pass = pass_node.take();

        self.render_pass_desc
            .color_attachments
            .append(&mut pass_node.color_attachments);
    }

    pub fn begin(&self, render_context: &mut RenderContext) {
        let mut command_buffer = render_context.device().create_command_buffer();
        let mut render_pass = render_context
            .device()
            .create_render_pass(self.render_pass_desc.clone());
        render_pass.do_init(render_context);
        command_buffer.begin_render_pass(render_context.device(), render_pass);
        render_context.set_cb(command_buffer);
    }

    pub fn end(&self, render_context: &mut RenderContext) {
        if let Some(mut command_buffer) = render_context.take_cb() {
            command_buffer.end_render_pass();
            render_context.device().submit(vec![command_buffer]);
        }
    }

    pub fn execute(&mut self, render_context: &mut RenderContext) {
        self.begin(render_context);

        if let Some(render_fn) = self.logic_pass.render_fn.take() {
            if let Err(e) = render_fn(render_context) {
                println!("render_fn error: {}", e)
            }
        }

        self.end(render_context);
    }
}

#[derive(Default)]
pub struct LogicPass {
    pub render_fn: Option<Box<DynRenderFn>>,
    pub resource_release_array: Vec<TypeHandle<Resource>>,
}
