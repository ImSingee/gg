use std::path::{Path, PathBuf};
use tokio::fs::read_to_string;
use crate::{Config, parse};
use crate::result::Result;
use gg_git::G;

/// load config from a file
pub async fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    parse(&read_to_string(path).await?)
}


pub struct LoadedConfig {
    pub path: PathBuf,
    pub config: Config,
}

/// trait for extending Option<LoadedConfig>
pub trait OptionalLoadedConfig {
    fn get(self) -> Config;
}

impl OptionalLoadedConfig for Option<LoadedConfig> {
    /// get config from LoadedConfig if exists, otherwise return default config
    fn get(self) -> Config {
        match self {
            Some(config) => config.config,
            None => Config::default(),
        }
    }
}


static CONFIG_FILENAMES: [&str; 3] = [".ggrc.json", ".gg.json", "gg.config.json"];

/// load config from a directory
/// If no config file is found, return Ok(None)
pub async fn auto_load<P: Into<PathBuf>>(dir: P) -> Result<Option<LoadedConfig>> {
    let dir = dir.into();

    for filename in CONFIG_FILENAMES.iter() {
        let path = dir.join(filename);
        let config = load(path.clone()).await;
        if config.is_ok() {
            return Ok(Some(LoadedConfig {
                path,
                config: config.unwrap(),
            }));
        } else {
            let err = config.err().unwrap();
            if !err.is_not_exist() {
                return Err(err);
            }
        }
    }

    // not found
    Ok(None)
}

/// load config from repo root
pub async fn auto_load_for_repo<P: Into<PathBuf>>(dir: P) -> Result<Option<LoadedConfig>> {
    let dir = dir.into();

    let root = G::new(&dir).root().await;
    if root.is_err() {
        // not a git repo or git is not available
        return auto_load(dir).await;
    }

    auto_load(root.unwrap()).await
}