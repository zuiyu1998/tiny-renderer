use crate::{define_atomic_id, define_gfx_type};

use super::{buffer::Buffer, device::Device, pipeline::RenderPipeline, render_pass::RenderPass};
use downcast_rs::Downcast;
use std::{fmt::Debug, ops::Range};

define_atomic_id!(CommandBufferId);

pub trait CommandBufferTrait: 'static + Sync + Send + Debug {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass);

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);

    fn end_render_pass(&mut self);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer);
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Debug + Downcast {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass);
    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn end_render_pass(&mut self);

    fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer);
}

impl<T: CommandBufferTrait> ErasedCommandBufferTrait for T {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass) {
        <T as CommandBufferTrait>::begin_render_pass(self, device, render_pass);
    }
    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        <T as CommandBufferTrait>::set_render_pipeline(self, render_pipeline);
    }
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        <T as CommandBufferTrait>::draw(self, vertices, instances);
    }

    fn end_render_pass(&mut self) {
        <T as CommandBufferTrait>::end_render_pass(self);
    }

    fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) {
        <T as CommandBufferTrait>::set_vertex_buffer(self, slot, buffer);
    }
}

define_gfx_type!(
    CommandBuffer,
    CommandBufferId,
    CommandBufferTrait,
    ErasedCommandBufferTrait
);

impl CommandBuffer {
    pub fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass) {
        self.value.begin_render_pass(device, render_pass);
    }

    pub fn end_render_pass(&mut self) {
        self.value.end_render_pass();
    }

    pub fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        self.value.set_render_pipeline(render_pipeline);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.value.draw(vertices, instances);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) {
        self.value.set_vertex_buffer(slot, buffer);
    }
}
