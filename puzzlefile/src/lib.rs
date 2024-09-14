mod error;
mod file;
mod file_builder;
mod marks;
mod meta;

pub use error::{PuzzleFileError, PuzzleFileResult};
pub use file::{PuzzleBoard, PuzzleFile, PuzzleMarks};
pub use file_builder::PuzzleFileBuilder;
pub use marks::Marks;
pub use meta::{PuzzleDifficulty, PuzzleId, PuzzleMeta, PuzzleSource};
pub const PZZL_HEADER: [u8; 2] = [0x92, 0x21];
