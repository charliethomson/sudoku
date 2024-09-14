use std::{
    io::{Read, Write},
    path::PathBuf,
};

mod error;
mod manifest;
use consts::PRODUCT_NAME;
pub use error::*;
pub use manifest::*;

use figment::{
    providers::{Env, Format, Json, Serialized, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

fn config_dir() -> ConfigResult<PathBuf> {
    let config_base = dirs::config_local_dir().ok_or(ConfigError::NoConfigDir)?;
    let dir = config_base.join(PRODUCT_NAME);
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub config_dir: PathBuf,
    pub puzzles_dir: PathBuf,
    pub manifest_path: PathBuf,
}
impl Default for Config {
    fn default() -> Self {
        let base_dir = config_dir().unwrap();
        let puzzles_dir = base_dir.clone().join("puzzles");
        let manifest_path = base_dir.clone().join("manifest.yml");

        Self {
            config_dir: base_dir,
            puzzles_dir,
            manifest_path,
        }
    }
}
impl Config {
    fn ensure_exists(&self) -> ConfigResult<()> {
        if !self.puzzles_dir.exists() {
            log::warn!("Creating missing puzzles directory: {:?}", self.puzzles_dir);
            std::fs::create_dir_all(&self.puzzles_dir)?;
        }

        if !self.manifest_path.exists() {
            log::warn!("Creating missing manifest: {:?}", self.manifest_path);
            let contents = serde_yaml::to_string(&Manifest::default())?;
            let mut file = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&self.manifest_path)?;

            file.write_all(contents.as_bytes())?;
        }

        Ok(())
    }

    pub fn load() -> ConfigResult<Self> {
        let base_dir = config_dir()?;

        let json_config = base_dir.join("cfg.json");
        let yml_config = base_dir.join("cfg.yml");

        if !yml_config.exists() {
            let file = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&yml_config)?;
            serde_yaml::to_writer(file, &Self::default())?;
        }

        log::info!("Loading configuration");
        let cfg: Self = Figment::from(Serialized::defaults(Self::default()))
            .merge(Yaml::file(yml_config))
            .merge(Json::file(json_config))
            .merge(Env::prefixed("SUDOKU_"))
            .extract()?;
        log::info!("Configuration loaded successfully: {:?}", cfg);

        cfg.ensure_exists()?;

        Ok(cfg)
    }

    pub fn manifest(&self) -> ConfigResult<Manifest> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(&self.manifest_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let manifest = serde_yaml::from_str(&contents)?;

        Ok(manifest)
    }

    pub fn update_manifest(&self, mut manifest: Manifest) -> ConfigResult<()> {
        log::info!("Updating manifest.");
        manifest.update()?;
        let contents = serde_yaml::to_string(&manifest)?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(&self.manifest_path)?;

        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
