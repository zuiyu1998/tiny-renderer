use wgpu::BufferDescriptor as WgpuBufferDescriptor;

use crate::render_backend::RenderBuffer;
pub use wgpu::BufferUsages;

pub struct Image {
    pub descriptor: ImageDescriptor,
}

#[derive(Clone, Debug)]
pub struct ImageDescriptor {}

pub struct Buffer {
    pub descriptor: BufferDescriptor,
    pub render_buffer: RenderBuffer,
}

impl Buffer {
    pub fn new(render_buffer: RenderBuffer, descriptor: BufferDescriptor) -> Self {
        Self {
            descriptor,
            render_buffer,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BufferDescriptor {
    pub label: String,
    pub size: u64,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}

impl BufferDescriptor {
    pub fn get_wgpu_descriptor(&self) -> WgpuBufferDescriptor {
        WgpuBufferDescriptor {
            label: Some(&self.label),
            size: self.size,
            usage: self.usage.clone(),
            mapped_at_creation: self.mapped_at_creation,
        }
    }
}
