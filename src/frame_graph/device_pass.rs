use crate::gfx_base::{
    handle::TypeHandle,
    render_context::{DynRenderFn, RenderApi, RenderContext},
    render_pass::{RenderPass, RenderPassDescriptor},
};

use super::{FrameGraph, PassNode, Resource};

pub struct DevicePass {
    logic_pass: LogicPass,
    render_pass_desc: Option<RenderPassDescriptor>,
    render_pass: Option<RenderPass>,
}

impl Default for DevicePass {
    fn default() -> Self {
        DevicePass {
            logic_pass: Default::default(),
            render_pass_desc: Some(RenderPassDescriptor::default()),
            render_pass: None,
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

        render_pass_desc.initialization(render_context);

        let render_pass = render_context.device().create_render_pass(render_pass_desc);

        self.render_pass = Some(render_pass);

        let mut render_api = RenderApi::new(render_context, self.render_pass.as_mut().unwrap());

        if let Some(render_fn) = self.logic_pass.render_fn.take() {
            if let Err(e) = render_fn(&mut render_api) {
                println!("render_fn error: {}", e)
            }
        }
        if let Some(mut render_pass) = self.render_pass.take() {
            let command_buffer = render_pass.finish();
            render_context.device().submit(vec![command_buffer]);
        }
    }
}

#[derive(Default)]
pub struct LogicPass {
    pub render_fn: Option<Box<DynRenderFn>>,
    pub resource_release_array: Vec<TypeHandle<Resource>>,
}
