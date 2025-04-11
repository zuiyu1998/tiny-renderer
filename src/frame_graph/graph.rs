use std::sync::Arc;

use super::{
    DevicePass, ImportedVirtualResource, PassNode, RenderContext, VirtualResource, ResourceBoard, ResourceInfo,
    ResourceNode, ResourceNodeHandle, ResourceTable, SwapChain, SwapChainDescriptor, Texture,
    TextureDescriptor, TransientResourceCache, pass_node_builder::PassNodeBuilder,
};
use crate::gfx_base::{
    buffer::{Buffer, BufferDescriptor},
    device::Device,
    handle::TypeHandle,
    pipeline::{CachedRenderPipelineId, PipelineCache, RenderPipeline, RenderPipelineDescriptor},
};

use std::{fmt::Debug, hash::Hash};

pub trait ImportToFrameGraph
where
    Self: Sized + FGResource,
{
    fn import(self: Arc<Self>) -> ImportedVirtualResource;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {
    Texture(TextureDescriptor),
    Buffer(BufferDescriptor),
    SwapChain(SwapChainDescriptor),
}

pub enum AnyFGResource {
    OwnedTexture(Texture),
    OwnedBuffer(Buffer),
    ImportedTexture(Arc<Texture>),
    ImportedBuffer(Arc<Buffer>),
    OwnedSwapChain(SwapChain),
}

pub trait FGResource: 'static {
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

pub struct ExecutingFrameGraph {
    resource_table: ResourceTable,
    device_passes: Vec<DevicePass>,
    resource_board: ResourceBoard,
    pipelines: CompiledPipelines,
}

pub struct RetiredFrameGraph {
    resource_table: ResourceTable,
}

impl RetiredFrameGraph {
    pub fn release_resources(self, transient_resource_cache: &mut TransientResourceCache) {
        self.resource_table
            .release_resources(transient_resource_cache);
    }
}

impl ExecutingFrameGraph {
    pub fn execute(mut self, device: &Device, pipeline_cache: &PipelineCache) -> RetiredFrameGraph {
        let mut render_context = RenderContext::new(
            &mut self.resource_table,
            device,
            &self.resource_board,
            pipeline_cache,
            &self.pipelines,
        );

        for i in 0..self.device_passes.len() {
            let device_pass = &mut self.device_passes[i];
            device_pass.execute(&mut render_context);
        }

        RetiredFrameGraph {
            resource_table: self.resource_table,
        }
    }

    pub fn new(
        resource_table: ResourceTable,
        device_passes: Vec<DevicePass>,
        resource_board: ResourceBoard,
        pipelines: CompiledPipelines,
    ) -> Self {
        ExecutingFrameGraph {
            resource_table,
            device_passes,
            resource_board,
            pipelines,
        }
    }
}

#[derive(Default, Debug)]
pub struct CompiledPipelines {
    pub render_pipeline_ids: Vec<CachedRenderPipelineId>,
}

pub struct CompiledFrameGraph {
    fg: FrameGraph,
    resource_table: ResourceTable,
    device_passes: Vec<DevicePass>,
    pipelines: CompiledPipelines,
}

impl CompiledFrameGraph {
    pub fn new(fg: FrameGraph, pipelines: CompiledPipelines) -> Self {
        CompiledFrameGraph {
            fg,
            resource_table: ResourceTable::default(),
            device_passes: vec![],
            pipelines,
        }
    }

    pub fn begin_execute(
        mut self,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) -> ExecutingFrameGraph {
        for index in 0..self.fg.pass_nodes.len() {
            let pass_node_handle = self.fg.pass_nodes[index].handle;

            for resource_handle in self
                .fg
                .get_pass_node(&pass_node_handle)
                .resource_request_array
                .clone()
            {
                let resource = self.fg.get_resource(&resource_handle);
                self.resource_table
                    .request_resources(resource, device, transient_resource_cache);
            }

            let mut device_pass = DevicePass::default();

            device_pass.extra(&mut self.fg, pass_node_handle);
            self.device_passes.push(device_pass);
        }

        ExecutingFrameGraph::new(
            self.resource_table,
            self.device_passes,
            self.fg.resource_board,
            self.pipelines,
        )
    }
}

#[derive(Default)]
pub struct FrameGraph {
    pub(crate) pass_nodes: Vec<PassNode>,
    resources: Vec<VirtualResource>,
    resource_nodes: Vec<ResourceNode>,
    resource_board: ResourceBoard,
    render_pipeline_descs: Vec<RenderPipelineDescriptor>,
}

impl FrameGraph {
    pub fn compiled_pipelines(&self, pipeline_cache: &mut PipelineCache) -> CompiledPipelines {
        let render_pipeline_ids: Vec<CachedRenderPipelineId> = self
            .render_pipeline_descs
            .iter()
            .map(|desc| pipeline_cache.register_render_pipeline(desc.clone()))
            .collect();

        CompiledPipelines {
            render_pipeline_ids,
        }
    }

    pub fn compute_resource_lifetime(&mut self) {
        for pass_node in self.pass_nodes.iter_mut() {
            //更新渲染节点读取的资源节点所指向资源的生命周期
            for resource_node_handle in pass_node.reads.iter() {
                let resource_node = &self.resource_nodes[resource_node_handle.index()];
                let resource = &mut self.resources[resource_node.resource_handle.index()];
                resource.info.update_lifetime(pass_node.handle);
            }

            //更新渲染节点吸入的资源节点所指向资源的生命周期
            for resource_node_handle in pass_node.writes.iter() {
                let resource_node = &self.resource_nodes[resource_node_handle.index()];
                let resource = &mut self.resources[resource_node.resource_handle.index()];
                resource.info.update_lifetime(pass_node.handle);
            }
        }

        //更新pass_node中资源使用的索引顺序
        for resource_index in 0..self.resources.len() {
            let resource = &self.resources[resource_index];
            let info = resource.info.clone();

            if info.first_pass_node_handle.is_none() || info.last_pass_node_handle.is_none() {
                continue;
            }

            let first_pass_node_handle = info.first_pass_node_handle.unwrap();
            let first_pass_node = &mut self.pass_nodes[first_pass_node_handle.index()];
            first_pass_node.resource_request_array.push(info.handle);

            let last_pass_node_handle = info.last_pass_node_handle.unwrap();
            let last_pass_node = &mut self.pass_nodes[last_pass_node_handle.index()];
            last_pass_node.resource_release_array.push(info.handle);
        }
    }

    fn sort(&mut self) {
        self.pass_nodes
            .sort_by(|a, b| a.insert_point.cmp(&b.insert_point));
    }

    pub fn compile(mut self, pipeline_cache: &mut PipelineCache) -> Option<CompiledFrameGraph> {
        if self.pass_nodes.is_empty() {
            return None;
        }

        self.sort();
        //todo cull

        self.compute_resource_lifetime();

        let pipelines = self.compiled_pipelines(pipeline_cache);

        Some(CompiledFrameGraph::new(self, pipelines))
    }
}

impl FrameGraph {
    pub fn register_render_pipeline(
        &mut self,
        desc: RenderPipelineDescriptor,
    ) -> TypeHandle<RenderPipeline> {
        if let Some(index) = self
            .render_pipeline_descs
            .iter()
            .enumerate()
            .find(|(_index, render_pipeline_desc)| **render_pipeline_desc == desc)
            .map(|(index, _)| index)
        {
            TypeHandle::new(index)
        } else {
            let index = self.render_pipeline_descs.len();
            self.render_pipeline_descs.push(desc);

            TypeHandle::new(index)
        }
    }

    pub fn get_resource_board(&self) -> &ResourceBoard {
        &self.resource_board
    }

    pub fn get_pass_node_mut(&mut self, handle: &TypeHandle<PassNode>) -> &mut PassNode {
        &mut self.pass_nodes[handle.index()]
    }

    pub fn get_pass_node(&self, handle: &TypeHandle<PassNode>) -> &PassNode {
        &self.pass_nodes[handle.index()]
    }

    pub fn get_resource_node(&self, handle: &TypeHandle<ResourceNode>) -> &ResourceNode {
        &self.resource_nodes[handle.index()]
    }

    pub fn get_resource_node_mut(
        &mut self,
        handle: &TypeHandle<ResourceNode>,
    ) -> &mut ResourceNode {
        &mut self.resource_nodes[handle.index()]
    }

    pub fn get_resource(&self, handle: &TypeHandle<VirtualResource>) -> &VirtualResource {
        &self.resources[handle.index()]
    }

    pub fn get_resource_mut(&mut self, handle: &TypeHandle<VirtualResource>) -> &mut VirtualResource {
        &mut self.resources[handle.index()]
    }

    pub fn create_pass_node_builder(&mut self, insert_point: u32, name: &str) -> PassNodeBuilder {
        PassNodeBuilder::new(insert_point, name, self)
    }

    pub(crate) fn create_resource_node(
        &mut self,
        resource_info: ResourceInfo,
    ) -> TypeHandle<ResourceNode> {
        let resource_handle = resource_info.handle;
        let version = resource_info.version();

        let handle = TypeHandle::new(self.resource_nodes.len());

        self.resource_nodes
            .push(ResourceNode::new(handle, resource_handle, version));

        handle
    }

    pub fn import<ResourceType>(
        &mut self,
        name: &str,
        resource: Arc<ResourceType>,
        desc: ResourceType::Descriptor,
    ) -> ResourceNodeHandle<ResourceType>
    where
        ResourceType: ImportToFrameGraph,
    {
        let imported_resource = ImportToFrameGraph::import(resource);
        self.imported(name, imported_resource, desc)
    }

    pub(crate) fn imported<ResourceType>(
        &mut self,
        name: &str,
        imported_resource: ImportedVirtualResource,
        desc: ResourceType::Descriptor,
    ) -> ResourceNodeHandle<ResourceType>
    where
        ResourceType: FGResource,
    {
        let resource_handle = TypeHandle::new(self.resources.len());
        let resource: VirtualResource =
            VirtualResource::new_imported::<ResourceType>(name, resource_handle, desc, imported_resource);

        let resource_info = resource.info.clone();
        self.resources.push(resource);

        let handle = self.create_resource_node(resource_info);

        let handle = ResourceNodeHandle::new(handle, resource_handle);

        self.resource_board.put(name, handle.raw());

        handle
    }

    pub fn create<DescriptorType>(&mut self, name: &str, desc: DescriptorType) -> ResourceNodeHandle<DescriptorType::Resource>
    where
        DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        let resource_handle = TypeHandle::new(self.resources.len());

        let resource: VirtualResource = VirtualResource::new_setuped::<DescriptorType::Resource>(
            name,
            resource_handle,
            TypeEquals::same(desc),
        );

        let resource_info = resource.info.clone();
        self.resources.push(resource);

        let handle = self.create_resource_node(resource_info);

        ResourceNodeHandle::new(handle, resource_handle)
    }
}
