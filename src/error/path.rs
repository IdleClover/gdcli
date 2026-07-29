use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Path {} has no parent directory", .0.display())]
    NoParent(PathBuf),

    #[error("{0}")]
    NotFound(String),
}
