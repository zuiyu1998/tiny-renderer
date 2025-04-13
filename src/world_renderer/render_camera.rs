use crate::gfx_base::{Buffer, BufferInitInfo, device::Device, texture_view::TextureView};
use std::sync::Arc;

pub enum RenderTarget {
    Window(Arc<TextureView>),
}

#[derive(Default, Clone)]
pub struct CameraInfo {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct RenderCamera {
    pub render_target: RenderTarget,
    pub info: CameraInfo,
}

#[repr(C)]
// derive 属性自动导入的这些 trait，令其可被存入缓冲区
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    // glam 的数据类型不能直接用于 bytemuck
    // 需要先将 Matrix4 矩阵转为一个 4x4 的浮点数数组
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &RenderCamera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

impl RenderCamera {
    pub fn get_camera_buffer(&self, device: &Device) -> Arc<Buffer> {
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(self);

        let camera_buffer = device.create_buffer_init(BufferInitInfo {
            label: Some("camera_buffer".into()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[camera_uniform]),
        });

        Arc::new(camera_buffer)
    }

    pub fn get_texture_view(&self) -> Arc<TextureView> {
        match &self.render_target {
            RenderTarget::Window(texture_view) => texture_view.clone(),
        }
    }

    fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.info.eye, self.info.target, self.info.up);
        let proj = glam::Mat4::perspective_rh(
            self.info.fovy.to_radians(),
            self.info.aspect,
            self.info.znear,
            self.info.zfar,
        );

        proj * view
    }
}
