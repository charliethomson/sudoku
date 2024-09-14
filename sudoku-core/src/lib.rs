mod marks;
pub use marks::*;
mod math;
pub use math::*;
use sudoku_config::Manifest;

use std::fmt::Display;

use puzzlefile::{PuzzleBoard, PuzzleFile, PuzzleMarks, PuzzleMeta};

pub type CellIndex = usize;
pub type RegionIndex = usize;

pub enum BoardValidationError {
    Missing {
        index: usize,
    },
    Invalid {
        index: usize,
        expected: u8,
        present: u8,
    },
}

#[derive(Clone, Debug)]
pub struct BoardState {
    pub initial: PuzzleBoard,
    pub solved: PuzzleBoard,
    pub current: PuzzleBoard,
    pub center_marks: PuzzleMarks,
    pub pencil_marks: PuzzleMarks,
    pub meta: PuzzleMeta,
}
impl BoardState {
    pub fn is_solved(&self) -> bool {
        self.get_errors().is_none()
    }

    pub fn get_errors(&self) -> Option<Vec<BoardValidationError>> {
        let mut errors = Vec::with_capacity(81);

        for i in 0..81 {
            let expected = self.solved[i];
            let present = self.current[i];

            if present == 0 {
                errors.push(BoardValidationError::Missing { index: i });
            } else if present != expected {
                errors.push(BoardValidationError::Invalid {
                    index: i,
                    expected,
                    present,
                })
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    pub fn cell_raw(&self, idx: usize) -> u8 {
        self.current[idx]
    }
}
impl From<PuzzleFile> for BoardState {
    fn from(value: PuzzleFile) -> Self {
        Self {
            initial: value.initial_state.clone(),
            solved: value.solved_state.clone(),
            current: value.initial_state.clone(),
            center_marks: value.center_marks.clone(),
            pencil_marks: value.pencil_marks.clone(),
            meta: value.meta.clone(),
        }
    }
}
impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const INTER_BOARD_PADDING: usize = 4;
        const INTER_REGION_PADDING: usize = 2;
        const INTER_CELL_PADDING: usize = 1;
        // 3 regions of 3 cells
        // 1 1 1  2 2 2  3 3 3
        const BOARD_WIDTH: usize =
        9 // cells
        + (2 * INTER_CELL_PADDING * 3) // cell padding per region
        + (2 * INTER_REGION_PADDING) // region padding
        ;

        // header
        write!(f, "{}", center("initial", BOARD_WIDTH))?;
        write!(f, "{}", " ".repeat(INTER_BOARD_PADDING))?;
        write!(f, "{}", center("solved", BOARD_WIDTH))?;
        write!(f, "{}", " ".repeat(INTER_BOARD_PADDING))?;
        write!(f, "{}", center("current", BOARD_WIDTH))?;

        write!(f, "{}", "\n".repeat(INTER_REGION_PADDING))?;

        for row in 0..9 {
            for board_index in 0..3 {
                let board = match board_index {
                    0 => self.initial.to_vec(),
                    1 => self.solved.to_vec(),
                    2 => self.current.to_vec(),
                    _ => unreachable!(),
                };
                for col in 0..9 {
                    let idx = col + (9 * row);
                    let cell = board[idx];

                    write!(f, "{}", cell)?;
                    if ((col + 1) % 3) == 0 {
                        write!(f, "{}", " ".repeat(INTER_REGION_PADDING))?;
                    } else {
                        write!(f, "{}", " ".repeat(INTER_CELL_PADDING))?;
                    }
                }
                write!(f, "{}", " ".repeat(INTER_BOARD_PADDING))?;
            }
            if ((row + 1) % 3) == 0 {
                write!(f, "{}", "\n".repeat(INTER_REGION_PADDING))?;
            } else {
                write!(f, "{}", "\n".repeat(INTER_CELL_PADDING))?;
            }
        }

        Ok(())
    }
}

pub fn load_from_manifest(manifest: &Manifest) -> Vec<BoardState> {
    let mut puzzles = Vec::new();

    match manifest {
        Manifest::Version1 {
            puzzles: p,
            unknown_puzzles,
            ..
        } => {
            for (source, p) in p {
                for (id, path) in p {
                    log::info!("Loading puzzle id {}", id);

                    let mut file = match std::fs::OpenOptions::new().read(true).open(path) {
                        Ok(p) => p,
                        Err(e) => {
                            log::error!("Failed to open puzzle file: {e}");
                            continue;
                        }
                    };
                    let puzzle = match PuzzleFile::load(&mut file) {
                        Ok(p) => p,
                        Err(e) => {
                            log::error!("Malformed puzzle file: {e}");
                            continue;
                        }
                    };

                    let state = BoardState::from(puzzle);
                    puzzles.push(state);
                }
            }

            for (_source, p) in unknown_puzzles {
                for path in p {
                    log::info!("Loading unknown puzzle from {:?}", path);

                    let mut file = match std::fs::OpenOptions::new().read(true).open(path) {
                        Ok(p) => p,
                        Err(e) => {
                            log::error!("Failed to open puzzle file: {e}");
                            continue;
                        }
                    };
                    let puzzle = match PuzzleFile::load(&mut file) {
                        Ok(p) => p,
                        Err(e) => {
                            log::error!("Malformed puzzle file: {e}");
                            continue;
                        }
                    };

                    let state = BoardState::from(puzzle);
                    puzzles.push(state);
                }
            }
        }
    }

    return puzzles;
}

fn center(s: &str, i: usize) -> String {
    let midpoint = i / 2;
    let odd_target = s.len() % 2;
    let half = s.len() / 2;
    let left = midpoint - half - odd_target;
    let right = i - (midpoint + half);
    return format!("{}{}{}", " ".repeat(left), s, " ".repeat(right));
}

#[test]
fn test_center() {
    let s1 = "abc";
    assert_eq!(center(s1, 10).len(), 10);
    assert_eq!(center(s1, 11).len(), 11);
    assert_eq!(center(s1, 9).len(), 9);
    let s2 = "abcd";
    assert_eq!(center(s2, 10).len(), 10);
    assert_eq!(center(s2, 11).len(), 11);
    assert_eq!(center(s2, 9).len(), 9);
}
