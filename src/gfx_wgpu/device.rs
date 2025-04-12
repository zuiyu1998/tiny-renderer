use wgpu::util::DeviceExt;

use crate::{
    gfx_base::{
        BindGroup, BindGroupInfo, BindGroupLayout, BindGroupLayoutInfo, BindingResource, Sample,
        SampleInfo, Texture, TextureInfo,
        buffer::{Buffer, BufferInfo, BufferInitInfo},
        command_buffer::CommandBuffer,
        device::DeviceTrait,
        pipeline::{RenderPipeline, RenderPipelineDescriptorState},
        pipeline_layout::{PipelineLayout, PipelineLayoutDescriptor},
        render_pass::{RenderPass, RenderPassDescriptor},
        shader_module::{ShaderModule, ShaderModuleDescriptor},
    },
    gfx_wgpu::{WgpuBindGroup, WgpuBindGroupLayout, WgpuBuffer},
};

use super::{
    WgpuCommandBuffer, WgpuPipelineLayout, WgpuRenderPipeline, WgpuSample, WgpuShaderModule,
    WgpuTextureView, render_pass::WgpuRenderPass, texture::WgpuTexture,
};

#[derive(Debug)]
pub struct WgpuDevice {
    pub device: wgpu::Device,

    queue: wgpu::Queue,
}

impl WgpuDevice {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        WgpuDevice { device, queue }
    }
}

impl DeviceTrait for WgpuDevice {
    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        RenderPass::new(WgpuRenderPass::new(desc))
    }

    fn submit(&self, command_buffers: Vec<CommandBuffer>) {
        let mut targets = vec![];

        for command_buffer in command_buffers.into_iter() {
            let mut command_buffer = command_buffer.downcast::<WgpuCommandBuffer>().unwrap();

            if let Some(command_buffer) = command_buffer.command_buffer.take() {
                targets.push(command_buffer);
            }
        }

        self.queue.submit(targets);
    }

    fn create_render_pipeline(&self, state: RenderPipelineDescriptorState) -> RenderPipeline {
        let vertex_module = state
            .vertex_module
            .downcast_ref::<WgpuShaderModule>()
            .unwrap();

        let vertex_buffer_layouts = state
            .desc
            .vertex
            .buffers
            .iter()
            .map(|layout| wgpu::VertexBufferLayout {
                array_stride: layout.array_stride,
                attributes: &layout.attributes,
                step_mode: layout.step_mode,
            })
            .collect::<Vec<_>>();

        let vertex_state: wgpu::VertexState = wgpu::VertexState {
            module: vertex_module.shader_module(),
            entry_point: Some(&state.desc.vertex.entry_point),
            compilation_options: Default::default(),
            buffers: &vertex_buffer_layouts,
        };

        let fragment_state = state.fragment_module.as_ref().map(|fragment_module| {
            let fragment_module = fragment_module.downcast_ref::<WgpuShaderModule>().unwrap();
            let fragment = state.desc.fragment.as_ref().unwrap();

            wgpu::FragmentState {
                module: fragment_module.shader_module(),
                entry_point: Some(&fragment.entry_point),
                compilation_options: Default::default(),
                targets: &fragment.targets,
            }
        });

        let layout = state.layout.as_ref().map(|layout| {
            layout
                .downcast_ref::<WgpuPipelineLayout>()
                .unwrap()
                .pipeline_layout()
        });

        let label = state.desc.label.as_ref().map(|label| label.to_string());

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: label.as_deref(),
                layout,
                vertex: vertex_state,
                fragment: fragment_state,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

        RenderPipeline::new(WgpuRenderPipeline::new(render_pipeline))
    }

    fn create_command_buffer(&self) -> CommandBuffer {
        CommandBuffer::new(WgpuCommandBuffer::default())
    }

    fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModule {
        let source =
            wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(desc.source.source.as_str()));

        let shader_module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: desc.label.as_deref(),
                source,
            });

        ShaderModule::new(WgpuShaderModule::new(shader_module))
    }

    fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayout {
        let bind_group_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|layout| {
                let layout = layout
                    .downcast_ref::<WgpuBindGroupLayout>()
                    .unwrap()
                    .bind_group_layout();
                layout
            })
            .collect::<Vec<_>>();

        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &desc.push_constant_ranges,
            });

        PipelineLayout::new(WgpuPipelineLayout::new(layout))
    }

    fn create_buffer(&self, desc: BufferInfo) -> Buffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: desc.label.as_deref(),
            usage: desc.usage,
            size: desc.size,
            mapped_at_creation: desc.mapped_at_creation,
        });

        Buffer::new(WgpuBuffer { buffer }, desc)
    }

    fn create_buffer_init(&self, desc: BufferInitInfo) -> Buffer {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: desc.label.as_deref(),
                usage: desc.usage,
                contents: desc.contents,
            });

        Buffer::new(
            WgpuBuffer { buffer },
            BufferInfo {
                label: desc.label,
                size: desc.contents.len() as u64,
                usage: desc.usage,
                mapped_at_creation: true,
            },
        )
    }

    fn create_texture(&self, desc: TextureInfo) -> Texture {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: desc.label.as_deref(),
            size: desc.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: desc.dimension,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: desc.usage,
            view_formats: &[],
        });

        Texture::new(
            WgpuTexture {
                texture,
                queue: self.queue.clone(),
                device: self.device.clone(),
            },
            desc,
        )
    }

    fn create_bind_group_layout(&self, desc: BindGroupLayoutInfo) -> BindGroupLayout {
        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: desc.label.as_deref(),
                    entries: &desc.entries,
                });
        BindGroupLayout::new(WgpuBindGroupLayout(bind_group_layout))
    }

    fn create_bind_group(&self, desc: BindGroupInfo) -> BindGroup {
        let layout = desc.layout.downcast_ref::<WgpuBindGroupLayout>().unwrap();

        let entries = desc
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: get_binding_resource(&entry.resource),
            })
            .collect::<Vec<_>>();

        let wgpu_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: desc.label.as_deref(),
            layout: &layout.0,
            entries: &entries,
        });

        BindGroup::new(WgpuBindGroup(wgpu_bind_group))
    }

    fn create_sampler(&self, _desc: SampleInfo) -> Sample {
        let sample = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Sample::new(WgpuSample(sample))
    }
}

fn get_binding_resource(source: &BindingResource) -> wgpu::BindingResource {
    match source {
        BindingResource::TextureView(res) => {
            let res = res.downcast_ref::<WgpuTextureView>().unwrap();
            wgpu::BindingResource::TextureView(&res.0)
        }

        BindingResource::Sampler(res) => {
            let res = res.downcast_ref::<WgpuSample>().unwrap();
            wgpu::BindingResource::Sampler(&res.0)
        }
    }
}
