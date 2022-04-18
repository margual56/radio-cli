extern crate xdg;

use serde::{Deserialize};
use std::fs::File;
use std::io::Read;
use inquire::{error::InquireError, Select};
use colored::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Station {
	pub station: String,
	pub url: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
	data: Vec<Station>
}

impl Config {
	pub fn load() -> Result<Config, std::io::Error> {
		// Load config.json from $XDG_CONFIG_HOME/radio-cli
		let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
		let mut config_file = match File::open(xdg_dirs.get_config_file("config.json")) {
			Err(error) => return Err(error),
			Ok(x) => x,
		};

		// Read and parse the config into the `cfg` variable
		let mut config: String = String::new();
		config_file.read_to_string(&mut config).expect("Couldn't read config");

		Ok(Config {
			data: serde_json::from_str::<Config>(&config).expect("Couldn't parse config").data,
		})
	}

	pub fn get_url_for(self, station_name: &str) -> Result<String, ()> {
		for s in self.data.iter() {
			if s.station.eq(station_name) {
				return Ok(s.url.clone());
			}
		}

		Err(())
	}

	pub fn get_all_stations(self) -> Vec<String> {
		let mut stations: Vec<String> = Vec::new();

		for s in self.data.iter() {
			stations.push(s.station.clone());
		}

		return stations;
	}

	pub fn prompt(self, options: Vec<String>) -> Result<String, InquireError> {
		let res: Result<&str, InquireError> = Select::new(&"Select a station to play:".bold(), options.iter().map(|s| s.as_ref()).collect()).prompt();
		
		match res {
			Ok(s) => Ok(s.to_string()),
			Err(error) => Err(error)
		}
	}
}