use std::sync::{Arc, mpsc::Receiver};

use fyrox_resource::event::ResourceEvent;

use crate::{
    gfx_base::{device::Device, pipeline::PipelineCache, shader::Shader},
    world_renderer::{
        MeshesRender, RenderCamera, WorldRenderer,
        meshes::{MeshMaterial, Vertex},
    },
};

pub struct InitializationGraphicContext {
    world_renderer: WorldRenderer,
    params: GraphicContextParams,
    shader_event_receiver: Receiver<ResourceEvent>,
    pipeline_cache: PipelineCache,
    mesh_material: MeshMaterial,
}

impl InitializationGraphicContext {
    pub fn new(
        device: Arc<Device>,
        params: GraphicContextParams,
        shader_event_receiver: Receiver<ResourceEvent>,
    ) -> Self {
        let mut pipeline_cache = PipelineCache::new(device);

        let world_renderer = WorldRenderer::new(&mut pipeline_cache);

        let vertex_buffers = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        InitializationGraphicContext {
            world_renderer,
            params,
            shader_event_receiver,
            pipeline_cache,
            mesh_material: MeshMaterial::new(vertex_buffers),
        }
    }
}

impl InitializationGraphicContext {
    fn update_pipeline_cache(&mut self, dt: f32) {
        while let Ok(event) = self.shader_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource)
            | ResourceEvent::Reloaded(resource)
            | ResourceEvent::Added(resource) = event
            {
                if let Some(shader) = resource.try_cast::<Shader>() {
                    self.pipeline_cache.remove(&shader);
                    self.pipeline_cache.set_shader(&shader);
                }
            }
        }

        self.pipeline_cache.update(dt);
    }

    fn render(&mut self, dt: f32, cameras: &[RenderCamera]) {
        self.update_pipeline_cache(dt);

        self.mesh_material
            .register_render_pipeline(&mut self.pipeline_cache);

        self.world_renderer
            .render(&mut self.pipeline_cache, cameras, &self.mesh_material)
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
    ) {
        *self = GraphicContext::Initialization(Box::new(InitializationGraphicContext::new(
            device,
            self.get_params().clone(),
            shader_event_receiver,
        )));
    }

    pub fn render(&mut self, dt: f32, cameras: &[RenderCamera]) {
        if let GraphicContext::Initialization(context) = self {
            context.render(dt, cameras)
        }
    }
}
