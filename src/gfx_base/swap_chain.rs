use std::fmt::Debug;

use crate::gfx_base::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

pub trait SwapChainTrait: 'static + Debug {
    fn present(&mut self);
}

#[derive(Debug)]
pub struct SwapChain {
    desc: SwapChainDescriptor,
    boxed: Box<dyn SwapChainTrait>,
}

impl SwapChain {
    pub fn new<T: SwapChainTrait>(desc: SwapChainDescriptor, swap_chain: T) -> Self {
        SwapChain {
            desc,
            boxed: Box::new(swap_chain),
        }
    }

    pub fn present(&mut self) {
        self.boxed.present();
    }
}

impl FGResource for SwapChain {
    type Descriptor = SwapChainDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::OwnedSwapChain(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn borrow_resource_mut(res: &mut AnyFGResource) -> &mut Self {
        match res {
            AnyFGResource::OwnedSwapChain(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn get_desc(&self) -> &Self::Descriptor {
        &self.desc
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SwapChainDescriptor;

impl From<SwapChainDescriptor> for AnyFGResourceDescriptor {
    fn from(value: SwapChainDescriptor) -> Self {
        AnyFGResourceDescriptor::SwapChain(value)
    }
}

impl FGResourceDescriptor for SwapChainDescriptor {
    type Resource = SwapChain;
}
