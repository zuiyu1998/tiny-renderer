use std::collections::HashMap;

use super::RawGraphResourceHandle;

#[derive(Default)]
pub struct ResourceBoard {
    resources: HashMap<String, RawGraphResourceHandle>,
}

impl ResourceBoard {
    pub fn put(&mut self, name: &str, handle: RawGraphResourceHandle) {
        self.resources.insert(name.to_owned(), handle);
    }

    pub fn get(&self, name: &str) -> Option<&RawGraphResourceHandle> {
        self.resources.get(name)
    }
}
