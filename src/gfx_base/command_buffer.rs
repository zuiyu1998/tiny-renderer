use downcast::{Any, downcast};
pub struct CommandBuffer(Box<dyn CommandBufferTrait>);

pub trait CommandBufferTrait: 'static + Any {}

downcast!(dyn CommandBufferTrait);

impl CommandBuffer {
    pub fn new<T: CommandBufferTrait>(command_buffer: T) -> Self {
        CommandBuffer(Box::new(command_buffer))
    }

    pub fn downcast<T: CommandBufferTrait>(self) -> Option<Box<T>> {
        let value: Option<Box<T>> = self.0.downcast::<T>().ok();
        value
    }
}
