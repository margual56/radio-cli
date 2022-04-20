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

impl std::fmt::Display for Station {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.station)
    }
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
			data: serde_json::from_str(&config).expect("Couldn't parse config"),
		}
	}

	fn load_config(dir: xdg::BaseDirectories) -> std::fs::File {
		match dir.find_config_file("config.json") {
			None => {
				// Get the name of the directory
				let tmp = dir.get_config_file("");
				let dir_name: &str = match tmp.to_str() {
					Some(x) => x,
					None => "??"
				};

				// Print an error message
				let msg = format!("The config file does not exist in \"{}\"", dir_name);
				perror(msg.as_str());

				// Download the file
				println!("\tLoading file from {}...", CONFIG_URL.italic());
				let resp = reqwest::blocking::get(CONFIG_URL).expect("Request failed");
				let body = resp.text().expect("Body invalid");

				// Create the new config file
				let file_ref = dir.place_config_file("config.json").expect("Could not create config file");

				println!("\tDone loading!");

				println!("\tTrying to open {} to write the config...", file_ref.to_str().expect("msg: &str").bold());

				let mut file = File::create(file_ref.clone()).unwrap();	// This is write-only!!
				file.write_all(body.as_bytes()).expect("Could not write to config");

				drop(file); // So we close the file to be able to read it

				println!("\tFinished writing config. Enjoy! :)\n\n");

				File::open(file_ref).unwrap()  // This is read-only
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

	pub fn prompt(self) -> Result<Station, InquireError> {
		Select::new(&"Select a station to play:".bold(), self.data).prompt()
	}
}

pub fn perror(msg: &str) {
	println!("{} {}", "Error:".red().bold(), msg);
}