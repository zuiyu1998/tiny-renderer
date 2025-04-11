use std::sync::{Arc, mpsc::Receiver};

use fyrox_resource::event::ResourceEvent;
use wgpu::{ColorTargetState, TextureFormat};

use crate::{
    build_in::get_test,
    gfx_base::{
        device::Device,
        pipeline::{
            CachedRenderPipelineId, FragmentState, PipelineCache, RenderPipelineDescriptor,
            VertexBufferLayout, VertexState,
        },
        shader::Shader,
    },
    world_renderer::{RenderCamera, WorldRenderer},
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
        let world_renderer = WorldRenderer::new(device.clone());

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
            pipeline_cache: PipelineCache::new(device),
            mesh_material: MeshMaterial::new(vertex_buffers),
        }
    }
}

pub struct MeshMaterial {
    pub vertex_buffers: Vec<Vertex>,
    pub id: Option<CachedRenderPipelineId>,
}

impl MeshMaterial {
    pub fn new(vertex_buffers: Vec<Vertex>) -> Self {
        MeshMaterial {
            vertex_buffers,
            id: None,
        }
    }

    fn register_render_pipeline(&mut self, pipeline_cache: &mut PipelineCache) {
        if self.id.is_some() {
            return;
        }

        let vertex_buffer_layout = VertexBufferLayout {
            array_stride: core::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vec![
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: core::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let test_desc = RenderPipelineDescriptor {
            label: Some("test".into()),
            vertex: VertexState {
                shader: get_test().clone(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: get_test().clone(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
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

        self.id = Some(pipeline_cache.register_render_pipeline(test_desc));
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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
