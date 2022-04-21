use std::process::{Command, Stdio};
use std::io::{Write};
use radio_libs::{Config, ConfigError, Station, Version, perror};
pub use structopt::StructOpt;
use std::path::PathBuf;
use colored::*;

pub fn help() {
	println!(
r#"
{}
	Usage: radio [OPTIONS]

	Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program

{}
	-u --url <URL>: Specifies an url to be played.
	-s --station <station name>: Specifies the name of the station to be played
	-c --config <config file>: Specify a different config file from the default one
	--show-video: If *not* present, a flag is passed down to mpv to not show the video and just play the audio.
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
	
    #[structopt(long="show-video")]
	show_video: bool,

    #[structopt(long, short, parse(from_os_str))]
	config: Option<PathBuf>,

	/// Show extra info
	#[structopt(short, long)]
	verbose: bool,

	/// Show debug info
	#[structopt(short, long)]
	debug: bool,

	/// Show the help and exit
	#[structopt(short, long)]
	help: bool,
	
}

fn main() {
	let version = match Version::from(String::from(env!("CARGO_PKG_VERSION"))) {
		Some(v) => v,
		None => {
			perror("There was an error parsing the program version");
			std::process::exit(1);
		}
	};

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
			if args.debug {
				perror(format!("{:?}", error).as_str());
			} else {
				perror(format!("{}", error).as_str());
			}
			
			std::process::exit(1);
		}
	};

	if args.debug {
		println!("{} {}", "Program version:".bright_black().bold().italic(), 
			format!("{}", version).bright_black().italic());

		println!("{} {}", "Config version:".bright_black().bold().italic(), 
			format!("{}", config.config_version).bright_black().italic());
	}

	if config.config_version.major < version.major {
		println!("\n{} {}\n", "Warning!".yellow().bold(), 
		"The config version does not match the program version.\nThis might lead to parsing errors.".italic())
	}

	let station = match args.url {
		None => {
			let station: Station = match args.station {
				// If the station name is passed as an argument:
				Some(x) => {
					let url = match config.get_url_for(&x) {
						Some(u) => u,
						None => {
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
						Err(error) => {
							println!("\n\t{}", "Bye!".bold().green());

							if args.verbose {
								println!("({:?})", error);
							}

							std::process::exit(0);
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
	
	println!("{}", "Info: press 'q' to exit".italic().bright_black());

	let mut mpv = Command::new("mpv");
	let mut mpv_args: Vec<String> = [station.url].to_vec();

	if !args.show_video {
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