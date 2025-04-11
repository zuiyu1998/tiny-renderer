use std::fmt::Debug;

use crate::{
    frame_graph::{AnyResource, AnyResourceDescriptor, Resource, ResourceDescriptor},
    gfx_base::texture_view::TextureView,
};

pub trait SwapChainTrait: 'static + Debug + Send + Sync {
    fn present(&self);

    fn get_texture_view(&self) -> TextureView;
}

#[derive(Debug)]
pub struct SwapChain {
    desc: SwapChainInfo,
    boxed: Box<dyn SwapChainTrait>,
}

impl SwapChain {
    pub fn new<T: SwapChainTrait>(desc: SwapChainInfo, swap_chain: T) -> Self {
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

    pub fn get_desc(&self) -> &SwapChainInfo {
        &self.desc
    }
}

impl Resource for SwapChain {
    type Descriptor = SwapChainInfo;

    fn borrow_resource(res: &AnyResource) -> &Self {
        match res {
            AnyResource::OwnedSwapChain(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SwapChainInfo;

impl From<SwapChainInfo> for AnyResourceDescriptor {
    fn from(value: SwapChainInfo) -> Self {
        AnyResourceDescriptor::SwapChain(value)
    }
}

impl ResourceDescriptor for SwapChainInfo {
    type Resource = SwapChain;
}
