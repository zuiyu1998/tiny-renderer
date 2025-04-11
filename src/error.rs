use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("ResourceNotFound, Resource Index is: {resource_index:?}")]
    ResourceNotFound { resource_index: usize },
}

pub type Result<T, E = RendererError> = std::result::Result<T, E>;
