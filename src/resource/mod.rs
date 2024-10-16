mod buffer;
mod image;

use buffer::Buffer;
use image::Image;

///渲染资源的抽象实例，因为渲染资源通常是固定的，不需要外部扩展。
pub enum AnyRenderResource {
    OwnedBuffer(Buffer),
    ImportedBuffer(Buffer),
    OwnedImage(Image),
    ImportedImage(Image),
}

impl AnyRenderResource {
    pub fn borrow(&self) -> AnyRenderResourceRef {
        match self {
            AnyRenderResource::OwnedBuffer(buffer) => AnyRenderResourceRef::Buffer(buffer),
            _ => {
                todo!()
            }
        }
    }
}

pub enum AnyRenderResourceRef<'a> {
    Image(&'a Image),
    Buffer(&'a Buffer),
}

///描述渲染资源如何被创建
pub trait RenderResourceDescriptor {
    type Resource: RenderResource;
}

///资源节点
pub struct ResourceNode {}

///渲染资源
/// Descriptor,渲染资源对应的描述
/// 是否可以使用dyn
pub trait RenderResource {
    type Descriptor: RenderResourceDescriptor;

    fn borrow_resource(res: &AnyRenderResource) -> &Self;
}
