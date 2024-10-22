pub mod error;
pub mod frame_graph;
pub mod render_backend;
pub mod renderer;
pub mod windows;

use render_backend::RenderBackend;
use renderer::Renderer;
use winit::window::Window;

pub enum GraphsContext {
    Uninitialized,
    Initialized(InitializedGraphsContext),
}

pub struct InitializedGraphsContext {
    pub primary_window: Window,
    pub renderer: Renderer,
}

impl InitializedGraphsContext {
    pub fn new(primary_window: Window) -> Self {
        let render_backend = RenderBackend::create_render_backend();
        let renderer = Renderer::new(render_backend);

        Self {
            primary_window,
            renderer,
        }
    }

    pub fn update(&mut self) {
        println!("InitializedGraphsContext update");

        self.renderer.render();
    }

    pub fn redraw_requested(&mut self) {
        self.primary_window.request_redraw();
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
