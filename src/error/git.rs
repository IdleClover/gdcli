use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error(transparent)]
    Git2(#[from] git2::Error),

    #[error("A repository cannot be bare")]
    BareRepository,

    #[error("The repository URL '{0}' is invalid")]
    InvalidUrl(String),

    #[error("Failed to initialize submodule '{name}'")]
    SubmoduleInitializationFailed {
        name: String,
        #[source]
        source: git2::Error,
    },
}
