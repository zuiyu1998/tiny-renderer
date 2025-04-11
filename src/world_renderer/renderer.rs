use super::FrameGraphContext;

pub trait Renderer {
    fn prepare(&self, context: &mut FrameGraphContext);
}
