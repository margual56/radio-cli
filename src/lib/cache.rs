extern crate xdg;

use crate::station::Station;
use crate::version::Version;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error as IOError, Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cache {
    pub stations: Vec<Station>,
    pub version: Version,
}

impl Cache {
    pub fn new(stations: Vec<Station>, version: Version) -> Self {
        Cache { stations, version }
    }
}

impl Cache {
    pub fn load() -> Self {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
        match xdg_dirs.find_cache_file("stations.cache") {
            None => {
                return Self {
                    stations: Vec::new(),
                    version: Version::
                };
            }
            Some(path) => {
                let mut config_file = match File::open(&file) {
                    Ok(x) => x,
                    Err(error) => {
            }
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), IOError> {
        let mut file = File::create(path)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
