pub mod build_in;
pub mod error;
pub mod frame_graph;
pub mod gfx_base;
pub mod gfx_wgpu;
pub mod graphic_context;
pub mod world_renderer;

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5)
    }
}
