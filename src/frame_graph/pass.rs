use std::fmt::Debug;

use super::{resource_registry::RenderApi, VirtualResourceHandle};
use crate::error::Result;

type DynRenderFn = dyn FnOnce(&mut RenderApi) -> Result<()>;

#[derive(Debug)]
pub struct PassResourceRef {
    pub(crate) handle: VirtualResourceHandle,
}

impl PassResourceRef {
    pub fn id(&self) -> u32 {
        self.handle.id
    }
}

///pass 节点
pub struct PassNode {
    pub id: u32,
    pub insert_point: u32,
    pub name: String,
    //写入资源索引
    pub writes: Vec<PassResourceRef>,
    //读取资源索引
    pub reads: Vec<PassResourceRef>,

    pub ref_count: u32,

    pub resource_request_array: Vec<u32>,
    pub resource_release_array: Vec<u32>,

    pub render_fn: Option<Box<DynRenderFn>>,
}

impl Debug for PassNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PassNode")
            .field("id", &self.id)
            .field("insert_point", &self.insert_point)
            .field("name", &self.name)
            .field("writes", &self.writes)
            .field("reads", &self.reads)
            .field("ref_count", &self.ref_count)
            .field("resource_request_array", &self.resource_request_array)
            .field("resource_release_array", &self.resource_release_array)
            .finish()
    }
}

impl PassNode {
    pub fn new(id: u32, insert_point: u32, name: &str) -> Self {
        Self {
            id,
            insert_point,
            name: name.to_string(),
            writes: vec![],
            reads: vec![],
            ref_count: 0,
            resource_release_array: vec![],
            resource_request_array: vec![],
            render_fn: None,
        }
    }
}

impl Eq for PassNode {}

impl PartialEq for PassNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl PartialOrd for PassNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.insert_point.partial_cmp(&other.insert_point) {
            Some(ord) => return Some(ord),
            ord => return ord,
        }
    }
}

impl Ord for PassNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.insert_point.cmp(&other.insert_point)
    }
}

mod test {

    #[test]
    fn test_pass_node_sort() {
        use super::PassNode;

        let mut array = vec![
            PassNode::new(0, 0, "test0"),
            PassNode::new(3, 3, "test3"),
            PassNode::new(1, 1, "test1"),
            PassNode::new(2, 2, "test2"),
        ];

        array.sort();

        let points = array
            .iter()
            .map(|node| node.insert_point)
            .collect::<Vec<u32>>();

        assert_eq!(points, vec![0, 1, 2, 3]);
    }
}
