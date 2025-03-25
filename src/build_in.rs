use std::sync::LazyLock;

use fyrox_resource::{embedded_data_source, manager::BuiltInResource};

use crate::gfx_base::shader::{Shader, ShaderResource};

static STANDARD: LazyLock<BuiltInResource<Shader>> = LazyLock::new(|| {
    BuiltInResource::new(embedded_data_source!("embedded/shader.wgsl"), |data| {
        ShaderResource::new_ok("test".into(), Shader::from_string_bytes(data))
    })
});

pub fn get_test() -> &'static ShaderResource {
   &STANDARD.resource
}
