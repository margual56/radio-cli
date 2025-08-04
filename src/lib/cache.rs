use crate::browser::StationCache;
use crate::get_project_dirs;
use crate::version::Version;

use radiobrowser::ApiStation;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error as IOError, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Cache {
    pub stations: Vec<ApiStation>,
    pub version: Version,
    cache_folder: PathBuf,
}

impl Cache {
    pub fn new(stations: Vec<ApiStation>, version: Version) -> Self {
        let xdg_dirs = get_project_dirs();
        let cache_folder = xdg_dirs.cache_dir().to_path_buf();

        Cache {
            stations,
            version,
            cache_folder,
        }
    }
}

impl Cache {
    pub fn load() -> Self {
        let xdg_dirs = get_project_dirs();
        let cache_folder = xdg_dirs.cache_dir().to_path_buf();
        match File::open(&cache_folder.join("data.cache")) {
            Err(_) => {
                return Self {
                    stations: Vec::new(),
                    version: Version::from(String::from(env!("CARGO_PKG_VERSION")))
                        .unwrap_or_default(),
                    cache_folder,
                };
            }
            Ok(mut file) => {
                let mut contents = String::new();

                if let Err(_) = file.read_to_string(&mut contents) {
                    return Self::default();
                }

                match serde_json::from_str(&contents) {
                    Ok(cache) => cache,
                    Err(_) => Self::default(),
                }
            }
        }
    }

    pub fn insert(&mut self, stations: Vec<ApiStation>) {
        self.stations = stations.clone();
    }

    pub fn insert_rc(&mut self, stations: StationCache) {
        self.stations = Vec::from(stations.as_slice());
    }

    pub fn get_cache(&self) -> Option<StationCache> {
        if self.stations.is_empty() {
            None
        } else {
            Some(Rc::from(self.stations.clone()))
        }
    }

    pub fn save(&self) -> Result<(), IOError> {
        std::fs::create_dir_all(&self.cache_folder)?;

        let mut file = File::create(&self.cache_folder.join("data.cache"))?;

        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
