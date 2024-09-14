// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use sudoku_config::{Config, ConfigError};
use sudoku_log::setup_logger;
use thiserror::Error;
use puzzlefile::PuzzleFile;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "error")]
pub enum AppError {
    #[error("Failed to get configuration: ({variant}): {message}")]
    ConfigError {
        variant: &'static str,
        message: String,
    },
}
impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        Self::ConfigError {
            variant: (&value).into(),
            message: value.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct AppResult<T> {
    ok: bool,
    data: Option<T>,
    error: Option<AppError>,
}
impl<T> AppResult<T> {
    fn ok(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }
    fn err(error: AppError) -> Self {
        Self {
            ok: false,
            error: Some(error),
            data: None,
        }
    }
}
impl<T, E> From<Result<T, E>> for AppResult<T>
where
    E: Into<AppError>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => Self::ok(data),
            Err(e) => Self::err(e.into()),
        }
    }
}

#[tauri::command]
fn get_puzzles() -> AppResult<Vec<PuzzleFile>> {
    let config = match Config::load() {
        Ok(conf) => conf,
        Err(e) => {return Err(e).into();}
    };

    let manifest = match config.manifest() {
        Ok(conf) => conf,
        Err(e) => {return Err(e).into();}
    };

    AppResult::ok(manifest.puzzles())
}

fn main() {
    setup_logger("tauri").expect("Failed to configure logger");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_puzzles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
