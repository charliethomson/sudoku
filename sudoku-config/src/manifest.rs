use std::{collections::HashMap, path::PathBuf};

use chrono::Utc;
use puzzlefile::{PuzzleFile, PuzzleId, PuzzleSource};
use serde::{Deserialize, Serialize};

use crate::{ConfigError, ConfigResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum Manifest {
    #[serde(rename = "v1")]
    Version1 {
        num_puzzles: usize,
        last_updated: String,

        sources: Vec<PuzzleSource>,

        puzzles: HashMap<PuzzleSource, HashMap<i64, PathBuf>>,
        unknown_puzzles: HashMap<PuzzleSource, Vec<PathBuf>>,
    },
}
impl Default for Manifest {
    fn default() -> Self {
        Self::Version1 {
            num_puzzles: 0,
            last_updated: Utc::now().to_rfc3339(),
            sources: vec![PuzzleSource::Nyt],
            puzzles: HashMap::new(),
            unknown_puzzles: HashMap::new(),
        }
    }
}

impl Manifest {
    pub fn puzzles(&self) -> Vec<PuzzleFile> {
        let paths = match self {
            Self::Version1 { puzzles, unknown_puzzles, .. } => {
                puzzles.values().flat_map(|map| map.values())
                .chain(unknown_puzzles.values().flat_map(|vec| vec.iter()))
                       }
        };

        return paths.filter_map(|path| {
            let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(path)  else {
                log::error!("No puzzle found at {:?}", path);
                return None;
            };

            match PuzzleFile::load(&mut file) {
                Ok(puzzle) => Some(puzzle),
                Err(e) => {
                    log::error!("Failed to load puzzle at {:?}: {}", path, e);
                    None
                }
            }
        }).collect();

    }
}
impl Manifest {
    pub fn needs_upgrade(&self) -> bool {
        !matches!(self, Manifest::Version1 { .. })
    }

    pub fn upgrade(self) -> Self {
        self
    }

    pub fn has_puzzle(&self, id: PuzzleId, source: PuzzleSource) -> ConfigResult<bool> {
        let PuzzleId::Id(id) = id else {
            return Ok(false);
        };

        match self {
            Self::Version1 {
                sources, puzzles, ..
            } => {
                if !sources.contains(&source) {
                    return Ok(false);
                }

                let Some(puzzles) = puzzles.get(&source) else {
                    return Ok(false);
                };

                return Ok(puzzles.iter().any(|(pid, _)| *pid == id));
            }
            _ => return Err(ConfigError::NotImplemented),
        }
    }

    pub fn add_puzzle(
        &mut self,
        id: PuzzleId,
        source: PuzzleSource,
        path: PathBuf,
    ) -> ConfigResult<()> {
        if self.has_puzzle(id, source)? {
            return Err(ConfigError::DuplicatePuzzle);
        }

        match self {
            Self::Version1 {
                sources,
                puzzles,
                unknown_puzzles,
                num_puzzles,
                ..
            } => {
                if !sources.contains(&source) {
                    sources.push(source);
                }

                match id {
                    PuzzleId::Id(id) => {
                        if let Some(puzzles) = puzzles.get_mut(&source) {
                            puzzles.insert(id, path);
                        } else {
                            let mut map = HashMap::new();
                            map.insert(id, path);
                            puzzles.insert(source, map);
                        }
                    }
                    PuzzleId::Unknown => {
                        if let Some(puzzles) = unknown_puzzles.get_mut(&source) {
                            puzzles.push(path);
                        } else {
                            unknown_puzzles.insert(source, vec![path]);
                        }
                    }
                }

                *num_puzzles += 1;
            }
            _ => return Err(ConfigError::NotImplemented),
        }

        Ok(())
    }

    pub(crate) fn update(&mut self) -> ConfigResult<()> {
        match self {
            Self::Version1 { last_updated, .. } => {
                *last_updated = Utc::now().to_rfc3339();
            }
            _ => return Err(ConfigError::NotImplemented),
        }

        Ok(())
    }
}
