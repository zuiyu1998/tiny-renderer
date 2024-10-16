mod resource;

use resource::ResourceNode;
use std::{cell::UnsafeCell, rc::Rc};

///pass 节点
pub struct PassNode {}

///FrameGraph是一个有向无环图，用于渲染数据的整合，cocos的rust版本
pub struct FrameGraph {
    pass_nodes: Rc<UnsafeCell<PassNode>>,
    resource_nodes: Rc<UnsafeCell<ResourceNode>>,
}

impl FrameGraph {
    ///compile阶段
    pub fn compile(&mut self) {}

    ///execute阶段
    pub fn execute(&mut self) {}
}
