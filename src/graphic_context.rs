use crate::{
    frame_graph::SwapChain,
    gfx_base::{
        color_attachment::{ColorAttachment, ColorAttachmentView},
        pipeline::RenderPipelineDescriptor,
    },
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

            let pipeline_handle = builder.register_render_pipeline(RenderPipelineDescriptor {
                label: Some("test".to_string()),
            });

            let new_swap_chain = builder.read_from_board::<SwapChain>("swap_chain").unwrap();

            builder.add_attachment(ColorAttachment {
                view: ColorAttachmentView::new(new_swap_chain.resource_handle()),
            });

            let pipeline_handle_clone = pipeline_handle.clone();
            builder.render(move |render_pass, api| {
                let pipeline = api.get_render_pipeline(&pipeline_handle_clone);
                render_pass.set_render_pipeline(&pipeline);
                render_pass.draw(0..3, 0..1);

                Ok(())
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
