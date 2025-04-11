use std::{
    collections::{HashMap, HashSet},
    mem,
    sync::Arc,
};

use super::{RenderPipeline, RenderPipelineDescriptor, RenderPipelineDescriptorState, ShaderCache};
use crate::gfx_base::{
    bind_group_layout::{BindGroupLayout, BindGroupLayoutId},
    device::Device,
    pipeline_layout::{PipelineLayout, PipelineLayoutDescriptor},
    shader::Shader,
};
use fyrox_resource::Resource;
use thiserror::Error;
use tracing::error;
use wgpu::PushConstantRange;

pub type CachedPipelineId = usize;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedRenderPipelineId(CachedPipelineId);

impl CachedRenderPipelineId {
    pub const INVALID: Self = CachedRenderPipelineId(usize::MAX);

    #[inline]
    pub fn id(&self) -> usize {
        self.0
    }

    pub fn new(id: CachedPipelineId) -> Self {
        CachedRenderPipelineId(id)
    }
}

#[derive(Error, Debug)]
pub enum PipelineCacheError {
    #[error(transparent)]
    ProcessShaderError(#[from] naga_oil::compose::ComposerError),
    #[error("Shader import not yet available.")]
    ShaderImportNotYetAvailable,
    #[error("Could not create shader module: {0}")]
    CreateShaderModule(String),
}

pub enum Pipeline {
    RenderPipeline(RenderPipeline),
}

pub enum CachedPipelineState {
    Queued,
    /// The pipeline GPU object was created successfully and is available (allocated on the GPU).
    Ok(Pipeline),
    Err(PipelineCacheError),
}

pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
}

pub struct CachedPipeline {
    pub descriptor: PipelineDescriptor,
    pub state: CachedPipelineState,
}

type LayoutCacheKey = (Vec<BindGroupLayoutId>, Vec<PushConstantRange>);

#[derive(Default)]
struct LayoutCache {
    layouts: HashMap<LayoutCacheKey, Arc<PipelineLayout>>,
}

impl LayoutCache {
    fn get(
        &mut self,
        device: &Device,
        bind_group_layouts: &[BindGroupLayout],
        push_constant_ranges: Vec<PushConstantRange>,
    ) -> Arc<PipelineLayout> {
        let bind_group_ids = bind_group_layouts.iter().map(BindGroupLayout::id).collect();
        self.layouts
            .entry((bind_group_ids, push_constant_ranges))
            .or_insert_with_key(|(_, push_constant_ranges)| {
                let bind_group_layouts = bind_group_layouts
                    .iter()
                    .map(BindGroupLayout::clone)
                    .collect::<Vec<_>>();
                Arc::new(device.create_pipeline_layout(PipelineLayoutDescriptor {
                    bind_group_layouts,
                    push_constant_ranges: push_constant_ranges.to_vec(),
                }))
            })
            .clone()
    }
}

pub struct PipelineCache {
    shader_cache: ShaderCache,
    layout_cache: LayoutCache,
    pipelines: Vec<CachedPipeline>,
    waiting_pipelines: HashSet<CachedPipelineId>,
    new_pipelines: Vec<CachedPipeline>,
    device: Arc<Device>,
}

