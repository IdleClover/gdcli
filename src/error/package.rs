use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error(transparent)]
    New(#[from] NewError),

    #[error("No package.gdcli found")]
    PackageNotFound,
}

#[derive(Debug, Error)]
pub enum NewError {
    #[error("'{}' don't exist", .0.display())]
    PathNotFound(PathBuf),

    #[error("'{}'is not a folder", .0.display())]
    NotADirectory(PathBuf),

    #[error("'{}' is already a package", .0.display())]
    AlreadyExists(PathBuf),
}
