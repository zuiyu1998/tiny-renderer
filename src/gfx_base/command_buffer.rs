use crate::{define_atomic_id, define_gfx_type};

use super::{
    BindGroup, buffer::Buffer, device::Device, pipeline::RenderPipeline, render_pass::RenderPass,
};
use downcast_rs::Downcast;
use std::{fmt::Debug, ops::Range};
use wgpu::IndexFormat;

define_atomic_id!(CommandBufferId);

pub trait CommandBufferTrait: 'static + Sync + Send + Debug {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass);

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);

    fn end_render_pass(&mut self);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);

    fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer);

    fn set_index_buffer(&mut self, buffer: &Buffer, index_format: IndexFormat);

    fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup);
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Debug + Downcast {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass);
    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn end_render_pass(&mut self);

    fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer);

    fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup);

    fn set_index_buffer(&mut self, buffer: &Buffer, index_format: IndexFormat);

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);
}

impl<T: CommandBufferTrait> ErasedCommandBufferTrait for T {
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        <T as CommandBufferTrait>::draw_indexed(self, indices, base_vertex, instances);
    }

    fn set_index_buffer(&mut self, buffer: &Buffer, index_format: IndexFormat) {
        <T as CommandBufferTrait>::set_index_buffer(self, buffer, index_format);
    }

    fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup) {
        <T as CommandBufferTrait>::set_bind_group(self, index, bind_group);
    }

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

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.value.draw_indexed(indices, base_vertex, instances);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer) {
        self.value.set_vertex_buffer(slot, buffer);
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup) {
        self.value.set_bind_group(index, bind_group);
    }

    pub fn set_index_buffer(&mut self, buffer: &Buffer, index_format: IndexFormat) {
        self.value.set_index_buffer(buffer, index_format);
    }
}
