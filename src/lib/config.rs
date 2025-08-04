use crate::errors::{ConfigError, ConfigErrorCode};
use crate::get_project_dirs;
use crate::station::Station;
use crate::version::Version;

use colored::*;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = include_str!("../../config.json");

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
        Config::load(Config::get_default_config_file())
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Config, ConfigError> {
        let mut config = Config::load(path.clone())?;
        config.config_path = Some(path.clone());
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

    fn get_default_config_file() -> PathBuf {
        let binding = get_project_dirs();
        let dir = binding.config_local_dir();

        if !dir.exists() || !dir.join("config.json").exists() {
            println!("The config does not exist, writing default...");
            std::fs::create_dir_all(dir).expect("Could not create config folders");

            // Create the new config file
            let mut file =
                File::create(dir.join("config.json")).expect("Could not create config file");
            file.write_all(DEFAULT_CONFIG.as_bytes())
                .expect("Could not write to config");
            file.flush()
                .expect("Error while writing to the config file");

            drop(file); // So we close the file to be able to read it

            println!("\tFinished writing config.");
            println!(
                "\tYou can find the config at: {}",
                format!("{:#?}", dir.as_os_str()).bold().yellow()
            );
            println!(
                "\tIn it you can add your favourite stations for easy access, and other settings too such as the country to filter the stations"
            );
            println!("{}", "\tEnjoy! :)".bold().bright_green());
        }

        let path = dir.join("config.json");
        path
    }

    fn get_config_file(&self) -> PathBuf {
        match &self.config_path {
            Some(p) => p.clone(),
            None => Config::get_default_config_file(),
        }
    }

    pub fn save(&self) {
        let path = self.get_config_file();
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
