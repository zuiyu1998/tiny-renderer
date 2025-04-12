use crate::gfx_base::SampleTrait;

#[derive(Debug, Clone)]
pub struct WgpuSample(pub wgpu::Sampler);

impl SampleTrait for WgpuSample {}
