mod dump;
mod nyt;
mod scraper;

use dump::write_puzzle;
use nyt::NytScraper;
use scraper::Scraper;
use sudoku_config::Config;
use sudoku_log::setup_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger("scrape")?;

    let cfg = Config::load()?;

    log::info!("Scraper started");

    let scrapers /*: Vec<Box<dyn Scraper>> */ = vec![Box::new(NytScraper::default())];

    let mut manifest = cfg.manifest()?;

    for scraper in scrapers {
        log::info!("Running {} scraper", scraper.as_ref().name());

        match scraper.as_ref().scrape().await {
            Ok(iter) => {
                for puzzle in iter {
                    let id = puzzle.meta.id_include_unknown().unwrap_or_default();
                    let path = write_puzzle(&cfg, puzzle).await?;
                    manifest.add_puzzle(id, scraper.as_ref().source(), path);
                }
            }
            Err(e) => {
                log::error!("{} scraper failed: {}", scraper.as_ref().name(), e);
                continue;
            }
        }

        log::info!(
            "{} scraper completed successfully.",
            scraper.as_ref().name()
        );
    }

    cfg.update_manifest(manifest)?;

    log::info!("Successfully scraped puzzles");

    return Ok(());
}
