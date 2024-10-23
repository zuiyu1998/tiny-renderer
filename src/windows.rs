use std::collections::{HashMap, VecDeque};

use wgpu::{Surface, SurfaceTargetUnsafe, SurfaceTexture};
use winit::window::{Window, WindowId};

use crate::{
    render_backend::RenderBackend,
    renderer::resource::{SwapchainImage, SwapchainImages},
};

pub struct WindowState {
    window: Window,
    surface: Surface<'static>,
    texture: Option<SurfaceTexture>,
}

impl WindowState {
    pub fn swap(&mut self) {
        self.texture = Some(
            self.surface
                .get_current_texture()
                .expect("get surface texture fail"),
        );
    }
}

pub struct Windows {
    primary_window_id: WindowId,
    windows: HashMap<WindowId, WindowState>,
}

impl Windows {
    pub fn request_redraw(&mut self) {
        for state in self.windows.values_mut() {
            state.window.request_redraw();
        }
    }

    pub fn new(primary_window: Window, backend: &RenderBackend) -> Self {
        let primary_window_id = primary_window.id();

        let window_state = create_window_state(backend, primary_window);

        let mut windows = HashMap::default();

        windows.insert(primary_window_id, window_state);

        Self {
            primary_window_id,
            windows,
        }
    }

    pub fn get_current_swapchain_images(&mut self) -> SwapchainImages {
        let mut images = VecDeque::default();

        for state in self.windows.values_mut() {
            state.swap();

            images.push_back(SwapchainImage::new(state.texture.take().unwrap()));
        }

        SwapchainImages { images }
    }
}

pub fn create_window_state(backend: &RenderBackend, window: Window) -> WindowState {
    let size = window.inner_size();

    let surface = unsafe {
        backend
            .instance
            .create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap())
            .unwrap()
    };

    let surface_caps = surface.get_capabilities(&backend.adapter);

    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    backend.device.configure_surface(&surface, &config);

    WindowState {
        surface,
        window,
        texture: None,
    }
}
