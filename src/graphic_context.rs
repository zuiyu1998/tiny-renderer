use crate::{
    frame_graph::SwapChain,
    gfx_base::color_attachment::{ColorAttachment, ColorAttachmentView},
    renderer::Renderer,
};

pub struct InitializationGraphicContext {
    renderer: Renderer,
    params: GraphicContextParams,
}

impl InitializationGraphicContext {
    fn render(&mut self) {
        self.renderer.prepare_frame(|fg| {
            let mut builder = fg.create_pass_node_builder(0, "final");

            let new_swap_chain = builder.read_from_board::<SwapChain>("swap_chain").unwrap();

            builder.add_attachment(ColorAttachment {
                view: ColorAttachmentView::new(new_swap_chain.handle().resource_handle.clone()),
            });
        });
        self.renderer.draw_frame();
    }
}

#[derive(Debug, Clone)]
pub struct GraphicContextParams {}

pub enum GraphicContext {
    Initialization(Box<InitializationGraphicContext>),
    Uninitialization(GraphicContextParams),
}

impl GraphicContext {
    pub fn get_params(&self) -> &GraphicContextParams {
        match &self {
            GraphicContext::Uninitialization(params) => params,
            GraphicContext::Initialization(init) => &init.params,
        }
    }

    pub fn initialization(&mut self, renderer: Renderer) {
        *self = GraphicContext::Initialization(Box::new(InitializationGraphicContext {
            renderer,
            params: self.get_params().clone(),
        }));
    }

    pub fn render(&mut self) {
        if let GraphicContext::Initialization(context) = self {
            context.render()
        }
    }
}
