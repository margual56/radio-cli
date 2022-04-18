extern crate xdg;

use serde::{Deserialize};
use std::fs::File; 
use std::io::Write;
use std::io::Read;
use inquire::{error::InquireError, Select};
use colored::*;

pub const CONFIG_URL: &str = "https://raw.githubusercontent.com/margual56/radio-cli/main/config.json";

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
	pub fn load() -> Config {
		// Load config.json from $XDG_CONFIG_HOME/radio-cli
		let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
		let mut config_file = Config::load_config(xdg_dirs);

		// Read and parse the config into the `cfg` variable
		let mut config: String = String::new();
		config_file.read_to_string(&mut config).expect("Couldn't read config");

		Config {
			data: serde_json::from_str::<Config>(&config).expect("Couldn't parse config").data,
		}
	}

	fn load_config(dir: xdg::BaseDirectories) -> std::fs::File {
		// Check if file exists 
		// If it exists, return it
		// Otherwise, try to download it
		// It it cant, raise an error with an URL to the config file
		match dir.find_config_file("config.json") {
			None => {
				let msg = format!("The config file does not exist in \"{:?}\"", dir.get_config_dirs()[0].to_str());
				perror(msg.as_str());

				println!("\tDownloading file from {}...", CONFIG_URL.italic());
				let resp = reqwest::blocking::get(CONFIG_URL).expect("Request failed");
				let body = resp.text().expect("Body invalid");

				let file_ref = dir.place_config_file("config.json").expect("Could not create config file");

				println!("\tDone downloading!");

				println!("\tTrying to open {}", file_ref.to_str().expect("msg: &str").bold());

				let mut file = File::create(file_ref.clone()).unwrap();
				file.write_all(body.as_bytes()).expect("Could not write to config");

				drop(file);

				println!("\tFinished writing config. Enjoy! :)\n\n");

				File::open(file_ref).unwrap()
			},
			Some(x) => File::open(x).expect("Could not open config")
		}
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

pub fn perror(msg: &str) {
	println!("{} {}", "Error:".red().bold(), msg);
}