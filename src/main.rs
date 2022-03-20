extern crate xdg;
use structopt::StructOpt;
use std::process::Command;

use serde::{Deserialize};
use std::fs::File;
use std::io::Read;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    url: Option<String>,
    
    #[structopt(short, long)]
    station: Option<String>
}

#[derive(Deserialize, Debug)]
struct Station {
    station: String,
    url: String
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Vec<Station>
}

fn main() {
    // Parse the arguments
    let args = Cli::from_args();

    // Load config.json from $XDG_CONFIG_HOME/radio-cli
    let xdg_dirs = xdg::BaseDirectories::with_prefix("radio-cli").unwrap();
    let mut config_file = File::open(xdg_dirs.get_config_file("config.json")).expect("Couldn't open config");

    // Read and parse the config into the `cfg` variable
    let mut config: String = String::new();
    config_file.read_to_string(&mut config).expect("Couldn't read config");
    let cfg: Vec<Station> = (serde_json::from_str::<Data>(&config).expect("Couldn't parse config")).data;


    let mut station_given: bool;
    let mut station_name: String = String::new();

    match args.station {
        None => {station_given = false;}
        Some(x) => {
            station_given = true;
            station_name = x;
        }
    }

    if station_given {
        let mut url: String = String::new();
        let mut found: bool = false;

        for s in cfg.iter() {
            if s.station.eq(&station_name) {
                url = s.url.clone();
                found = true;
                break;
            }
        };

        if found {
            Command::new("mpv").arg(url).output().expect("Failed to run mpv");
        }
    }
}