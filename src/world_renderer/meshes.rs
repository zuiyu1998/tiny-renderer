use std::sync::Arc;

use wgpu::{BufferUsages, ColorTargetState, TextureFormat};

use crate::{
    build_in::get_test,
    gfx_base::{
        BufferInitInfo, ColorAttachmentInfo,
        pipeline::{
            CachedRenderPipelineId, FragmentState, PipelineCache, RenderPipelineDescriptor,
            VertexBufferLayout, VertexState,
        },
    },
};

use super::{FrameGraphContext, renderer::Renderer};

pub trait MeshesRender: Renderer {
    fn register_render_pipeline(&mut self, pipeline_cache: &mut PipelineCache);
}

pub struct MeshMaterial {
    pub vertex_buffers: Vec<Vertex>,
    pub id: Option<CachedRenderPipelineId>,
}

impl Renderer for MeshMaterial {
    fn prepare(&self, context: &mut FrameGraphContext) {
        if self.id.is_none() {
            return;
        }
        let pipeline_id = self.id.clone().unwrap();

        if context
            .pipeline_cache
            .get_render_pipeline(&pipeline_id)
            .is_none()
        {
            return;
        }

        let buffer = context.device.create_buffer_init(BufferInitInfo {
            label: Some("vertex_buffer".into()),
            usage: BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&self.vertex_buffers),
        });

        let vertex_buffer = Arc::new(buffer);

        let mut builder = context.frame_graph.create_pass_node_builder(2, "vertex");

        let swap_chain_handle = builder.import("swap_chain", context.camera.get_texture_view());
        let swap_chain_read = builder.read(swap_chain_handle);

        let vertex_buffer_handle = builder.import("vertex_buffer", vertex_buffer);
        let vertex_buffer_read = builder.read(vertex_buffer_handle);

        builder.add_attachment_info(ColorAttachmentInfo::SwapChain(swap_chain_read));

        builder.render(move |render_context| {
            render_context.set_render_pipeline(&pipeline_id);
            render_context.set_vertex_buffer(0, vertex_buffer_read);
            render_context.draw(0..3, 0..1);

            Ok(())
        });
    }
}

impl MeshesRender for MeshMaterial {
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

impl MeshMaterial {
    pub fn new(vertex_buffers: Vec<Vertex>) -> Self {
        MeshMaterial {
            vertex_buffers,
            id: None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
