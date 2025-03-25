use crate::gfx_base::bind_group_layout::BindGroupLayoutTrait;

#[derive(Debug, Clone)]
pub struct WgpuBindGroupLayout(wgpu::BindGroupLayout);

impl WgpuBindGroupLayout {
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.0
    }
}

impl BindGroupLayoutTrait for WgpuBindGroupLayout {}
