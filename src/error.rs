use thiserror::Error;

#[derive(Error, Debug)]
pub enum BumpVersionError {
    #[error("No source specified")]
    SourceNotSpecified,

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("Invalid version format: {0}")]
    InvalidVersion(String),

    #[error("Invalid file format: {0}")]
    InvalidFileFormat(String),

    #[error("Git: {0}")]
    Git(#[from] git2::Error),

    #[error("Read/Write: {0}")]
    Io(std::io::Error),

    #[error("Parse/Toml: {0}")]
    TomlRead(#[from] toml::de::Error),

    #[error("Write/Toml: {0}")]
    TomlWrite(#[from] toml::ser::Error),

    #[error("Parse/Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(&'static str),
}

impl From<std::io::Error> for BumpVersionError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => BumpVersionError::FileNotFound(
                err.to_string().replace("No such file or directory", "").trim().to_string()
            ),
            _ => BumpVersionError::Io(err),
        }
    }
}