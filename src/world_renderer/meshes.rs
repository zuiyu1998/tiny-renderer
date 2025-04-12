use std::{mem, sync::Arc};

use image::GenericImageView;
use wgpu::{BufferUsages, ColorTargetState, TextureFormat};

use crate::{
    build_in::get_test,
    gfx_base::{
        BindGroupEntryInfo, BindGroupLayout, BindGroupLayoutInfo, BindGroupRef,
        BindingResourceInfo, BufferInitInfo, ColorAttachmentInfo, SampleInfo, TextureInfo,
        pipeline::{
            CachedRenderPipelineId, FragmentState, PipelineCache, RenderPipelineDescriptor,
            VertexBufferLayout, VertexState,
        },
    },
};

use super::{FrameGraphContext, Renderer};

pub trait MeshesRender: Renderer {
    fn register_render_pipeline(&mut self, pipeline_cache: &mut PipelineCache);
}

pub struct Image {
    bytes: Vec<u8>,
    texture_info: TextureInfo,
}

impl Default for Image {
    fn default() -> Self {
        Image::new()
    }
}

impl Image {
    pub fn new() -> Image {
        let diffuse_bytes = include_bytes!("happy-tree.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let dimensions = diffuse_image.dimensions();
        let diffuse_rgba = diffuse_image.to_rgba8().to_vec();

        let texture_info = TextureInfo {
            size: wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            label: Some("diffuse_image".into()),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };

        Image {
            bytes: diffuse_rgba,
            texture_info,
        }
    }
}

pub struct MeshMaterial {
    pub vertex_buffers: Vec<Vertex>,
    pub indexes: Vec<u16>,
    pub image: Image,
    pub id: Option<CachedRenderPipelineId>,
    pub texture_bind_group_layout: Option<BindGroupLayout>,
}

impl Renderer for MeshMaterial {
    fn prepare(&self, context: &mut FrameGraphContext) {
        if self.id.is_none() || self.texture_bind_group_layout.is_none() {
            return;
        }
        let pipeline_id = self.id.unwrap();
        let texture_bind_group_layout = self.texture_bind_group_layout.clone().unwrap();

        if context
            .pipeline_cache
            .get_render_pipeline(&pipeline_id)
            .is_none()
        {
            return;
        }
        let num_indices = self.indexes.len() as u32;

        let buffer = context.device.create_buffer_init(BufferInitInfo {
            label: Some("index_buffer".into()),
            usage: BufferUsages::INDEX,
            contents: bytemuck::cast_slice(&self.indexes),
        });

        let index_buffer = Arc::new(buffer);

        let buffer = context.device.create_buffer_init(BufferInitInfo {
            label: Some("vertex_buffer".into()),
            usage: BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&self.vertex_buffers),
        });

        let vertex_buffer = Arc::new(buffer);

        let texture = context
            .device
            .create_texture(self.image.texture_info.clone());
        texture.write_texture(&self.image.bytes);

        let texture = Arc::new(texture);

        let mut builder = context.frame_graph.create_pass_node_builder(2, "vertex");

        let texture_handle = builder.import("texture", texture);
        let texture_read = builder.read(texture_handle);

        let swap_chain_handle = builder.import("swap_chain", context.camera.get_texture_view());
        let swap_chain_read = builder.read(swap_chain_handle);

        let index_buffer_handle = builder.import("index_buffer", index_buffer);
        let index_buffer_read = builder.read(index_buffer_handle);

        let vertex_buffer_handle = builder.import("vertex_buffer", vertex_buffer);
        let vertex_buffer_read = builder.read(vertex_buffer_handle);

        builder.add_attachment_info(ColorAttachmentInfo::SwapChain(swap_chain_read));

        let bind_group = BindGroupRef {
            label: Some("diffuse_bind_group".into()),
            layout: texture_bind_group_layout,
            entries: vec![
                BindGroupEntryInfo {
                    binding: 0,
                    resource: BindingResourceInfo::TextureView(texture_read.clone()),
                },
                BindGroupEntryInfo {
                    binding: 1,
                    resource: BindingResourceInfo::Sampler(SampleInfo {}),
                },
            ],
            index: 0,
        };

        builder.render(move |render_context| {
            render_context.set_render_pipeline(&pipeline_id);
            render_context.set_bind_group(0, &bind_group);
            render_context.set_vertex_buffer(0, vertex_buffer_read);
            render_context.set_index_buffer(index_buffer_read, wgpu::IndexFormat::Uint16);
            render_context.draw_indexed(0..num_indices, 0, 0..1);

            Ok(())
        });
    }
}

impl MeshesRender for MeshMaterial {
    fn register_render_pipeline(&mut self, pipeline_cache: &mut PipelineCache) {
        if self.id.is_some() {
            return;
        }

        let texture_bind_group_layout =
            pipeline_cache
                .device
                .create_bind_group_layout(BindGroupLayoutInfo {
                    label: Some("texture_bind_group_layout".into()),
                    entries: vec![
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            // This should match the filterable field of the
                            // corresponding Texture entry above.
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

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
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
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
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: vec![],
        };

        self.id = Some(pipeline_cache.register_render_pipeline(test_desc));
        self.texture_bind_group_layout = Some(texture_bind_group_layout);
    }
}

impl MeshMaterial {
    pub fn new(vertex_buffers: Vec<Vertex>, indexes: Vec<u16>) -> Self {
        MeshMaterial {
            vertex_buffers,
            id: None,
            image: Image::new(),
            texture_bind_group_layout: None,
            indexes,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
