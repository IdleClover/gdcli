use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Shell not found")]
    NotFound,

    #[error("Auto installation not supported")]
    AutoInstallationNotSupported,
}
