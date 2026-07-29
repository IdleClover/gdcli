use thiserror::Error;

#[derive(Debug, Error)]
pub enum TomlError {
    #[error("Failed to serialize data to TOML")]
    SerializationFailed(#[source] toml::ser::Error),

    #[error("Failed to parse TOML")]
    DeserializationFailed(#[source] toml::de::Error),
}
