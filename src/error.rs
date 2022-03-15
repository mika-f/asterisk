use inquire::error::InquireError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Args(String),

    #[error("{0}")]
    InquireError(InquireError),
}

pub type Result<T> = std::result::Result<T, Error>;
