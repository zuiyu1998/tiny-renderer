use std::collections::HashMap;

use wgpu::Surface;
use winit::window::{Window, WindowId};

use crate::render_backend::RenderBackend;

pub struct WindowState {
    window: Window,
    surface: Surface<'static>,
}

pub struct Windows {
    primary_window_id: WindowId,
    windows: HashMap<WindowId, WindowState>,
}

impl Windows {
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
}

pub fn create_window_state(backend: &RenderBackend, window: Window) -> WindowState {
    // let surface = backend.device.

    todo!()
}
