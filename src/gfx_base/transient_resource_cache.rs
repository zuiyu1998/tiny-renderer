use std::collections::HashMap;

use crate::frame_graph::{AnyFGResource, AnyFGResourceDescriptor};

#[derive(Default, Debug)]
pub struct TransientResourceCache {
    resources: HashMap<AnyFGResourceDescriptor, Vec<AnyFGResource>>,
}

impl TransientResourceCache {
    pub fn get_resource(&mut self, desc: &AnyFGResourceDescriptor) -> Option<AnyFGResource> {
        if let Some(entry) = self.resources.get_mut(desc) {
            entry.pop()
        } else {
            None
        }
    }

    pub fn insert_resource(&mut self, desc: AnyFGResourceDescriptor, resource: AnyFGResource) {
        if let Some(entry) = self.resources.get_mut(&desc) {
            entry.push(resource);
        } else {
            self.resources.insert(desc, vec![resource]);
        }
    }
}
