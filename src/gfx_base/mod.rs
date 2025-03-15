pub mod device;
pub mod handle;
pub mod render_pass;
pub mod swap_chain;
pub mod texture;
pub mod transient_resource_cache;

use std::{fmt::Debug, hash::Hash};

use swap_chain::{SwapChain, SwapChainDescriptor};
use texture::{Texture, TextureDescriptor};

pub enum RendererError {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {
    Texture(TextureDescriptor),
    SwapChain(SwapChainDescriptor),
}

pub enum AnyFGResource {
    OwnedTexture(Texture),
    OwnedSwapChain(SwapChain),
}

pub trait FGResource: 'static + Debug {
    type Descriptor: FGResourceDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self;

    fn borrow_resource_mut(res: &mut AnyFGResource) -> &mut Self;

    fn get_desc(&self) -> &Self::Descriptor;
}

pub trait FGResourceDescriptor:
    'static + Clone + Hash + Eq + Debug + Into<AnyFGResourceDescriptor>
{
    type Resource: FGResource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}
