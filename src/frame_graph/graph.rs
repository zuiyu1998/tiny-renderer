use std::{marker::PhantomData, mem::take, sync::Arc};

use crate::render_backend::{RenderBackend, RenderDevice};

use super::{
    pass::PassNode,
    resource::{
        GraphResourceCreateInfo, ImportToFrameGraph, RenderResource, RenderResourceDescriptor,
        ResourceNode, ResourceNodeHandle, VirtualResource,
    },
    resource_registry::{RegistryResource, RenderContext},
    GraphResourceInfo, RawResourceNodeHandle,
};

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

///FrameGraph是一个有向无环图，用于渲染数据的整合
#[derive(Default)]
pub struct FrameGraph {
    pub pass_nodes: Vec<PassNode>,
    pub resource_nodes: Vec<ResourceNode>,
    pub virtual_resources: Vec<VirtualResource>,

    pub compiled_frame_fraph: Option<CompiledFrameGraph>,
}

pub struct CompiledFrameGraph {
    pub pass_nodes: Vec<PassNode>,
    pub resource_nodes: Vec<ResourceNode>,
    pub virtual_resources: Vec<VirtualResource>,
    pub registry_resources: Vec<RegistryResource>,
}

impl CompiledFrameGraph {
    pub fn execute(self, backend: &RenderBackend) {
        let CompiledFrameGraph {
            pass_nodes,
            registry_resources,
            ..
        } = self;

        let encoder = backend
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame graph encoder"),
            });

        let mut render_context = RenderContext {
            resources: registry_resources,
            encoder,
        };

        //注入屏幕

        for mut pass_node in pass_nodes.into_iter() {
            println!("{}", pass_node.name);

            //申请资源

            if let Some(render_fn) = pass_node.render_fn.take() {
                render_fn(&mut render_context);
            }

            //释放资源
        }
    }
}

impl FrameGraph {
    pub fn add_pass_node(&mut self, pass_node: PassNode) {
        self.pass_nodes.push(pass_node);
    }

    pub fn import<R: ImportToFrameGraph>(&mut self, resource: Arc<R>) -> ResourceNodeHandle<R> {
        ImportToFrameGraph::import(resource, self)
    }

    ///根据描述创建资源节点
    pub fn create<D: RenderResourceDescriptor>(
        &mut self,
        descriptor: D,
    ) -> ResourceNodeHandle<D::Resource>
    where
        D: TypeEquals<
            Other = <<D as RenderResourceDescriptor>::Resource as RenderResource>::Descriptor,
        >,
    {
        let raw = self.create_raw_resource_node(GraphResourceCreateInfo {
            desciptor: descriptor.clone().into(),
        });

        ResourceNodeHandle {
            raw,
            descriptor: TypeEquals::same(descriptor),
            marker: PhantomData,
        }
    }

    pub fn create_raw_resource_node(
        &mut self,
        info: GraphResourceCreateInfo,
    ) -> RawResourceNodeHandle {
        let index = self.resource_nodes.len() as u32;

        let raw = RawResourceNodeHandle { index };

        let virtual_resource = VirtualResource {
            id: index,
            version: 0,
            ..Default::default()
        };
        self.virtual_resources.push(virtual_resource);

        self.resource_nodes.push(ResourceNode::created(info));

        raw
    }
}

impl FrameGraph {
    ///compile阶段
    pub fn compile(&mut self) {
        //sort 对插入的pass node进行排序
        self.sort();

        //去除不需要的资源和pass
        self.cull();

        //计算生命周期
        self.compute_resource_lifetime();

        //生成CompiledFrameGraph
        self.compiled_frame_fraph();
    }

    fn compiled_frame_fraph(&mut self) {
        if self.pass_nodes.is_empty() {
            return;
        }

        let registry_resources: Vec<RegistryResource> = self
            .resource_nodes
            .iter()
            .map(|node| match &node.info {
                GraphResourceInfo::Created(created) => RegistryResource::Created(created.clone()),
                GraphResourceInfo::Imported(improted) => {
                    RegistryResource::Resource(improted.imported())
                }
            })
            .collect();

        let pass_nodes = take(&mut self.pass_nodes);
        let resource_nodes = take(&mut self.resource_nodes);
        let virtual_resources = take(&mut self.virtual_resources);

        self.compiled_frame_fraph = Some(CompiledFrameGraph {
            registry_resources,
            pass_nodes,
            resource_nodes,
            virtual_resources,
        })
    }

    fn compute_resource_lifetime(&mut self) {
        //更新资源的使用范围
        for (index, pass_node) in self.pass_nodes.iter().enumerate() {
            for read_ref in pass_node.reads.iter() {
                let read_index = read_ref.id() as usize;

                self.virtual_resources[read_index].update_life_time(index);
            }

            for write_ref in pass_node.writes.iter() {
                let write_index = write_ref.id() as usize;

                self.virtual_resources[write_index].update_life_time(index);

                self.virtual_resources[write_index].writer_count += 1;
            }
        }

        //更新pass_node中要使用资源
        for (index, resource) in self.virtual_resources.iter().enumerate() {
            if resource.first_pass.is_none() || resource.last_pass.is_none() {
                continue;
            }

            if resource.ref_count == 0 {
                continue;
            }

            self.pass_nodes[resource.first_pass.unwrap() as usize]
                .resource_request_array
                .push(index as u32);

            self.pass_nodes[resource.last_pass.unwrap() as usize]
                .resource_release_array
                .push(index as u32);
        }
    }

    //清除掉不需要使用的资源和pass
    fn cull(&mut self) {
        //初始化pass_node的引用，和资源的reader_count数目
        for pass_node in self.pass_nodes.iter_mut() {
            pass_node.ref_count = pass_node.writes.len() as u32;

            for read_ref in pass_node.reads.iter() {
                let index = read_ref.id() as usize;

                self.resource_nodes[index].reader_count += 1;
            }
        }

        let mut stack: Vec<usize> = Vec::with_capacity(self.resource_nodes.len());

        //添加只有写入，没有读取的资源节点
        for (index, resource_node) in self.resource_nodes.iter().enumerate() {
            if resource_node.reader_count == 0 && resource_node.writer.is_some() {
                stack.push(index);
            }
        }

        while !stack.is_empty() {
            let index = stack.pop().unwrap();

            let writer_pass_node = &mut self.pass_nodes[index];

            //去除资源的引用
            writer_pass_node.ref_count -= 1;

            if writer_pass_node.ref_count == 0 {
                for read_ref in writer_pass_node.reads.iter() {
                    let resource_index = read_ref.id() as usize;

                    self.resource_nodes[resource_index].reader_count -= 1;

                    //添加节点去除后，需要再次剔除的资源节点
                    if self.resource_nodes[resource_index].reader_count == 0 {
                        stack.push(resource_index);
                    }
                }
            }
        }

        //更新资源的引用
        for (index, resource_node) in self.resource_nodes.iter().enumerate() {
            self.virtual_resources[index].ref_count = resource_node.reader_count;
        }
    }

    fn sort(&mut self) {
        self.pass_nodes.sort();
    }

    ///execute阶段
    pub fn execute(&mut self, backend: &RenderBackend) {
        if let Some(frame_graph) = self.compiled_frame_fraph.take() {
            frame_graph.execute(backend);
        }
    }
}
