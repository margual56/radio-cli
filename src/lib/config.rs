extern crate xdg;

use crate::errors::{ConfigError, ConfigErrorCode};
use crate::perror;
use crate::station::Station;
use crate::version::Version;

use colored::*;
use serde::de::{Deserializer, Error as SeError, Visitor};
use serde::Deserialize;
use std::fmt::{Formatter, Result as ResultFmt};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const _CONFIG_URL: &str = "https://raw.githubusercontent.com/margual56/radio-cli/main/config.json";

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
        // Load config.json from $XDG_CONFIG_HOME/radio-cli
        let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
        let config_file = Config::load_config(xdg_dirs);

        Config::load(config_file)
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

    fn load_config(dir: xdg::BaseDirectories) -> PathBuf {
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
