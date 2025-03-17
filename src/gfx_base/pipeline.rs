use downcast::{Any, downcast};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::device::Device;

pub struct PipelineCache {
    rp_handle_to_pipeline: HashMap<RenderPipelineHandle, RenderPipelineState>,
    rp_descs_to_handle: HashMap<RenderPipelineDescriptor, RenderPipelineHandle>,
    render_pipelines: Vec<Option<Arc<RenderPipeline>>>,
    device: Arc<Device>,
}

impl PipelineCache {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            rp_handle_to_pipeline: Default::default(),
            rp_descs_to_handle: Default::default(),
            render_pipelines: Default::default(),
            device,
        }
    }

    pub fn get_render_pipeline(&self, handle: &RenderPipelineHandle) -> Arc<RenderPipeline> {
        self.rp_handle_to_pipeline
            .get(handle)
            .unwrap()
            .pipeline
            .clone()
            .unwrap()
    }

    pub fn register_render_pipeline(
        &mut self,
        desc: RenderPipelineDescriptor,
    ) -> RenderPipelineHandle {
        if let Some(handle) = self.rp_descs_to_handle.get(&desc) {
            *handle
        } else {
            let handle = self.render_pipelines.len();
            let handle = RenderPipelineHandle(handle);

            //todo async

            let pipeline = Some(Arc::new(self.device.create_render_pipeline(desc.clone())));

            self.render_pipelines.push(pipeline.clone());

            self.rp_handle_to_pipeline
                .insert(handle, RenderPipelineState { pipeline });

            self.rp_descs_to_handle.insert(desc, handle);

            handle
        }
    }
}

#[derive(Default)]
pub struct RenderPipelineState {
    pipeline: Option<Arc<RenderPipeline>>,
}

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq)]
pub struct RenderPipelineHandle(usize);

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct RenderPipelineDescriptor {
    pub label: Option<String>,
}

pub trait RenderPipelineTrait: 'static + Any + Debug + Sync + Send {}

pub struct RenderPipeline(Box<dyn RenderPipelineTrait>);

downcast!(dyn RenderPipelineTrait);

impl RenderPipeline {
    pub fn new<T: RenderPipelineTrait>(pipeline: T) -> Self {
        RenderPipeline(Box::new(pipeline))
    }

    pub fn downcast<T: RenderPipelineTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.0.downcast::<T>().ok();
        value
    }

    pub fn downcast_ref<T: RenderPipelineTrait>(&self) -> Option<&T> {
        let value: Option<&T> = self.0.downcast_ref::<T>().ok();
        value
    }
}
