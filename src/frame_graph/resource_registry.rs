use super::{AnyRenderResource, GraphResourceCreateInfo};

//根据资源节点信息生成的资源
pub enum RegistryResource {
    Created(GraphResourceCreateInfo),
    Resource(AnyRenderResource),
}

pub struct RenderContext {
    resources: Vec<RegistryResource>,
}
