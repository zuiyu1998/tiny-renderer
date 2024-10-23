use std::collections::VecDeque;

use wgpu::{BufferDescriptor as WgpuBufferDescriptor, SurfaceTexture};

use crate::render_backend::RenderBuffer;
pub use wgpu::BufferUsages;

pub struct SwapchainImages {
    pub images: VecDeque<SwapchainImage>,
}

pub struct SwapchainImage {
    pub texture: SurfaceTexture,
}

impl SwapchainImage {
    pub fn new(texture: SurfaceTexture) -> Self {
        SwapchainImage { texture }
    }
}

#[derive(Clone, Debug)]
pub struct SwapchainImageDescriptor {}

pub struct Image {
    pub descriptor: ImageDescriptor,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]

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
