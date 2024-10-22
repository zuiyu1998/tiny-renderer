mod buffer;
mod image;

use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use crate::{
    render_backend::RenderDevice,
    renderer::resource::{Buffer, BufferDescriptor, Image, ImageDescriptor},
};

use super::FrameGraph;

#[derive(Debug)]
pub struct Ref<ResType: RenderResource, ViewType: GpuViewType> {
    pub(crate) handle: VirtualResourceHandle,
    pub(crate) descriptor: <ResType as RenderResource>::Descriptor,
    pub(crate) marker: PhantomData<(ResType, ViewType)>,
}

impl<ResType: RenderResource, ViewType: GpuViewType> Ref<ResType, ViewType> {
    pub fn descriptor(&self) -> &<ResType as RenderResource>::Descriptor {
        &self.descriptor
    }
}

impl<ResType: RenderResource, ViewType: GpuViewType> Clone for Ref<ResType, ViewType>
where
    <ResType as RenderResource>::Descriptor: Clone,
    ViewType: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            descriptor: self.descriptor.clone(),
            marker: PhantomData,
        }
    }
}

impl<ResType: RenderResource, ViewType: GpuViewType> Copy for Ref<ResType, ViewType>
where
    <ResType as RenderResource>::Descriptor: Copy,
    ViewType: Copy,
{
}

pub trait GpuViewType {
    const IS_WRITABLE: bool;
}

#[derive(Clone, Copy)]
pub struct GpuSrv;
pub struct GpuUav;

impl GpuViewType for GpuSrv {
    const IS_WRITABLE: bool = false;
}
impl GpuViewType for GpuUav {
    const IS_WRITABLE: bool = true;
}

pub trait ImportToFrameGraph
where
    Self: RenderResource + Sized,
{
    fn import(self: Arc<Self>, fg: &mut FrameGraph) -> ResourceNodeHandle<Self>;
}

///渲染资源的抽象实例，因为渲染资源通常是固定的，不需要外部扩展。
pub enum AnyRenderResource {
    OwnedBuffer(Buffer),
    ImportedBuffer(Arc<Buffer>),
    OwnedImage(Image),
    ImportedImage(Arc<Image>),
    Pending,
}

impl AnyRenderResource {
    pub fn borrow(&self) -> AnyRenderResourceRef {
        match self {
            AnyRenderResource::OwnedBuffer(buffer) => AnyRenderResourceRef::Buffer(buffer),
            AnyRenderResource::ImportedBuffer(buffer) => {
                AnyRenderResourceRef::Buffer(buffer.as_ref())
            }
            AnyRenderResource::OwnedImage(image) => AnyRenderResourceRef::Image(image),
            AnyRenderResource::ImportedImage(image) => AnyRenderResourceRef::Image(image.as_ref()),
            AnyRenderResource::Pending => {
                unimplemented!()
            }
        }
    }
}

pub enum AnyRenderResourceRef<'a> {
    Image(&'a Image),
    Buffer(&'a Buffer),
}

#[derive(Clone)]
pub enum AnyRenderResourceDescriptor {
    Buffer(BufferDescriptor),
    Image(ImageDescriptor),
    SwapchainImage,
}

impl AnyRenderResourceDescriptor {
    pub fn create_resource(&self, device: &RenderDevice) -> AnyRenderResource {
        match self {
            AnyRenderResourceDescriptor::Buffer(buffer) => {
                AnyRenderResource::OwnedBuffer(buffer.create_resource(device))
            }
            AnyRenderResourceDescriptor::Image(buffer) => {
                AnyRenderResource::OwnedImage(buffer.create_resource(device))
            }
            _ => AnyRenderResource::Pending,
        }
    }
}

///描述渲染资源如何被创建
pub trait RenderResourceDescriptor: Clone + Debug + Into<AnyRenderResourceDescriptor> {
    type Resource: RenderResource;

    fn create_resource(&self, device: &RenderDevice) -> Self::Resource;
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualResourceHandle {
    pub id: u32,
    pub version: u32,
}

///资源节点的Handle
pub struct VirtualResource {
    pub id: u32,
    pub version: u32,
    //该资源被使用的次数
    pub ref_count: u32,
    //该资源被写入的次数
    pub writer_count: u32,
    //是否为导入
    pub imported: bool,
    pub never_loaded: bool,
    pub never_stored: bool,
    pub memory_less: bool,
    pub memory_less_msaa: bool,
    pub first_pass: Option<usize>,
    pub last_pass: Option<usize>,
}

impl VirtualResource {
    pub fn get_handle(&self) -> VirtualResourceHandle {
        VirtualResourceHandle {
            id: self.id,
            version: self.version,
        }
    }
}

impl VirtualResource {
    pub fn update_life_time(&mut self, pass_index: usize) {
        if self.first_pass.is_none() {
            self.first_pass = Some(pass_index);
        }

        self.last_pass = Some(
            self.last_pass
                .map(|last_access| last_access.max(pass_index))
                .unwrap_or(pass_index),
        );
    }

    pub fn new_version(&mut self) {
        self.version += 1;
    }
}

impl Default for VirtualResource {
    fn default() -> Self {
        Self {
            id: 0,
            version: 0,
            ref_count: 0,
            writer_count: 0,
            imported: false,
            never_loaded: true,
            never_stored: true,
            memory_less: false,
            memory_less_msaa: false,
            first_pass: None,
            last_pass: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawResourceNodeHandle {
    pub index: u32,
}

pub struct ResourceNodeHandle<R: RenderResource> {
    pub raw: RawResourceNodeHandle,
    pub descriptor: <R as RenderResource>::Descriptor,
    pub marker: PhantomData<R>,
}

///资源节点
/// 一类是graph自己管理的节点
/// 一类是从外部导入的节点
pub struct ResourceNode {
    pub info: GraphResourceInfo,
    pub reader_count: u32,
    pub writer: Option<u32>,
}

impl ResourceNode {
    pub fn new(info: GraphResourceInfo) -> Self {
        Self {
            info,
            reader_count: 0,
            writer: None,
        }
    }

    pub fn created(info: GraphResourceCreateInfo) -> Self {
        Self::new(GraphResourceInfo::Created(info))
    }

    pub fn imported(info: GraphResourceImportInfo) -> Self {
        Self::new(GraphResourceInfo::Imported(info))
    }
}

#[derive(Clone)]
pub enum GraphResourceInfo {
    Created(GraphResourceCreateInfo),
    Imported(GraphResourceImportInfo),
}

#[derive(Clone)]

pub struct GraphResourceCreateInfo {
    pub desciptor: AnyRenderResourceDescriptor,
}

#[derive(Clone)]

pub enum GraphResourceImportInfo {
    Image { resource: Arc<Image> },
    Buffer { resource: Arc<Buffer> },
}

impl GraphResourceImportInfo {
    pub fn imported(&self) -> AnyRenderResource {
        match self {
            GraphResourceImportInfo::Buffer { resource } => {
                AnyRenderResource::ImportedBuffer(resource.clone())
            }
            GraphResourceImportInfo::Image { resource } => {
                AnyRenderResource::ImportedImage(resource.clone())
            }
        }
    }
}

///渲染资源
/// Descriptor,渲染资源对应的描述
/// 是否可以使用dyn
pub trait RenderResource {
    type Descriptor: RenderResourceDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self;
}
