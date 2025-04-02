use crate::gfx_base::buffer::BufferTrait;

#[derive(Debug)]
pub struct WgpuBuffer {
    pub buffer: wgpu::Buffer,
}

impl BufferTrait for WgpuBuffer {}
