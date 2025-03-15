use std::sync::Arc;

use crate::{
    device::Device,
    renderer::Renderer,
    swap_chain::{SwapChain, SwapChainDescriptor},
};

pub struct InitializationGraphicContext {
    renderer: Renderer,
    device: Arc<Device>,
}

impl InitializationGraphicContext {
    fn render(&mut self) {
        self.renderer.prepare_frame(|fg| {
            let mut builder = fg.create_pass_node_builder(0, "final");
            let swap_chain = builder.create("swap_chain", SwapChainDescriptor);

            let new_swap_chain = builder.write(swap_chain.resource_node_handle);

            let new_swap_chain_resource_handle = new_swap_chain.resource_handle.clone();

            builder.render(move |table| {
                if let Some(swap_chain) =
                    table.get_resouce_mut::<SwapChain>(&new_swap_chain_resource_handle)
                {
                    swap_chain.present();
                }

                Ok(())
            })
        });
        self.renderer.draw_frame();
    }
}

pub enum GraphicContext {
    Initialization(InitializationGraphicContext),
    Uninitialization,
}

impl GraphicContext {
    pub fn initialization(&mut self, device: Arc<Device>, renderer: Renderer) {
        *self = GraphicContext::Initialization(InitializationGraphicContext { renderer, device });
    }

    pub fn render(&mut self) {
        if let GraphicContext::Initialization(context) = self {
            context.render()
        }
    }
}
