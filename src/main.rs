use std::process::{Command, Stdio};
use std::io::{Write};
use radio_libs::{Config, ConfigError, Station, perror};
pub use structopt::StructOpt;
use std::path::PathBuf;
use colored::*;

pub fn help() {
	println!(
r#"
{}
	Usage: radio [OPTIONS]

{}
	-u --url <URL>: Specifies an url to be played.
	-s --station <station name>: Specifies the name of the station to be played
	-c --config <config file>: Specify a different config file from the default one
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

    #[structopt(long, short, parse(from_os_str))]
	config: Option<PathBuf>,

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
	let config_result: Result<Config, ConfigError> = match args.config {
		None => Config::load_default(),
		Some(x) => Config::load_from_file(x)
	};

	let config = match config_result {
		Ok(x) => x,
		Err(error) => {
			if args.verbose {
				perror(format!("{}: {}", error, error.extra).as_str());
			} else {
				perror(format!("{}", error).as_str());
			}
			
			std::process::exit(1);
		}
	};

	let station = match args.url {
		None => {
			let station: Station = match args.station {
				// If the station name is passed as an argument:
				Some(x) => {
					let url = match config.get_url_for(&x) {
						Ok(u) => u,
						Err(()) => {
							perror("This station is not configured :(");
							std::process::exit(1);
						}
					};

					Station {
						station: x,
						url: url
					}
				},

				// Otherwise
				None => {
					// And let the user choose one
					match config.clone().prompt() {
						Ok(s) => s,
						Err(_error) => {
							perror("Choice not valid");
							std::process::exit(1);
						}
					}
				}
			};

			

			println!("Playing {}", station.station.green());

			station
		},
		Some(x) => {
			println!("Playing url '{}'", x.blue());
			
			Station {
				station: String::from("URL"),
				url: x
			}
		}
	};
	
	let mut mpv = Command::new("mpv");
	let mut mpv_args: Vec<String> = [station.url].to_vec();

	if args.no_video {
		mpv_args.push(String::from("--no-video"));
	}
	
	if !args.verbose {
		mpv_args.push(String::from("--really-quiet"));
	}

	let output = mpv
					.args(mpv_args)
					.stdin(Stdio::inherit())
					.stdout(Stdio::inherit())
					.output()
					.expect("Failed to execute command");
	
	std::io::stdout().write_all(&output.stdout).unwrap();
	std::io::stderr().write_all(&output.stderr).unwrap();

	if !output.status.success() {
		perror(format!("mpv {}", output.status).as_str());

		if !args.verbose {
			println!("{}: {}", 
				"Hint".italic().bold(), 
				"Try running radio-cli with the verbose flag (-v or --verbose)".italic());
		}

		std::process::exit(2);
	}
}