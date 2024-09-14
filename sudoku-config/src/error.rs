use strum_macros::IntoStaticStr;
use thiserror::Error;

#[derive(Error, Debug, IntoStaticStr)]
pub enum ConfigError {
    #[error("Failed to locate config dir")]
    NoConfigDir,
    #[error("Failed to extract configurations: {0}")]
    FigmentError(#[from] figment::Error),
    #[error("IO Operation failed: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to convert yaml: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Not implemented")]
    NotImplemented,
    #[error("Puzzle already exists")]
    DuplicatePuzzle,
}

pub type ConfigResult<T> = Result<T, ConfigError>;
