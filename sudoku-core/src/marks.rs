use puzzlefile::{Marks, PuzzleMarks};

use crate::CellIndex;

pub struct MarksState {
    marks: PuzzleMarks,
}
impl MarksState {
    pub fn get_marks(&self, cell: CellIndex) -> Option<Marks> {
        let m = self.marks[cell];
        if m.unset() {
            None
        } else {
            Some(m)
        }
    }
}
