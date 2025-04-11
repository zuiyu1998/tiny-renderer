use std::sync::Arc;

use super::{
    DevicePass, ImportToFrameGraph, PassNode, RenderContext, Resource, ResourceBoard,
    ResourceDescriptor, ResourceInfo, ResourceNode, ResourceNodeHandle, TypeEquals,
    VirtualResource, pass_node_builder::PassNodeBuilder,
};
use crate::gfx_base::handle::TypeHandle;

#[derive(Default)]
pub struct FrameGraph {
    pub(crate) pass_nodes: Vec<PassNode>,
    resources: Vec<VirtualResource>,
    resource_nodes: Vec<ResourceNode>,
    resource_board: ResourceBoard,
    device_passes: Option<Vec<DevicePass>>,
}

impl FrameGraph {
    fn reset(&mut self) {
        self.pass_nodes = vec![];
        self.resources = vec![];
        self.resource_nodes = vec![];
        self.resource_board = Default::default();
        self.device_passes = None;
    }

    pub fn execute(&mut self, render_context: &mut RenderContext) {
        if self.device_passes.is_none() {
            return;
        }

        let device_passes = self.device_passes.take().unwrap();

        for mut device_pass in device_passes {
            device_pass.execute(render_context);
        }

        self.reset();
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

    fn generate_device_passes(&mut self) {
        if self.pass_nodes.is_empty() {
            return;
        }

        let mut device_passes = vec![];

        for index in 0..self.pass_nodes.len() {
            let pass_node_handle = self.pass_nodes[index].handle;

            let mut device_pass = DevicePass::default();

            device_pass.extra(self, pass_node_handle);
            device_passes.push(device_pass);
        }

        self.device_passes = Some(device_passes);
    }

    pub fn compile(&mut self) {
        if self.pass_nodes.is_empty() {
            return;
        }

        self.sort();
        //todo cull

        self.compute_resource_lifetime();

        self.generate_device_passes();
    }
}

impl FrameGraph {
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

    pub fn get_resource_mut(
        &mut self,
        handle: &TypeHandle<VirtualResource>,
    ) -> &mut VirtualResource {
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
        let resource_handle = TypeHandle::new(self.resources.len());
        let resource: VirtualResource = VirtualResource::new_imported::<ResourceType>(
            name,
            resource_handle,
            desc,
            imported_resource,
        );

        let resource_info = resource.info.clone();
        self.resources.push(resource);

        let handle = self.create_resource_node(resource_info);

        let handle = ResourceNodeHandle::new(handle, resource_handle);

        self.resource_board.put(name, handle.raw());

        handle
    }

    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> ResourceNodeHandle<DescriptorType::Resource>
    where
        DescriptorType: ResourceDescriptor
            + TypeEquals<
                Other = <<DescriptorType as ResourceDescriptor>::Resource as Resource>::Descriptor,
            >,
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
