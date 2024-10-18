pub mod frame_graph;
pub mod render_backend;
pub mod renderer;

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
        let render_backend = RenderBackend::create_render_backend(&primary_window);
        let renderer = Renderer::new(render_backend);

        Self {
            primary_window,
            renderer,
        }
    }

    pub fn update(&mut self) {}
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
}
