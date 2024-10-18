pub mod resource;

use crate::render_backend::RenderBackend;

pub struct BaseRenderer {}

pub struct Renderer {
    base_renderer: BaseRenderer,
}

impl Renderer {
    pub fn new(_backend: &RenderBackend) -> Self {
        Self {
            base_renderer: BaseRenderer {},
        }
    }
}
