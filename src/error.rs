use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {}

pub type Result<T, E = RendererError> = std::result::Result<T, E>;
