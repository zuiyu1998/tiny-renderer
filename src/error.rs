use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("ResourceTypeNoMatch")]
    ResourceTypeNoMatch,
    #[error("ResourceAlreadyTaken")]
    ResourceAlreadyTaken,
    #[error("ResourceUninitialized")]
    ResourceUninitialized,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Kind: {0}")]
    Kind(#[from] Kind),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
