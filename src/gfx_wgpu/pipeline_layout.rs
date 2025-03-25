use crate::gfx_base::pipeline_layout::PipelineLayoutTrait;

#[derive(Debug, Clone)]
pub struct WgpuPipelineLayout(wgpu::PipelineLayout);

impl WgpuPipelineLayout {
    pub fn new(value: wgpu::PipelineLayout) -> Self {
        Self(value)
    }

    pub fn pipeline_layout(&self) -> &wgpu::PipelineLayout {
        &self.0
    }
}

impl PipelineLayoutTrait for WgpuPipelineLayout {}
