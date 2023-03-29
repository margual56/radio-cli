use clap::Parser;
use colored::*;
use log::{debug, error, info, log_enabled, warn};
use radio_libs::{browser::Browser, perror, Cli, Config, ConfigError, Station, Version};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let version = match Version::from(String::from(env!("CARGO_PKG_VERSION"))) {
        Some(v) => v,
        None => {
            error!("There was an error parsing the program version");
            std::process::exit(1);
        }
    };

    // Parse the arguments
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    if args.list_countries {
        if let Ok(countries) = Browser::get_countries() {
            for country in countries {
                println!("{}: \"{}\"", country.name, country.iso_3166_1.bold());
            }
        } else {
            error!("Could not connect to the server, please check your connection.");
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
            debug!("{:?}", error);
            error!("{}", error);
            info!("{}", "Try pasing the debug flag (-vvv). ".yellow());

            info!(
                "{}",
                "Deleting your config will download the updated one."
                    .yellow()
                    .bold()
            );

            std::process::exit(1);
        }
    };

    debug!(
        "{} {}",
        "Program version:".bright_black().bold().italic(),
        format!("{}", version).bright_black().italic()
    );

    debug!(
        "{} {}",
        "Config version:".bright_black().bold().italic(),
        format!("{}", config.config_version).bright_black().italic()
    );

    if config.config_version.major < version.major {
        warn!("\n{} {}\n", "Warning!".yellow().bold(), 
		"The config version does not match the program version.\nThis might lead to parsing errors.".italic())
    }

    if let None = config.country_code {
        warn!("\n{} {}", "Warning!".yellow().bold(), 
		"The config does not contain a valid country (for example, \"ES\" for Spain or \"US\" for the US).".italic());
        info!(
            "{} {} {}\n",
            "You can use the option".italic(),
            "--list-countries".bold().italic(),
            "to see the available options.".italic()
        );
        warn!(
            "{}",
            "No country filter will be used, so searches could be slower and less accurate."
                .italic()
        );
    }

    let station = match args.url {
        None => {
            let (station, internet) = get_station(args.station, config);

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

    let output_status = run_mpv(station, args.show_video);

    if !output_status.success() {
        perror(format!("mpv {}", output_status).as_str());

        if !log_enabled!(log::Level::Info) {
            println!(
                "{}: {}",
                "Hint".italic().bold(),
                "Try running radio-cli with the verbose flag (-vv or -vvv)".italic()
            );
        }

        std::process::exit(2);
    }
}

fn run_mpv(station: Station, show_video: bool) -> std::process::ExitStatus {
    let mut mpv = Command::new("mpv");
    let mut mpv_args: Vec<String> = [station.url].to_vec();

    if !show_video {
        mpv_args.push(String::from("--no-video"));
    }

    if !log_enabled!(log::Level::Info) {
        mpv_args.push(String::from("--really-quiet"));
    }

    let output = mpv
        .args(mpv_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("mpv error: {:?}", output.status);
        std::io::stderr().write_all(&output.stderr).unwrap();
    } else {
        std::io::stdout().write_all(&output.stdout).unwrap();
    }

    output.status
}


fn get_station(station: Option<String>, config: Config) -> (Station, bool) {
    let mut internet = false;

    match station {
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

                    let brows = match Browser::new(config) {
                        Ok(b) => b,
                        Err(e) => {
                            error!("Could not connect with the API");

                            debug!("{}", e);

                            std::process::exit(1);
                        }
                    };

                    match brows.get_station(x.clone()) {
                        Ok(s) => s.url,
                        Err(e) => {
                            error!("This station was not found :(");
                            debug!("{}", e);

                            std::process::exit(1);
                        }
                    }
                }
            };

            (
                Station {
                    station: String::from(x),
                    url,
                },
                internet,
            )
        }

        // Otherwise
        None => {
            // And let the user choose one
            match config.clone().prompt() {
                Ok((s, b)) => (s, b),
                Err(error) => {
                    println!("\n\t{}", "Bye!".bold().green());

                    info!("({:?})", error);

                    std::process::exit(0);
                }
            }
        }
    }
}
