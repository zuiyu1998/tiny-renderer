use std::fmt::Debug;

use wgpu::TextureView;

use crate::frame_graph::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

pub trait SwapChainTrait: 'static + Debug {
    fn present(&self);

    fn get_texture_view(&self) -> TextureView;
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

    pub fn present(&self) {
        self.boxed.present();
    }

    pub fn get_texture_view(&self) -> TextureView {
        self.boxed.get_texture_view()
    }
}

impl FGResource for SwapChain {
    type Descriptor = SwapChainDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::ImportedSwapChain(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn borrow_resource_mut(res: &mut AnyFGResource) -> &mut Self {
        match res {
            AnyFGResource::ImportedSwapChain(_res) => {
                unimplemented!()
            }
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
pub struct SwapChainDescriptor {}

impl From<SwapChainDescriptor> for AnyFGResourceDescriptor {
    fn from(value: SwapChainDescriptor) -> Self {
        AnyFGResourceDescriptor::SwapChain(value)
    }
}

impl FGResourceDescriptor for SwapChainDescriptor {
    type Resource = SwapChain;
}
