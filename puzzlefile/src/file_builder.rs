use crate::{Marks, PuzzleBoard, PuzzleFile, PuzzleMarks, PuzzleMeta, PZZL_HEADER};

pub struct PuzzleFileBuilder {
    header: [u8; 2],
    initial_state: PuzzleBoard,
    solved_state: PuzzleBoard,
    center_marks: PuzzleMarks,
    pencil_marks: PuzzleMarks,
    meta: PuzzleMeta,
}
impl Default for PuzzleFileBuilder {
    fn default() -> Self {
        Self {
            header: PZZL_HEADER,
            initial_state: vec![0; 81],
            solved_state: vec![0; 81],
            center_marks: vec![Marks::default(); 81],
            pencil_marks: vec![Marks::default(); 81],
            meta: PuzzleMeta::Nil,
        }
    }
}
impl PuzzleFileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial_state_iter<I: Iterator<Item = u8>>(mut self, iter: I) -> Self {
        self.initial_state = iter.take(81).collect();
        self
    }

    pub fn solved_state_iter<I: Iterator<Item = u8>>(mut self, iter: I) -> Self {
        self.solved_state = iter.take(81).collect();
        self
    }

    pub fn initial_state(mut self, v: Vec<u8>) -> Self {
        self.initial_state = v;
        self
    }

    pub fn solved_state(mut self, v: Vec<u8>) -> Self {
        self.solved_state = v;
        self
    }
    pub fn center_marks_iter<I: Iterator<Item = Marks>>(mut self, iter: I) -> Self {
        self.center_marks = iter.take(81).collect();
        self
    }

    pub fn pencil_marks_iter<I: Iterator<Item = Marks>>(mut self, iter: I) -> Self {
        self.pencil_marks = iter.take(81).collect();
        self
    }

    pub fn center_marks(mut self, marks: Vec<Marks>) -> Self {
        self.center_marks = marks;
        self
    }

    pub fn pencil_marks(mut self, marks: Vec<Marks>) -> Self {
        self.pencil_marks = marks;
        self
    }

    pub fn meta(mut self, meta: PuzzleMeta) -> Self {
        self.meta = meta;
        self
    }

    pub fn build(self) -> PuzzleFile {
        PuzzleFile {
            header: self.header,
            initial_state: self.initial_state,
            solved_state: self.solved_state,
            center_marks: self.center_marks,
            pencil_marks: self.pencil_marks,
            meta: self.meta,
        }
    }
}
