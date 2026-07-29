pub mod extension;
pub mod fs;
pub mod game;
pub mod git;
pub mod path;
pub mod project;
pub mod shell;
pub mod toml;

use thiserror::Error;

use crate::error::{
    extension::ExtensionError, fs::FsError, game::GameError, git::GitError, path::PathError,
    project::ProjectError, shell::ShellError, toml::TomlError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Game(#[from] GameError),

    #[error(transparent)]
    Extension(#[from] ExtensionError),

    #[error(transparent)]
    Project(#[from] ProjectError),

    #[error(transparent)]
    Path(#[from] PathError),

    #[error(transparent)]
    Git(#[from] GitError),

    #[error(transparent)]
    Fs(#[from] FsError),

    #[error(transparent)]
    Toml(#[from] TomlError),

    #[error(transparent)]
    Shell(#[from] ShellError),
}
