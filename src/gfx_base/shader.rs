use std::{borrow::Cow, sync::Arc};

use fyrox_core::{
    TypeUuidProvider, Uuid, reflect::prelude::*, sparse::AtomicIndex, uuid, visitor::prelude::*,
};
use fyrox_resource::{Resource, ResourceData};

pub const SHADER_RESOURCE_UUID: Uuid = uuid!("f1346417-b726-492a-b80f-c02096c6c019");

pub type ShaderResource = Resource<Shader>;

#[derive(Default, Debug, Reflect, Visit)]
pub struct Shader {
    /// Shader definition contains description of properties and render passes.
    #[visit(optional)]
    pub definition: ShaderDefinition,

    #[reflect(hidden)]
    #[visit(skip)]
    pub(crate) cache_index: Arc<AtomicIndex>,
}

impl Shader {
    pub fn from_string_bytes(bytes: &[u8]) -> Self {
        Self {
            definition: ShaderDefinition::from_wgsl(
                std::str::from_utf8(bytes).unwrap().to_owned(),
                "test",
            ),
            cache_index: Default::default(),
        }
    }
}

#[derive(Default, Debug, Reflect, Visit, Clone)]
pub struct ShaderDefinition {
    pub path: String,
    pub source: Source,
    pub shader_defs: Vec<ShaderDefVal>,
    pub import_path: ShaderImport,
    pub imports: Vec<ShaderImport>,
}

impl ShaderDefinition {
    fn preprocess(source: &str, path: &str) -> (ShaderImport, Vec<ShaderImport>) {
        let (import_path, imports, _) = naga_oil::compose::get_preprocessor_data(source);

        let import_path = import_path
            .map(ShaderImport::Custom)
            .unwrap_or_else(|| ShaderImport::AssetPath(path.to_owned()));

        let imports = imports
            .into_iter()
            .map(|import| {
                if import.import.starts_with('\"') {
                    let import = import
                        .import
                        .chars()
                        .skip(1)
                        .take_while(|c| *c != '\"')
                        .collect();
                    ShaderImport::AssetPath(import)
                } else {
                    ShaderImport::Custom(import.import)
                }
            })
            .collect();

        (import_path, imports)
    }

    pub fn from_wgsl(
        source: impl Into<Cow<'static, str>>,
        path: impl Into<String>,
    ) -> ShaderDefinition {
        let source = source.into();
        let path = path.into();
        let (import_path, imports) = ShaderDefinition::preprocess(&source, &path);
        ShaderDefinition {
            path,
            imports,
            import_path,
            source: Source::Wgsl(source.to_string()),
            shader_defs: Default::default(),
        }
    }
}

impl TypeUuidProvider for Shader {
    fn type_uuid() -> Uuid {
        SHADER_RESOURCE_UUID
    }
}

impl ResourceData for Shader {
    fn type_uuid(&self) -> fyrox_core::Uuid {
        <Shader as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Reflect, Visit)]
pub enum ShaderImport {
    AssetPath(String),
    Custom(String),
}

impl Default for ShaderImport {
    fn default() -> Self {
        Self::Custom("".to_string())
    }
}

impl ShaderImport {
    pub fn module_name(&self) -> Cow<'_, String> {
        match self {
            ShaderImport::AssetPath(s) => Cow::Owned(format!("\"{s}\"")),
            ShaderImport::Custom(s) => Cow::Borrowed(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Visit)]
pub enum Source {
    Wgsl(String),
}

impl Default for Source {
    fn default() -> Self {
        Self::Wgsl("".to_string())
    }
}

impl Source {
    pub fn as_str(&self) -> &str {
        match self {
            Source::Wgsl(s) => s,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Reflect, Visit)]
pub enum ShaderDefVal {
    Bool(String, bool),
    Int(String, i32),
    UInt(String, u32),
}

impl Default for ShaderDefVal {
    fn default() -> Self {
        Self::Bool("".to_string(), false)
    }
}

impl From<&str> for ShaderDefVal {
    fn from(key: &str) -> Self {
        ShaderDefVal::Bool(key.to_string(), true)
    }
}
