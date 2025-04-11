use std::{collections::HashMap, sync::Arc};

use downcast_rs::{Downcast, impl_downcast};

use crate::gfx_base::{ColorAttachmentInfo, device::Device, pipeline::PipelineCache};

use super::{FrameGraphContext, renderer::Renderer};

pub trait RenderScheduleTrait: 'static + Downcast + Sync + Send {
    fn name() -> &'static str;

    fn prepare(&self, context: &mut FrameGraphContext);

    fn from_pipeline_cache(_pipeline_cache: &mut PipelineCache) -> Self;
}

pub trait ErasedRenderScheduleTrait: 'static + Downcast + Sync + Send {
    fn prepare(&self, context: &mut FrameGraphContext);
}

impl_downcast!(ErasedRenderScheduleTrait);

impl<T: RenderScheduleTrait> ErasedRenderScheduleTrait for T {
    fn prepare(&self, context: &mut FrameGraphContext) {
        <T as RenderScheduleTrait>::prepare(self, context)
    }
}

pub struct RenderSchedule {
    name: String,
    value: Box<dyn ErasedRenderScheduleTrait>,
}

impl RenderSchedule {
    pub fn new<T: RenderScheduleTrait>(value: T) -> Self {
        RenderSchedule {
            name: T::name().to_string(),
            value: Box::new(value),
        }
    }
}

pub struct RenderSchedules {
    schedules: HashMap<String, RenderSchedule>,
}

impl Renderer for RenderSchedules {
    fn prepare(&self, context: &mut FrameGraphContext) {
        for schedule in self.schedules.values() {
            schedule.value.prepare(context);
        }
    }
}

impl RenderSchedules {
    pub fn new(pipeline_cache: &mut PipelineCache) -> Self {
        let mut schedules = RenderSchedules::empty();
        
        schedules.add_schedule(CameraDriverSchedule::from_pipeline_cache(pipeline_cache));
        
        schedules
        
    }

    pub fn empty() -> Self {
        Self {
            schedules: Default::default(),
        }
    }

    pub fn add_schedule<T: RenderScheduleTrait>(&mut self, value: T) {
        let schedule = RenderSchedule::new(value);

        self.schedules.insert(schedule.name.to_string(), schedule);
    }
}

pub trait RenderStage {
    fn prepare(&self, context: &mut FrameGraphContext);

    fn new(_device: &Arc<Device>, _pipeline_cache: &mut PipelineCache) -> Self;
}

pub struct CameraDriverSchedule;

impl RenderScheduleTrait for CameraDriverSchedule {
    fn name() -> &'static str {
        "CameraDriverSchedule"
    }

    fn prepare(&self, context: &mut FrameGraphContext) {
        let mut builder = context
            .frame_graph
            .create_pass_node_builder(1, "camera_driver");

        let swap_chain_handle = builder.import("swap_chain", context.camera.get_texture_view());

        let swap_chain_read = builder.read(swap_chain_handle);

        builder.add_attachment_info(ColorAttachmentInfo::SwapChain(swap_chain_read));

        builder.render(|_render_context| Ok(()));
    }

    fn from_pipeline_cache(_pipeline_cache: &mut PipelineCache) -> Self {
        CameraDriverSchedule
    }
}
