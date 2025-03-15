use crate::{
    AnyFGResource, AnyFGResourceDescriptor,
    swap_chain::{SwapChain, SwapChainDescriptor},
};

pub trait DeviceTrait: 'static {
    fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain;
}

pub struct Device(Box<dyn DeviceTrait>);

impl Device {
    pub fn new<T: DeviceTrait>(device: T) -> Self {
        Device(Box::new(device))
    }

    pub fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
        match desc {
            AnyFGResourceDescriptor::SwapChain(desc) => {
                AnyFGResource::OwnedSwapChain(self.create_swap_chain(desc))
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn create_swap_chain(&self, desc: SwapChainDescriptor) -> SwapChain {
        self.0.create_swap_chain(desc)
    }
}
