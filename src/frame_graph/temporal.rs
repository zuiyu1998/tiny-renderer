use std::{
    collections::{hash_map, HashMap},
    sync::Arc,
};

use crate::{
    error::{Kind, Result},
    render_backend::RenderDevice,
    renderer::resource::{Buffer, Image, ImageDescriptor},
};

use super::{
    ExportableGraphResource, FrameGraph, RenderResource, RenderResourceDescriptor,
    ResourceNodeHandle, TypeEquals,
};

#[derive(Default)]
pub struct TemporalFrameGraph {
    pub state: TemporalFrameGraphState,
    pub frame_graph: FrameGraph,
}

impl TemporalFrameGraph {
    pub fn compile(&mut self) {
        self.frame_graph.compile();
    }

    pub fn execute(&mut self) {
        self.frame_graph.execute();
    }
}

//用于存储超过一帧的资源，在渲染之前的阶段用于初始化frame graph,在渲染之后的阶段保存数据，避免多次内存申请。
#[derive(Default)]
pub struct TemporalFrameGraphState {
    pub(crate) resources: HashMap<TemporalResourceKey, TemporalResourceState>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct TemporalResourceKey(String);

impl<'a> From<&'a str> for TemporalResourceKey {
    fn from(s: &'a str) -> Self {
        TemporalResourceKey(String::from(s))
    }
}

impl From<String> for TemporalResourceKey {
    fn from(s: String) -> Self {
        TemporalResourceKey(s)
    }
}

#[derive(Clone)]
pub(crate) enum TemporalResource {
    Image(Arc<Image>),
    Buffer(Arc<Buffer>),
}

pub(crate) enum TemporalResourceState {
    Inert {
        resource: TemporalResource,
    },
    Imported {
        resource: TemporalResource,
        handle: ExportableGraphResource,
    },
    Exported {
        resource: TemporalResource,
        // handle: ExportedResource,
    },
}

pub trait PutResourceNode<D: RenderResourceDescriptor> {
    fn put(
        &mut self,
        key: impl Into<TemporalResourceKey>,
        descriptor: D,
        device: &RenderDevice,
    ) -> Result<ResourceNodeHandle<<D as RenderResourceDescriptor>::Resource>>
    where
        D: TypeEquals<
            Other = <<D as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
        >;
}

impl PutResourceNode<ImageDescriptor> for TemporalFrameGraph {
    fn put(
        &mut self,
        key: impl Into<TemporalResourceKey>,
        descriptor: ImageDescriptor,
        device: &RenderDevice,
    ) -> Result<ResourceNodeHandle<<ImageDescriptor as RenderResourceDescriptor>::Resource>>
    where
        ImageDescriptor: TypeEquals<
            Other = <<ImageDescriptor as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
    >{
        let key = key.into();

        match self.state.resources.entry(key.clone()) {
            hash_map::Entry::Occupied(mut entry) => {
                let state = entry.get_mut();

                match state {
                    TemporalResourceState::Inert { resource } => {
                        let resource = resource.clone();

                        match &resource {
                            TemporalResource::Image(image) => {
                                let handle = self.frame_graph.import(image.clone());

                                *state = TemporalResourceState::Imported {
                                    resource,
                                    handle: ExportableGraphResource::Image(
                                        handle.clone_unchecked(),
                                    ),
                                };

                                Ok(handle)
                            }
                            TemporalResource::Buffer(_) => Err(Kind::ResourceTypeNoMatch.into()),
                        }
                    }
                    TemporalResourceState::Imported { .. } => {
                        Err(Kind::ResourceAlreadyTaken.into())
                    }
                    TemporalResourceState::Exported { .. } => {
                        unreachable!()
                    }
                }
            }
            hash_map::Entry::Vacant(entry) => {
                let resource = Arc::new(descriptor.create_resource(&device));
                let handle = self.frame_graph.import(resource.clone());
                entry.insert(TemporalResourceState::Imported {
                    resource: TemporalResource::Image(resource),
                    handle: ExportableGraphResource::Image(handle.clone_unchecked()),
                });
                Ok(handle)
            }
        }
    }
}
