extern crate xdg;

use crate::errors::{ConfigError, ConfigErrorCode};
use crate::perror;
use crate::station::Station;
use crate::version::Version;

use colored::*;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const _CONFIG_URL: &str = "https://raw.githubusercontent.com/margual56/radio-cli/main/config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(
        deserialize_with = "deserialize_version",
        serialize_with = "serialize_version"
    )]
    pub config_version: Version,
    pub max_lines: Option<usize>,

    #[serde(alias = "country")]
    pub country_code: Option<String>,

    #[serde(skip)]
    pub config_path: Option<PathBuf>,

    pub data: Vec<Station>,
}

impl Config {
    pub fn load_default() -> Result<Config, ConfigError> {
        // Load config.json from $XDG_CONFIG_HOME/radio-cli
        let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
        let config_file = Config::get_config_path(xdg_dirs);

        let mut config = Config::load(config_file.clone())?;
        config.config_path = Some(config_file);
        Ok(config)
    }

    pub fn load_from_file(path: PathBuf) -> Result<Config, ConfigError> {
        let mut config = Config::load(path.clone())?;
        config.config_path = Some(path);
        Ok(config)
    }

    fn load(file: PathBuf) -> Result<Config, ConfigError> {
        let mut config_file = match File::open(&file) {
            Ok(x) => x,
            Err(error) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::OpenError,
                    message: format!("Could not open the file {:?}", file),
                    extra: format!("{:?}", error),
                });
            }
        };

        // Read and parse the config into the `cfg` variable
        let mut config: String = String::new();
        match config_file.read_to_string(&mut config) {
            Ok(_) => {}
            Err(error) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::ReadError,
                    message: format!("Couldn't read the file {:?}", file),
                    extra: format!("{:?}", error),
                });
            }
        }

        let data: Config = match serde_json::from_str::<Config>(&config) {
            Ok(x) => x,
            Err(error) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::ParseError,
                    message: "Couldn't parse config".to_string(),
                    extra: format!("{:?}", error),
                });
            }
        };

        Ok(data)
    }

    /// Get the XDG path for the config file
    /// Creates the file if it doesn't exist
    ///
    /// Returns the path to the config file
    fn get_config_path(dir: xdg::BaseDirectories) -> PathBuf {
        match dir.find_config_file("config.json") {
            None => {
                // Get the name of the directory
                let tmp = dir.get_config_file("");
                let dir_name: &str = match tmp.to_str() {
                    Some(x) => x,
                    None => "??",
                };

                // Print an error message
                let msg = format!("The config file does not exist in \"{}\"", dir_name);
                perror(msg.as_str());

                // Download the file
                println!("\tLoading file from {}...", _CONFIG_URL.italic());
                let resp = reqwest::blocking::get(_CONFIG_URL).expect("Request failed");
                let body = resp.text().expect("Body invalid");

                // Create the new config file
                let file_ref = dir
                    .place_config_file("config.json")
                    .expect("Could not create config file");

                println!("\tDone loading!");

                println!(
                    "\tTrying to open {} to write the config...",
                    file_ref.to_str().expect("msg: &str").bold()
                );

                let mut file = File::create(file_ref.clone()).unwrap(); // This is write-only!!
                file.write_all(body.as_bytes())
                    .expect("Could not write to config");

                drop(file); // So we close the file to be able to read it

                println!("\tFinished writing config. Enjoy! :)\n\n");

                file_ref
            }
            Some(x) => x,
        }
    }

    pub fn save(&self) {
        let path = match &self.config_path {
            Some(p) => p,
            None => {
                let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
                &Config::get_config_path(xdg_dirs)
            }
        };
        let mut file = File::create(path).unwrap(); // This is write-only!!
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .expect("Could not write to config");

        drop(file); // So we close the file to be able to read it
    }

    pub fn get_url_for(&self, station_name: &str) -> Option<String> {
        for s in self.data.iter() {
            if s.0.name.eq(station_name) {
                return Some(s.0.url.clone());
            }
        }

        None
    }

    pub fn get_all_stations(self) -> Vec<String> {
        let mut stations: Vec<String> = Vec::new();

        for s in self.data.iter() {
            stations.push(s.0.name.clone());
        }

        stations
    }
}

fn serialize_version<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&version.to_string())
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    // Should be in form "major.minor.patch"
    let version_str = String::deserialize(deserializer)?;
    match Version::from(String::from(version_str)) {
        Some(version) => Ok(version),
        None => Err(serde::de::Error::custom("Error parsing version")),
    }
}
