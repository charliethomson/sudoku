use cell::{cell_centermarks, cell_value};
use iced::widget::{column, container, row, slider, text, Column, Row};
use iced::{Element, Padding, Sandbox, Settings, Size};
use log::{error, info};
use puzzlefile::Marks;
use std::time::SystemTime;
use sudoku_config::Config;
use sudoku_core::{load_from_manifest, region_start, BoardState, RegionIndex, REGION_OFFSETS};
use sudoku_log::setup_logger;

mod cell;

struct Host {
    pub board: BoardState,
}

#[derive(Debug, Clone, Copy)]
enum HostMessage {}

fn region(board: &BoardState, region_id: RegionIndex) -> Element<HostMessage> {
    let base_idx = region_start(region_id);
    let mut cells = REGION_OFFSETS
        .into_iter()
        .map(|offset| cell_value(board.cell_raw(base_idx + offset)));
    // .map(|offset| cell_centermarks(Marks::from_marks([2u8, 3, 4, 5].into_iter())));

    let p = Padding::new(8f32);

    let mut cols = Column::new();
    let mut row = Vec::new();

    for cell in cells {
        if row.len() == 3 {
            cols = cols.push(Row::with_children(row.drain(..)));
            row.clear();
        }

        row.push(cell.view());
    }

    if row.len() == 3 {
        cols = cols.push(Row::with_children(row.drain(..)));
        row.clear();
    }

    return cols.padding(p).into();
}

impl Sandbox for Host {
    fn new() -> Self {
        let config = Config::load().expect("Failed to load configuration");
        let manifest = config.manifest().expect("Failed to get manifest");

        let puzzles = load_from_manifest(&manifest);

        Self {
            board: puzzles.first().unwrap().clone(),
        }
    }

    fn update(&mut self, message: HostMessage) {
        match message {}
    }

    fn view(&self) -> Element<HostMessage> {
        let mut content = column!(
            row!(
                region(&self.board, 0),
                region(&self.board, 1),
                region(&self.board, 2)
            ),
            row!(
                region(&self.board, 3),
                region(&self.board, 4),
                region(&self.board, 5)
            ),
            row!(
                region(&self.board, 6),
                region(&self.board, 7),
                region(&self.board, 8)
            ),
        );

        content.align_items(iced::Alignment::Center).into()
    }

    type Message = HostMessage;

    fn title(&self) -> String {
        "Sudoku".into()
    }
}

fn main() {
    setup_logger("game").expect("Failed to configure the logger");

    info!("Starting application");

    let mut settings = Settings::default();
    settings.window.size = Size::new(1000f32, 1000f32);
    if let Err(e) = Host::run(settings) {
        error!("Exited with error: {e}")
    };

    info!("Exited normally.");
}
