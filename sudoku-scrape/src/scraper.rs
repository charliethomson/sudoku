use puzzlefile::{PuzzleFile, PuzzleSource};

pub trait Scraper: Default + Copy {
    fn name(self) -> &'static str;
    fn source(self) -> PuzzleSource;
    async fn scrape(self) -> anyhow::Result<impl Iterator<Item = PuzzleFile>>;
}
