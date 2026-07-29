use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("")]
    CreateDirsFailed(#[source] std::io::Error),

    #[error("Failed to create {} file", .path.display())]
    CreateFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write to {}", .path.display())]
    WriteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read {}", .path.display())]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to rename {} file to {}", .from.display(), .to.display())]
    RenameFailed {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to determine current directory")]
    CurrentDirUnavailable(#[source] std::io::Error),
}
