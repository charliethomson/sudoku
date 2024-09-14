use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use crate::{Marks, PuzzleFileError, PuzzleFileResult, PuzzleMeta, PZZL_HEADER};

pub type PuzzleBoard = Vec<u8>;
pub type PuzzleMarks = Vec<Marks>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuzzleFile {
    pub header: [u8; 2],
    pub initial_state: PuzzleBoard,
    pub solved_state: PuzzleBoard,
    pub center_marks: PuzzleMarks,
    pub pencil_marks: PuzzleMarks,
    pub meta: PuzzleMeta,
}
impl PuzzleFile {
    fn validate_board(board: &[u8]) -> PuzzleFileResult<()> {
        for (i, cell) in board.iter().enumerate() {
            if *cell > 9u8 {
                return Err(PuzzleFileError::InvalidCell(i, *cell));
            }
        }

        Ok(())
    }

    pub fn load<R: Read>(reader: &mut R) -> PuzzleFileResult<Self> {
        let this: Self = match ciborium::from_reader(reader) {
            Ok(this) => this,
            Err(e) => {
                log::error!("Failed to deserialize: {e}");
                return Err(PuzzleFileError::DeError);
            }
        };

        if this.header != PZZL_HEADER {
            return Err(PuzzleFileError::InvalidHeader);
        }

        Self::validate_board(&this.initial_state)?;
        Self::validate_board(&this.solved_state)?;

        Ok(this)
    }

    pub fn store<W: Write>(&self, writer: &mut W) -> PuzzleFileResult<()> {
        if let Err(e) = ciborium::into_writer(self, writer) {
            log::error!("Failed to serialize: {e}");
            return Err(PuzzleFileError::SerError);
        }

        Ok(())
    }
}
