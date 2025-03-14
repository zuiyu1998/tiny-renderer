pub mod device;
pub mod frame_graph;
pub mod handle;
pub mod renderer;
pub mod transient_resource_cache;

use std::{fmt::Debug, hash::Hash};

pub enum RendererError {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {}

#[derive(PartialEq, Eq, Debug)]
pub enum AnyFGResource {}

pub trait FGResource: 'static + Debug {
    type Descriptor: FGResourceDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self;
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
