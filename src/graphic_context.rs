use std::sync::{Arc, mpsc::Receiver};

use fyrox_resource::event::ResourceEvent;
use wgpu::{ColorTargetState, TextureFormat};

use crate::{
    build_in::get_test,
    frame_graph::SwapChainDescriptor,
    gfx_base::{
        color_attachment::{ColorAttachment, ColorAttachmentView},
        device::Device,
        pipeline::{FragmentState, RenderPipelineDescriptor, VertexState},
        shader::Shader,
    },
    renderer::Renderer,
};

pub struct InitializationGraphicContext {
    renderer: Renderer,
    params: GraphicContextParams,
    shader_event_receiver: Receiver<ResourceEvent>,
    format: TextureFormat,
}

impl InitializationGraphicContext {
    fn update_pipeline_cache(&mut self, dt: f32) {
        while let Ok(event) = self.shader_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource)
            | ResourceEvent::Reloaded(resource)
            | ResourceEvent::Added(resource) = event
            {
                if let Some(shader) = resource.try_cast::<Shader>() {
                    self.renderer.pipeline_cache_mut().remove(&shader);
                    self.renderer.pipeline_cache_mut().set_shader(&shader);
                }
            }
        }

        self.renderer.pipeline_cache_mut().update(dt);
    }

    fn render(&mut self, dt: f32) {
        let test_desc = RenderPipelineDescriptor {
            label: Some("test".into()),
            vertex: VertexState {
                shader: get_test().clone(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: get_test().clone(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: self.format.add_srgb_suffix(),
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            layout: vec![],
            push_constant_ranges: vec![],
        };

        self.renderer.prepare_frame(|fg| {
            let mut builder = fg.create_pass_node_builder(0, "final");

            let pipeline_handle = builder.register_render_pipeline(&test_desc);

            let new_swap_chain = builder.create("swap_chain", SwapChainDescriptor);

            let swap_chain_read_ref = builder.read(new_swap_chain);

            builder.add_attachment(ColorAttachment {
                view: ColorAttachmentView::new(swap_chain_read_ref.resource_handle()),
            });

            builder.render(move |render_pass, api| {
                let pipeline = api.get_render_pipeline(&pipeline_handle).unwrap();
                render_pass.set_render_pipeline(pipeline);
                render_pass.draw(0..3, 0..1);

                Ok(())
            });
        });

        self.update_pipeline_cache(dt);

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

    pub fn initialization(
        &mut self,
        device: Arc<Device>,
        shader_event_receiver: Receiver<ResourceEvent>,
        format: TextureFormat,
    ) {
        let renderer = Renderer::new(device.clone());

        *self = GraphicContext::Initialization(Box::new(InitializationGraphicContext {
            renderer,
            params: self.get_params().clone(),
            shader_event_receiver,
            format,
        }));
    }

    pub fn render(&mut self, dt: f32) {
        if let GraphicContext::Initialization(context) = self {
            context.render(dt)
        }
    }
}
