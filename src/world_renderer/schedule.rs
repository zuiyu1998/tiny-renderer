use std::collections::HashMap;

use downcast_rs::{Downcast, impl_downcast};

use crate::gfx_base::ColorAttachmentInfo;

use super::{FrameGraphContext, renderer::Renderer};

pub trait RenderScheduleTrait: 'static + Downcast + Sync + Send {
    fn name() -> &'static str;

    fn prepare(&self, context: &mut FrameGraphContext);
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

impl Default for RenderSchedules {
    fn default() -> Self {
        let mut schedules = RenderSchedules::new();
        schedules.add_schedule(CameraDriverSchedule);

        schedules
    }
}

impl RenderSchedules {
    pub fn new() -> Self {
        Self {
            schedules: Default::default(),
        }
    }
}

impl RenderSchedules {
    pub fn add_schedule<T: RenderScheduleTrait>(&mut self, value: T) {
        let schedule = RenderSchedule::new(value);

        self.schedules.insert(schedule.name.to_string(), schedule);
    }
}

pub trait RenderStage {
    fn prepare(&self, context: &mut FrameGraphContext);
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
}
