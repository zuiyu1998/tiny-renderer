use crate::gfx_base::pipeline::RenderPipelineTrait;

#[derive(Debug)]
pub struct WgpuRenderPipeline(pub wgpu::RenderPipeline);

impl WgpuRenderPipeline {
    pub fn new(pipeline: wgpu::RenderPipeline) -> Self {
        WgpuRenderPipeline(pipeline)
    }
}

impl RenderPipelineTrait for WgpuRenderPipeline {}
