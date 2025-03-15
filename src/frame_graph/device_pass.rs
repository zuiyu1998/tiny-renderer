use crate::gfx_base::{device::Device, handle::TypeHandle};

use super::{DynRenderFn, FrameGraph, PassNode, Resource};

#[derive(Default)]
pub struct DevicePass {
    logic_pass: LogicPass,
}

impl DevicePass {
    pub fn extra(&mut self, graph: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = graph.get_pass_node_mut(&handle);
        self.logic_pass = pass_node.take();
    }

    pub fn execute(&mut self, device: &Device) {
        println!("[DevicePass] execute")
    }
}

#[derive(Default)]
pub struct LogicPass {
    pub render_fn: Option<Box<DynRenderFn>>,
    pub resource_release_array: Vec<TypeHandle<Resource>>,
}