impl PipelineCache {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            shader_cache: ShaderCache::default(),
            pipelines: Default::default(),
            waiting_pipelines: Default::default(),
            new_pipelines: Default::default(),
            device,
            layout_cache: Default::default(),
        }
    }

    pub fn process_queue(&mut self) {
        let mut waiting_pipelines = mem::take(&mut self.waiting_pipelines);
        let mut pipelines = mem::take(&mut self.pipelines);

        for new_pipeline in self.new_pipelines.drain(..) {
            let id = pipelines.len();
            pipelines.push(new_pipeline);
            waiting_pipelines.insert(id);
        }

        for id in waiting_pipelines {
            self.process_pipeline(&mut pipelines[id], id);
        }

        self.pipelines = pipelines;
    }

    fn process_pipeline(&mut self, cached_pipeline: &mut CachedPipeline, id: usize) {
        match &mut cached_pipeline.state {
            CachedPipelineState::Queued => {
                cached_pipeline.state = match &cached_pipeline.descriptor {
                    PipelineDescriptor::RenderPipelineDescriptor(descriptor) => {
                        self.start_create_render_pipeline(id, *descriptor.clone())
                    }
                };
            }

            CachedPipelineState::Err(err) => match err {
                PipelineCacheError::ShaderImportNotYetAvailable => {
                    cached_pipeline.state = CachedPipelineState::Queued;
                }

                PipelineCacheError::ProcessShaderError(_err) => {
                    error!("failed to process shader:\n",);
                    return;
                }
                PipelineCacheError::CreateShaderModule(description) => {
                    error!("failed to create shader module: {}", description);
                    return;
                }
            },

            CachedPipelineState::Ok(_) => return,
        }

        // Retry
        self.waiting_pipelines.insert(id);
    }

    fn start_create_render_pipeline(
        &mut self,
        id: CachedPipelineId,
        descriptor: RenderPipelineDescriptor,
    ) -> CachedPipelineState {
        let vertex_module = match self
            .shader_cache
            .get(&self.device, id, &descriptor.vertex.shader)
        {
            Some(module) => module,
            None => {
                return CachedPipelineState::Err(PipelineCacheError::ShaderImportNotYetAvailable);
            }
        };

        let fragment_module = match &descriptor.fragment {
            Some(fragment) => match self.shader_cache.get(&self.device, id, &fragment.shader) {
                Some(module) => Some(module),
                None => {
                    return CachedPipelineState::Err(
                        PipelineCacheError::ShaderImportNotYetAvailable,
                    );
                }
            },
            None => None,
        };

        let layout = if descriptor.layout.is_empty() && descriptor.push_constant_ranges.is_empty() {
            None
        } else {
            Some(self.layout_cache.get(
                &self.device,
                &descriptor.layout,
                descriptor.push_constant_ranges.to_vec(),
            ))
        };

        let pipeline = self
            .device
            .create_render_pipeline(RenderPipelineDescriptorState {
                vertex_module: &vertex_module,
                fragment_module: fragment_module.as_deref(),
                layout: layout.as_deref(),
                desc: descriptor,
            });

        CachedPipelineState::Ok(Pipeline::RenderPipeline(pipeline))
    }

    pub fn update(&mut self, dt: f32) {
        let free_pipeline_ids = self.shader_cache.update(dt);
        self.free(free_pipeline_ids);

        self.process_queue();
    }

    pub fn set_shader(&mut self, shader: &Resource<Shader>) {
        let free_pipeline_ids = self.shader_cache.set_shader(shader);
        self.free(free_pipeline_ids);
    }

    pub fn remove(&mut self, shader: &Resource<Shader>) {
        let free_pipeline_ids = self.shader_cache.remove(shader);
        self.free(free_pipeline_ids);
    }

    pub fn free(&mut self, ids: Vec<CachedPipelineId>) {
        for cached_pipeline in ids {
            self.pipelines[cached_pipeline].state = CachedPipelineState::Queued;
            self.waiting_pipelines.insert(cached_pipeline);
        }
    }

    pub fn get_render_pipeline(&self, id: &CachedRenderPipelineId) -> Option<&RenderPipeline> {
        if let CachedPipelineState::Ok(Pipeline::RenderPipeline(pipeline)) =
            &self.pipelines[id.0].state
        {
            Some(pipeline)
        } else {
            None
        }
    }

    pub fn register_render_pipeline(
        &mut self,
        desc: RenderPipelineDescriptor,
    ) -> CachedRenderPipelineId {
        let id = CachedRenderPipelineId::new(self.pipelines.len() + self.new_pipelines.len());
        self.new_pipelines.push(CachedPipeline {
            descriptor: PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc)),
            state: CachedPipelineState::Queued,
        });
        id
    }
}
