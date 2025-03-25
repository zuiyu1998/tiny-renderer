use crate::gfx_base::shader_module::ShaderModuleTrait;

#[derive(Debug)]
pub struct WgpuShaderModule(wgpu::ShaderModule);

impl WgpuShaderModule {
    pub fn new(shader_module: wgpu::ShaderModule) -> Self {
        WgpuShaderModule(shader_module)
    }

    pub fn shader_module(&self) -> &wgpu::ShaderModule {
        &self.0
    }
}

impl ShaderModuleTrait for WgpuShaderModule {}
