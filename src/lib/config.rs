use crate::errors::{ConfigError, ConfigErrorCode};
use crate::station::Station;
use crate::version::Version;
use directories::ProjectDirs;

use colored::*;
use serde::de::{Deserializer, Error as SeError, Visitor};
use serde::Deserialize;
use std::fmt::{Formatter, Result as ResultFmt};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = include_str!("../../config.json");
const PROJECT_QUALIFIER: &str = "org";
const PROJECT_ORGANIZATION: &str = "margual56";
const PROJECT_NAME: &str = "radio-cli";


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_version")]
    pub config_version: Version,
    pub max_lines: Option<usize>,

    #[serde(alias = "country")]
    pub country_code: Option<String>,

    pub data: Vec<Station>,
}

impl Config {
    pub fn load_default() -> Result<Config, ConfigError> {
        Config::load(Config::get_config_file())
    }

    pub fn load_from_file(path: PathBuf) -> Result<Config, ConfigError> {
        Config::load(path)
    }

    fn load(file: PathBuf) -> Result<Config, ConfigError> {
        let mut config_file = match File::open(&file) {
            Ok(x) => x,
            Err(error) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::OpenError,
                    message: format!("Could not open the file {:?}", file),
                    extra: format!("{:?}", error),
                })
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
                })
            }
        }

        let data: Config = match serde_json::from_str::<Config>(&config) {
            Ok(mut x) => {
                x.data.push(Station {
                    station: "Other".to_string(),
                    url: "".to_string(),
                });

                x
            }
            Err(error) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::ParseError,
                    message: "Couldn't parse config".to_string(),
                    extra: format!("{:?}", error),
                })
            }
        };

        Ok(data)
    }

    fn get_config_file() -> PathBuf {
        let binding = ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_NAME).expect("Error finding Home directory");
        let dir = binding.config_local_dir();
        
        if !dir.exists() || !dir.join("config.json").exists() {
                println!("The config does not exist, writing default...");
                std::fs::create_dir_all(dir).expect("Could not create config folders");

                // Create the new config file
                let mut file = File::create(dir.join("config.json")).expect("Could not create config file");
                file.write_all(DEFAULT_CONFIG.as_bytes())
                    .expect("Could not write to config");
                file.flush().expect("Error while writing to the config file");

                drop(file); // So we close the file to be able to read it

                println!("\tFinished writing config.");
                println!("\tYou can find the config at: {}", format!("{:#?}", dir.as_os_str()).bold().yellow());
                println!("\tIn it you can add your favourite stations for easy access, and other settings too such as the country to filter the stations");
                println!("{}", "\tEnjoy! :)".bold().bright_green());
            }
        
        return dir.join("config.json");
    }

    pub fn get_url_for(&self, station_name: &str) -> Option<String> {
        for s in self.data.iter() {
            if s.station.eq(station_name) {
                return Some(s.url.clone());
            }
        }

        None
    }

    pub fn get_all_stations(self) -> Vec<String> {
        let mut stations: Vec<String> = Vec::new();

        for s in self.data.iter() {
            stations.push(s.station.clone());
        }

        stations
    }
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    struct JsonStringVisitor;

    impl<'de> Visitor<'de> for JsonStringVisitor {
        type Value = Version;

        fn expecting(&self, formatter: &mut Formatter) -> ResultFmt {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: SeError,
        {
            // unfortunately we lose some typed information
            // from errors deserializing the json string
            match Version::from(String::from(v)) {
                Some(x) => Ok(x),
                None => Err(SeError::custom("Could not parse version")),
            }
        }
    }

    // use our visitor to deserialize an `ActualValue`
    deserializer.deserialize_any(JsonStringVisitor)
}
