use crate::{CellIndex, RegionIndex};

pub fn region_start(region: RegionIndex) -> CellIndex {
    match region {
        0 => 0,
        1 => 3,
        2 => 6,
        3 => 27,
        4 => 30,
        5 => 33,
        6 => 54,
        7 => 57,
        8 => 60,
        _ => unreachable!(),
    }
}

pub const REGION_OFFSETS: [usize; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];
