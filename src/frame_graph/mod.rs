mod device_pass;
mod graph;
mod pass_node;
mod pass_node_builder;
mod resource;
mod resource_node;
mod resource_table;

pub use device_pass::*;
pub use graph::*;
pub use pass_node::*;
pub use resource::*;
pub use resource_node::*;
pub use resource_table::*;

use crate::gfx_base::RendererError;

pub type DynRenderFn = dyn FnOnce(&mut ResourceTable) -> Result<(), RendererError>;
