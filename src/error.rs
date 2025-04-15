use thiserror::Error;

#[derive(Error, Debug)]
pub enum BumpVersionError {
    #[error("No source specified")]
    NoSourceSpecified,

    #[error("Git: {0}")]
    Git(#[from] git2::Error),

    #[error("Read/Write: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse/Toml: {0}")]
    TomlRead(#[from] toml::de::Error),

    #[error("Write/Toml: {0}")]
    TomlWrite(#[from] toml::ser::Error),

    #[error("Parse/Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(&'static str),
}
