use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum PuzzleDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum PuzzleSource {
    Nyt,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "id_type", content = "id")]
pub enum PuzzleId {
    Id(i64),
    Unknown,
}
impl Default for PuzzleId {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum PuzzleMeta {
    Nil,
    #[serde(rename = "v1")]
    Version1 {
        source: PuzzleSource,
        publish_date: String,
        load_time: String,
        id: PuzzleId,
        difficulty: PuzzleDifficulty,
        puzzle_version: String,
        name: String,
        slug: String,
    },
}
impl PuzzleMeta {
    pub fn id(&self) -> Option<i64> {
        match self {
            Self::Nil => None,
            Self::Version1 { id, .. } => match *id {
                PuzzleId::Id(id) => Some(id),
                PuzzleId::Unknown => None,
            },
        }
    }
    pub fn id_include_unknown(&self) -> Option<PuzzleId> {
        match self {
            Self::Nil => None,
            Self::Version1 { id, .. } => Some(*id),
        }
    }

    pub fn src(&self) -> Option<String> {
        match self {
            Self::Nil => None,
            Self::Version1 { source, .. } => match source {
                PuzzleSource::Nyt => Some("nyt".into()),
            },
        }
    }
}
