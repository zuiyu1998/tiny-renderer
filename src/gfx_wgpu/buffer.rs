use crate::gfx_base::buffer::BufferTrait;

#[derive(Debug, Clone)]
pub struct WgpuBuffer {
    pub buffer: wgpu::Buffer,
}

impl BufferTrait for WgpuBuffer {}
