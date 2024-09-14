use thiserror::Error;

#[derive(Error, Debug)]
pub enum PuzzleFileError {
    #[error("Failed to load puzzlefile: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid puzzlefile: Cell at index {0} is invalid: {1}")]
    InvalidCell(usize, u8),

    #[error("Invalid puzzlefile: Invalid header")]
    InvalidHeader,

    #[error("Serialization Error")]
    SerError,

    #[error("Deserialization Error")]
    DeError,
}

pub type PuzzleFileResult<T> = Result<T, PuzzleFileError>;
