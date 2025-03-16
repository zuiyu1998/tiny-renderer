use std::collections::HashMap;

use super::GraphResourceHandle;

#[derive(Default)]
pub struct ResourceBoard {
    resources: HashMap<String, GraphResourceHandle>,
}

impl ResourceBoard {
    pub fn put(&mut self, name: &str, handle: GraphResourceHandle) {
        self.resources.insert(name.to_owned(), handle);
    }

    pub fn get(&self, name: &str) -> Option<&GraphResourceHandle> {
        self.resources.get(name)
    }
}
