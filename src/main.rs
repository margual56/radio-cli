pub use clap::Parser;
use colored::*;
use radio_libs::{browser, perror, Config, ConfigError, Station, Version};
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
    /// Option: -u --url <URL>: Specifies an url to be played.
    #[clap(short, long, help = "Specifies an url to be played.")]
    url: Option<String>,

    /// Option: -s --station <station name>: Specifies the name of the station to be played
    #[clap(
        short,
        long,
        conflicts_with = "url",
        help = "Specifies the name of the station to be played."
    )]
    station: Option<String>,

    /// Flag: --show-video: If *not* present, a flag is passed down to mpv to not show the video and just play the audio.
    #[clap(
        long = "show-video",
        help = "If *not* present, a flag is passed down to mpv to not show the video and just play the audio."
    )]
    show_video: bool,

    /// Option: -c --config: Specify a config file other than the default.
    #[clap(
        long,
        short,
        parse(from_os_str),
        help = "Specify a different config file from the default one."
    )]
    config: Option<PathBuf>,

    /// Option: --country-code <CODE>: Specify a country code to filter the search results
    #[clap(
        long = "country-code",
        help = "Specify a country code to filter the search."
    )]
    country_code: Option<String>,

    /// Flag: --list-countries: List all the available countries and country codes to put in the config.
    #[clap(
        long = "list-countries",
        help = "List all the available countries and country codes to put in the config."
    )]
    list_countries: bool,

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

    if args.list_countries {
        let result = browser::get_countries();

        if let Ok(countries) = result {
            for country in countries {
                println!("{}: \"{}\"", country.name, country.iso_3166_1.bold());
            }
        } else {
            println!("Could not connect to the server, please check your connection.");
        }

        std::process::exit(0);
    }

    // Parse the config file
    let config_result: Result<Config, ConfigError> = match args.config {
        None => Config::load_default(),
        Some(x) => Config::load_from_file(x),
    };

    let config = match config_result {
        Ok(mut x) => {
            if let Some(cc) = args.country_code {
                x.country_code = Some(cc);
            }

            x
        }
        Err(error) => {
            if args.debug {
                perror(format!("{:?}", error).as_str());
            } else {
                perror(format!("{}", error).as_str());
                print!("{}", "Try pasing the debug flag (-d). ".yellow());
            }

            println!(
                "{}",
                "Deleting your config will download the updated one."
                    .yellow()
                    .bold()
            );

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

    if let None = config.country_code {
        println!("\n{} {}", "Warning!".yellow().bold(), 
		"The config does not contain a valid country (for example, \"ES\" for Spain or \"US\" for the US).".italic());
        println!("{} {} {}\n", "You can use the option".italic(), "--list-countries".bold().italic(), "to see the available options".italic());
        println!(
            "{}",
            "No country filter will be used, so searches could be slower and less accurate."
                .italic()
        );
    }

    let mut internet = false;
    let station = match args.url {
        None => {
            let station: Station = match args.station {
                // If the station name is passed as an argument:
                Some(x) => {
                    let url = match config.clone().get_url_for(&x) {
                        Some(u) => u,
                        None => {
                            println!(
                                "{}",
                                "Station not found in local config, searching on the internet..."
                                    .yellow()
                                    .italic()
                            );

                            internet = true;

                            match browser::get_station(x.clone(), config.country_code.clone()) {
                                Ok(s) => s.url,
                                Err(e) => {
                                    perror("This station was not found :(");

                                    if args.debug {
                                        println!("{}", e);
                                    }

                                    std::process::exit(1);
                                }
                            }
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

            print!("Playing {}", station.station.green());

            if internet {
                println!(" ({})", station.url.yellow().italic());
            } else {
                println!();
            }

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
