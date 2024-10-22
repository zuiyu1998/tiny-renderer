use std::{
    collections::{hash_map, HashMap},
    sync::Arc,
};

use crate::{
    error::{Kind, Result},
    render_backend::RenderBackend,
    renderer::resource::{Buffer, BufferDescriptor, Image, ImageDescriptor, SwapchainImages},
};

use super::{
    pass_builder::PassBuilder, FrameGraph, RenderResource, RenderResourceDescriptor,
    ResourceNodeHandle, TypeEquals,
};

pub struct TemporalFrameGraph {
    pub state: TemporalFrameGraphState,
    pub(crate) frame_graph: FrameGraph,
    pub render_backend: RenderBackend,
}

impl TemporalFrameGraph {
    pub fn new(render_backend: RenderBackend) -> Self {
        TemporalFrameGraph {
            state: Default::default(),
            frame_graph: Default::default(),
            render_backend,
        }
    }

    pub fn add_pass_node<'a>(
        &'a mut self,
        name: &str,
        insert_point: Option<u32>,
    ) -> PassBuilder<'a> {
        PassBuilder::new(self, name, insert_point)
    }

    pub fn compile(&mut self) {
        self.frame_graph.compile();
    }

    pub fn execute(&mut self, swapchain_images: SwapchainImages) {
        self.frame_graph
            .execute(&self.render_backend, swapchain_images);
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

pub(crate) struct TemporalResourceState {
    resource: TemporalResource,
}

pub trait PutResourceNode<D: RenderResourceDescriptor> {
    fn put(
        &mut self,
        key: impl Into<TemporalResourceKey>,
        descriptor: D,
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
    ) -> Result<ResourceNodeHandle<<ImageDescriptor as RenderResourceDescriptor>::Resource>>
    where
        ImageDescriptor: TypeEquals<
            Other = <<ImageDescriptor as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
    >{
        let key = key.into();

        match self.state.resources.entry(key.clone()) {
            hash_map::Entry::Occupied(entry) => {
                let state = entry.get();
                let resource = state.resource.clone();

                match &resource {
                    TemporalResource::Image(image) => {
                        let handle = self.frame_graph.import(image.clone());

                        Ok(handle)
                    }
                    TemporalResource::Buffer(_) => Err(Kind::ResourceTypeNoMatch.into()),
                }
            }
            hash_map::Entry::Vacant(entry) => {
                let resource = Arc::new(descriptor.create_resource(&self.render_backend.device));
                let handle = self.frame_graph.import(resource.clone());
                entry.insert(TemporalResourceState {
                    resource: TemporalResource::Image(resource),
                });
                Ok(handle)
            }
        }
    }
}

impl PutResourceNode<BufferDescriptor> for TemporalFrameGraph {
    fn put(
        &mut self,
        key: impl Into<TemporalResourceKey>,
        descriptor: BufferDescriptor,
    ) -> Result<ResourceNodeHandle<<BufferDescriptor as RenderResourceDescriptor>::Resource>>
    where
        ImageDescriptor: TypeEquals<
            Other = <<ImageDescriptor as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
    >{
        let key = key.into();

        match self.state.resources.entry(key.clone()) {
            hash_map::Entry::Occupied(entry) => {
                let state = entry.get();
                let resource = state.resource.clone();

                match &resource {
                    TemporalResource::Buffer(image) => {
                        let handle = self.frame_graph.import(image.clone());

                        Ok(handle)
                    }
                    TemporalResource::Image(_) => Err(Kind::ResourceTypeNoMatch.into()),
                }
            }
            hash_map::Entry::Vacant(entry) => {
                let resource = Arc::new(descriptor.create_resource(&self.render_backend.device));
                let handle = self.frame_graph.import(resource.clone());
                entry.insert(TemporalResourceState {
                    resource: TemporalResource::Buffer(resource),
                });
                Ok(handle)
            }
        }
    }
}
