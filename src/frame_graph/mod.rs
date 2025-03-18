mod device_pass;
mod graph;
mod pass_node;
mod pass_node_builder;
mod render_context;
mod resource;
mod resource_board;
mod resource_node;
mod resource_table;

pub use device_pass::*;
pub use graph::*;
pub use pass_node::*;
pub use render_context::*;
pub use resource::*;
pub use resource_board::*;
pub use resource_node::*;
pub use resource_table::*;

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5)
    }
}
