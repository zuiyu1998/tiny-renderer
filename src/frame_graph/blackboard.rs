use std::{
    collections::{hash_map, HashMap},
    sync::Arc,
};

use crate::renderer::resource::{Buffer, Image, ImageDescriptor};

use super::{
    ExportableGraphResource, FrameGraph, RenderResource, RenderResourceDescriptor,
    ResourceNodeHandle, TypeEquals,
};

//用于存储超过一帧的资源，在渲染之前的阶段用于初始化frame graph,在渲染之后的阶段保存数据，避免多次内存申请。
#[derive(Default)]
pub struct Blackboard {
    pub(crate) resources: HashMap<BlackboardResourceKey, BlackboardResourceState>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct BlackboardResourceKey(String);

impl<'a> From<&'a str> for BlackboardResourceKey {
    fn from(s: &'a str) -> Self {
        BlackboardResourceKey(String::from(s))
    }
}

impl From<String> for BlackboardResourceKey {
    fn from(s: String) -> Self {
        BlackboardResourceKey(s)
    }
}

#[derive(Clone)]
pub(crate) enum BlackboardResource {
    Image(Arc<Image>),
    Buffer(Arc<Buffer>),
}

pub(crate) enum BlackboardResourceState {
    Inert {
        resource: BlackboardResource,
    },
    Imported {
        resource: BlackboardResource,
        handle: ExportableGraphResource,
    },
    Exported {
        resource: BlackboardResource,
        // handle: ExportedResource,
    },
}

pub trait PutResourceNode<D: RenderResourceDescriptor> {
    fn put(
        &mut self,
        key: impl Into<BlackboardResourceKey>,
        desc: D,
    ) -> ResourceNodeHandle<<D as RenderResourceDescriptor>::Resource>
    where
        D: TypeEquals<
            Other = <<D as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
        >;
}

impl PutResourceNode<ImageDescriptor> for FrameGraph {
    fn put(
        &mut self,
        key: impl Into<BlackboardResourceKey>,
        desc: ImageDescriptor,
    ) -> ResourceNodeHandle<<ImageDescriptor as RenderResourceDescriptor>::Resource>
    where
        ImageDescriptor: TypeEquals<
            Other = <<ImageDescriptor as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
    >{
        let key = key.into();

        todo!()
    }
}
