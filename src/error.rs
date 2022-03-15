use inquire::error::InquireError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Args(String),

    #[error("{0}")]
    CommandExecutionError(std::io::Error),

    #[error("{0}")]
    CommandNotFoundError(String),

    #[error("{0}")]
    InquireError(InquireError),

    #[error("{0}")]
    OsError(String),

    #[error("{0}")]
    ShellNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
