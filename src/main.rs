use std::process::Command;
use radio_libs::{Config, perror};
pub use structopt::StructOpt;
use colored::*;

pub fn help() {
	println!(
r#"
{}
	Usage: radio [OPTIONS]

{}
	-u --url <URL>: Specifies an url to be played.
	-s --station <station name>: Specifies the name of the station to be played
	--no-video: A flag passed down to mpv, in case you want to listen to the audio of a youtube music video or something
	-v --verbose: Show extra information
	-h --help: Show this help and exit

{}
	The config file is personal and you should modify it with your own preferred stations.
	However, if you'd like to easily update from the one on GitHub, it is as easy as deleting the config.json file (see next section).
	The next time you launch the program, it will automatically download the file again.
	
	Feel free to add new stations to the GitHub config file!

{}
	The config file should be located in "$XDG_CONFIG_HOME/radio-cli/config.json". 
	If the file does not exist (e.g.: first time you run it), the program will {} of the repository.
	Inside this config file you can find all the stations and their URLs, feel free to add the ones you listen to,
	and it would be awesome if you added them to the main config file too! (https://github.com/margual56/radio-cli/blob/main/config.json)
"#, 
	"An interactive radio player that uses mpv".bold(),
	"OPTIONS: Used to play somethig directly".bold(),
	"UPDATE: Update the config file".bold(),
	"CONFIG: How to add new stations, edit and such".bold(),
	"automatically download the one from the main branch".bold()
	);
}


#[derive(StructOpt, Debug)]
pub struct Cli {
	/// Option: -u <URL>: Specifies an url to be played.
    #[structopt(short, long)]
    url: Option<String>,
    
	/// Option: -s <station name>: Specifies the name of the station to be played
    #[structopt(short, long, conflicts_with="url")]
    station: Option<String>,
	
    #[structopt(long="no-video")]
	no_video: bool,

	/// Show extra info
	#[structopt(short, long)]
	verbose: bool,

	/// Show the help and exit
	#[structopt(short, long)]
	help: bool,
	
}

fn main() {
    // Parse the arguments
    let args = Cli::from_args();

	// Just print the help and exit
	if args.help {
		help();
		std::process::exit(0);
	}

	// Parse the config file
	let config: Config = Config::load();
	let mut url: String = "".to_string();
	let url_given: bool;

	match args.url {
		None => {url_given = false;},
		Some(u) => {
			url_given = true;
			url = u;
		}
	}

	if !url_given{ 
		let station_given: bool;
		let mut station_name: String = String::new();

		match args.station {
			None => {station_given = false;}
			Some(x) => {
				station_given = true;
				station_name = x;
			}
		}

		if !station_given {
			let options = config.clone().get_all_stations();
			station_name = match config.clone().prompt(options) {
				Ok(s) => s,
				Err(_error) => {
					perror("Could not parse your choice");
					std::process::exit(1);
				}
			};
		}

		url = match config.get_url_for(&station_name) {
			Ok(u) => u,
			Err(()) => {
				perror("This station is not configured :(");
				std::process::exit(1);
			}
		};

		println!("Playing {}", station_name.green());
	}else{
		println!("Playing url '{}'", url.blue());
	}
	
	let mut mpv = Command::new("mpv");
	let mpv_args;

	if args.no_video {
		mpv_args = [url, "--no-video".to_string()];
	} else {
		mpv_args = [url, "".to_string()];
	}

	if args.verbose {
		let mut process = mpv.args(mpv_args).spawn().expect("failed to execute mpv");
		let _ecode = process.wait().expect("Failed to wait on mpv to finish");
	}else{
		let _process = mpv.args(mpv_args).output().expect("failed to execute mpv");
	}
}