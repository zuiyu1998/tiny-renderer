pub mod bind_group_layout;
pub mod buffer;
pub mod command_buffer;
pub mod device;
pub mod pipeline_layout;
pub mod render_pass;
pub mod render_pipeline;
pub mod shader_module;

pub use bind_group_layout::*;
pub use buffer::*;
pub use command_buffer::*;
pub use device::*;
pub use pipeline_layout::*;
pub use render_pipeline::*;
pub use shader_module::*;

use crate::gfx_base::texture_view::TextureViewTrait;

#[derive(Debug, Clone)]
pub struct WgpuTextureView(pub wgpu::TextureView);

impl TextureViewTrait for WgpuTextureView {}
