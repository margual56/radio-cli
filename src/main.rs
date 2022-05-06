pub use clap::Parser;
use colored::*;
use radio_libs::{perror, Config, ConfigError, Station, Version};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    /// Option: -u <URL>: Specifies an url to be played.
    #[clap(short, long, help = "Specifies an url to be played.")]
    url: Option<String>,

    /// Option: -s <station name>: Specifies the name of the station to be played
    #[clap(
        short,
        long,
        conflicts_with = "url",
        help = "Specifies the name of the station to be played."
    )]
    station: Option<String>,

    #[clap(
        long = "show-video",
        help = "If *not* present, a flag is passed down to mpv to not show the video and just play the audio."
    )]
    show_video: bool,

    #[clap(
        long,
        short,
        parse(from_os_str),
        help = "Specify a different config file from the default one."
    )]
    config: Option<PathBuf>,

    /// Show extra info
    #[structopt(short, long, help = "Show extra information.")]
    verbose: bool,

    /// Show debug info
    #[structopt(short, long)]
    debug: bool,
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
    let args = Cli::parse();

    // Parse the config file
    let config_result: Result<Config, ConfigError> = match args.config {
        None => Config::load_default(),
        Some(x) => Config::load_from_file(x),
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
        println!(
            "{} {}",
            "Program version:".bright_black().bold().italic(),
            format!("{}", version).bright_black().italic()
        );

        println!(
            "{} {}",
            "Config version:".bright_black().bold().italic(),
            format!("{}", config.config_version).bright_black().italic()
        );
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

                    Station { station: x, url }
                }

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
        }

        Some(x) => {
            println!("Playing url '{}'", x.blue());

            Station {
                station: String::from("URL"),
                url: x,
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
            println!(
                "{}: {}",
                "Hint".italic().bold(),
                "Try running radio-cli with the verbose flag (-v or --verbose)".italic()
            );
        }

        std::process::exit(2);
    }
}
