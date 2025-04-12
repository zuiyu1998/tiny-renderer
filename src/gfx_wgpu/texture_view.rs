use crate::gfx_base::texture_view::TextureViewTrait;

#[derive(Debug, Clone)]
pub struct WgpuTextureView(pub wgpu::TextureView);

impl TextureViewTrait for WgpuTextureView {}
