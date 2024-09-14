use anyhow::bail;
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use puzzlefile::{
    PuzzleDifficulty, PuzzleFile, PuzzleFileBuilder, PuzzleId, PuzzleMeta, PuzzleSource,
};
use serde::{Deserialize, Serialize};

use crate::scraper::Scraper;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    display_date: String,
    easy: PuzzleInformation,
    hard: PuzzleInformation,
    medium: PuzzleInformation,
}
impl GameData {
    pub async fn fetch() -> anyhow::Result<Self> {
        let body = reqwest::get("https://www.nytimes.com/puzzles/sudoku/hard")
            .await?
            .text()
            .await?;

        let Some(trimmed_start) = body.split("window.gameData = ").nth(1) else {
            bail!("Unable to find game data!");
        };

        let mut depth = 0;
        let mut end_index = 0;

        for c in trimmed_start.chars() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }

            end_index += 1;
            if depth == 0 {
                break;
            };
        }

        let json = &trimmed_start[..end_index];

        let this: Self = serde_json::from_str(json)?;

        this.easy.puzzle_data.validate()?;
        this.medium.puzzle_data.validate()?;
        this.hard.puzzle_data.validate()?;

        Ok(this)
    }

    fn iter(self) -> GameDataIter {
        GameDataIter::new(self)
    }
}

struct GameDataIter {
    data: GameData,
    i: usize,
}
impl GameDataIter {
    fn new(data: GameData) -> Self {
        Self { data, i: 0 }
    }
}

impl Iterator for GameDataIter {
    type Item = PuzzleFile;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.i {
            0 => Some(self.data.easy.to_puzzlefile(PuzzleDifficulty::Easy)),
            1 => Some(self.data.medium.to_puzzlefile(PuzzleDifficulty::Medium)),
            2 => Some(self.data.hard.to_puzzlefile(PuzzleDifficulty::Hard)),
            _ => None,
        };

        self.i += 1;

        return item;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PuzzleInformation {
    day_of_week: String,
    difficulty: String,
    print_date: String,
    published: String,
    puzzle_id: i64,
    version: i64,
    puzzle_data: PuzzleData,
}
impl PuzzleInformation {
    fn to_puzzlefile(&self, difficulty: PuzzleDifficulty) -> PuzzleFile {
        let publish_date = NaiveDate::parse_from_str(&self.print_date, "%Y-%m-%d")
            .map(|d| Utc.from_utc_datetime(&d.and_time(NaiveTime::MIN)))
            .unwrap_or(Utc::now());
        PuzzleFileBuilder::new()
            .meta(PuzzleMeta::Version1 {
                source: PuzzleSource::Nyt,
                id: PuzzleId::Id(self.puzzle_id),
                difficulty,
                puzzle_version: self.version.to_string(),
                publish_date: publish_date.clone().to_rfc3339(),
                load_time: Utc::now().to_rfc3339(),
                name: format!("NYT {} - {:?}", publish_date.format("%A, %e %B, %Y"), difficulty),
                slug: format!("nyt_{}", self.puzzle_id)
            })
            .initial_state_iter(self.puzzle_data.puzzle.iter().copied())
            .solved_state_iter(self.puzzle_data.solution.iter().copied())
            .build()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PuzzleData {
    hints: Vec<u8>,
    puzzle: Vec<u8>,
    solution: Vec<u8>,
}
impl PuzzleData {
    fn validate(&self) -> anyhow::Result<()> {
        if self.puzzle.len() != 81 {
            bail!(
                "Invalid puzzle! puzzle length {}, expected 81",
                self.puzzle.len()
            )
        }
        if self.solution.len() != 81 {
            bail!(
                "Invalid puzzle! solution length {}, expected 81",
                self.puzzle.len()
            )
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
pub struct NytScraper;
impl Scraper for NytScraper {
    fn name(self) -> &'static str {
        "NYT"
    }

    async fn scrape(self) -> anyhow::Result<impl Iterator<Item = PuzzleFile>> {
        let gamedata = GameData::fetch().await?;

        Ok(gamedata.iter())
    }

    fn source(self) -> PuzzleSource {
        PuzzleSource::Nyt
    }
}
