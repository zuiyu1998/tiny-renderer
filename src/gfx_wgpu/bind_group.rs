use crate::gfx_base::BindGroupTrait;

#[derive(Debug, Clone)]
pub struct WgpuBindGroup(pub wgpu::BindGroup);

impl BindGroupTrait for WgpuBindGroup {}
