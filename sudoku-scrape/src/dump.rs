use std::path::PathBuf;

use puzzlefile::PuzzleFile;
use sudoku_config::Config;

async fn next_unknown(cfg: &Config) -> anyhow::Result<String> {
    let mut max = 0;
    let mut reader = tokio::fs::read_dir(&cfg.puzzles_dir).await?;
    while let Some(entry) = reader.next_entry().await? {
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or_default();

        if !file_name.starts_with("unknown_") {
            continue;
        }

        let id = file_name
            .split("unknown_")
            .nth(1)
            .unwrap_or_default()
            .split(".")
            .next()
            .unwrap_or_default();

        if id.is_empty() {
            continue;
        }

        let Ok(id) = id.parse::<usize>() else {
            continue;
        };

        if id > max {
            max = id;
        }
    }

    return Ok(format!("unknown_{max}"));
}

pub async fn write_puzzle(cfg: &Config, puzzle: PuzzleFile) -> anyhow::Result<PathBuf> {
    let slug = match puzzle.meta.id() {
        Some(id) => match puzzle.meta.src() {
            Some(src) => format!("{}_{}", src, id),
            None => next_unknown(cfg).await?,
        },
        None => next_unknown(cfg).await?,
    };

    let path = cfg.puzzles_dir.clone().join(format!("{slug}.pzzl"));

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)?;

    puzzle.store(&mut file)?;

    log::info!("Wrote puzzle to {:?}", &path);

    Ok(path)
}
