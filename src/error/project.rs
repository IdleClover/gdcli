use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error(transparent)]
    New(#[from] NewError),

    #[error("No project.gdcli found")]
    ProjectNotFound,
}

#[derive(Debug, Error)]
pub enum NewError {
    #[error("'{}' is not a folder", .0.display())]
    PathNotFound(PathBuf),

    #[error("'{}' don't exist", .0.display())]
    NotADirectory(PathBuf),

    #[error("'{}' is already a project", .0.display())]
    AlreadyExists(PathBuf),
}
