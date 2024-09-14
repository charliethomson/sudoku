use iced::{
    advanced::{layout, renderer, Widget},
    widget::{column, row, text},
    Border, Element, Padding, Size,
};
use puzzlefile::Marks;

use crate::HostMessage;

pub enum CellMessage {}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Value(u8),
    PencilMarks(Marks),
    CenterMarks(Marks),
}
impl Cell {
    pub fn view<'a>(self) -> Element<'a, HostMessage> {
        let ui = match self {
            Self::Value(value) => column!(row!(text(value.to_string()).size(32))),
            Self::CenterMarks(marks) => column!(row!(text(
                marks
                    .marks()
                    .into_iter()
                    .map(|mark| mark.to_string())
                    .collect::<String>()
            ))),
            _ => unimplemented!(),
        };

        ui.width(48.).height(48.).into()
    }
}
pub fn cell_value(value: u8) -> Cell {
    Cell::Value(value)
}
pub fn cell_pencilmarks(marks: Marks) -> Cell {
    Cell::PencilMarks(marks)
}
pub fn cell_centermarks(marks: Marks) -> Cell {
    Cell::CenterMarks(marks)
}
