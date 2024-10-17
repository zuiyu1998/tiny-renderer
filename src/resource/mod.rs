mod buffer;
mod image;

use std::{marker::PhantomData, sync::Arc};

use buffer::{Buffer, BufferDescriptor};
use image::{Image, ImageDescriptor};

///渲染资源的抽象实例，因为渲染资源通常是固定的，不需要外部扩展。
pub enum AnyRenderResource {
    OwnedBuffer(Buffer),
    ImportedBuffer(Arc<Buffer>),
    OwnedImage(Image),
    ImportedImage(Arc<Image>),
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
}

///描述渲染资源如何被创建
pub trait RenderResourceDescriptor: Clone + Into<AnyRenderResourceDescriptor> {
    type Resource: RenderResource;
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

pub struct ResourceNodeHandle<R: RenderResource> {
    pub index: u32,
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

pub struct GraphResourceImportInfo {}

///渲染资源
/// Descriptor,渲染资源对应的描述
/// 是否可以使用dyn
pub trait RenderResource {
    type Descriptor: RenderResourceDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self;
}
