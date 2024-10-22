pub mod error;
pub mod frame_graph;
pub mod render_backend;
pub mod renderer;
pub mod windows;

use render_backend::RenderBackend;
use renderer::Renderer;
use windows::Windows;
use winit::window::Window;

pub enum GraphsContext {
    Uninitialized,
    Initialized(InitializedGraphsContext),
}

pub struct InitializedGraphsContext {
    pub windows: Windows,
    pub renderer: Renderer,
}

impl InitializedGraphsContext {
    pub fn new(primary_window: Window) -> Self {
        let render_backend = RenderBackend::create_render_backend();
        let windows = Windows::new(primary_window, &render_backend);
        let renderer = Renderer::new(render_backend);

        Self { windows, renderer }
    }

    pub fn update(&mut self) {
        println!("InitializedGraphsContext update");

        self.renderer.render(&mut self.windows);
    }

    pub fn redraw_requested(&mut self) {
        self.windows.request_redraw();
    }
}

impl GraphsContext {
    pub fn initialize(&mut self, window: Window) {
        if let GraphsContext::Uninitialized = self {
            *self = GraphsContext::Initialized(InitializedGraphsContext::new(window));
        }
    }

    pub fn update(&mut self) {
        if let GraphsContext::Initialized(context) = self {
            context.update();
        }
    }

    pub fn redraw_requested(&mut self) {
        if let GraphsContext::Initialized(context) = self {
            context.redraw_requested();
        }
    }
}
