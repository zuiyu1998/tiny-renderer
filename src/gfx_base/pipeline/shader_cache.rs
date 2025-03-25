use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::gfx_base::{
    cache::TemporaryCache,
    device::Device,
    shader::{ShaderDefinition, ShaderResource},
    shader_module::{ShaderModule, ShaderModuleDescriptor},
};

use super::CachedPipelineId;

pub struct ShaderData {
    cached_pipeline_id: CachedPipelineId,
    shader_module: Arc<ShaderModule>,
}

#[derive(Default)]
pub struct ShaderCache {
    //todo 不需要ShaderDefinition
    pub(super) cache: TemporaryCache<ShaderDefinition>,
    data: HashMap<usize, ShaderData>,
}

impl Debug for ShaderCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShaderCache").finish()
    }
}

impl ShaderCache {
    pub fn remove(&mut self, shader: &ShaderResource) -> Vec<CachedPipelineId> {
        let mut state = shader.state();

        let mut free_pipeline_ids = vec![];

        if let Some(shader_state) = state.data() {
            self.cache.remove(&shader_state.cache_index);

            if let Some(data) = self.data.remove(&shader_state.cache_index.get()) {
                free_pipeline_ids.push(data.cached_pipeline_id);
            }
        }

        free_pipeline_ids
    }

    pub fn set_shader(&mut self, shader: &ShaderResource) -> Vec<CachedPipelineId> {
        let mut shader_state = shader.state();
        let mut free_pipeline_ids = vec![];

        if let Some(shader_state) = shader_state.data() {
            let _ = self.cache.get_or_insert_with(
                &shader_state.cache_index,
                Default::default(),
                || {
                    let target: Result<ShaderDefinition, ()> = Ok(shader_state.definition.clone());
                    target
                },
            );

            if let Some(data) = self.data.remove(&shader_state.cache_index.get()) {
                free_pipeline_ids.push(data.cached_pipeline_id);
            }
        }

        free_pipeline_ids
    }

    pub fn get(
        &mut self,
        device: &Device,
        pipeline: CachedPipelineId,
        shader: &ShaderResource,
    ) -> Option<Arc<ShaderModule>> {
        let mut shader_state = shader.state();
        if let Some(shader_state) = shader_state.data() {
            if let Some(data) = self.data.get(&shader_state.cache_index.get()) {
                Some(data.shader_module.clone())
            } else {
                let _ = self.cache.get_or_insert_with(
                    &shader_state.cache_index,
                    Default::default(),
                    || {
                        let target: Result<ShaderDefinition, ()> =
                            Ok(shader_state.definition.clone());
                        target
                    },
                );

                let shader_module = Arc::new(device.create_shader_module(ShaderModuleDescriptor {
                    label: None,
                    source: shader_state.definition.clone(),
                }));

                self.data.insert(
                    shader_state.cache_index.get(),
                    ShaderData {
                        cached_pipeline_id: pipeline,
                        shader_module: shader_module.clone(),
                    },
                );

                Some(shader_module)
            }
        } else {
            None
        }
    }

    pub fn update(&mut self, dt: f32) -> Vec<CachedPipelineId> {
        let free_shader_ids = self.cache.update(dt);

        let mut free_pipeline_ids = vec![];

        for free_shader_id in free_shader_ids {
            if let Some(data) = self.data.remove(&free_shader_id) {
                free_pipeline_ids.push(data.cached_pipeline_id);
            }
        }

        free_pipeline_ids
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn alive_count(&self) -> usize {
        self.cache.alive_count()
    }
}
