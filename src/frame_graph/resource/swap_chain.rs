use std::{fmt::Debug, sync::Arc};

use crate::{
    frame_graph::{
        AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor, FrameGraph,
        ImportToFrameGraph, ResourceNodeHandle,
    },
    gfx_base::texture_view::TextureView,
};

use super::ImportedResource;

pub trait SwapChainTrait: 'static + Debug + Send + Sync {
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

impl ImportToFrameGraph for SwapChain {
    fn import(
        self: Arc<Self>,
        name: &str,
        desc: Self::Descriptor,
        fg: &mut FrameGraph,
    ) -> ResourceNodeHandle<Self> {
        fg._import::<SwapChain>(name, ImportedResource::SwapChain(self), desc)
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
