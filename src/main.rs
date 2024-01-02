use clap::Parser;
use colored::*;
use inquire::{InquireError, Select};
use log::{debug, error, info, log_enabled, warn};
use radio_libs::{
    browser::{Browser, StationCache},
    perror, Cli, Config, ConfigError, Station, Version,
};
use std::io::Write;
use std::process::{Command, Stdio};
use std::rc::Rc;

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
            info!("{}", "Try passing the debug flag (-vvv). ".yellow());

            info!(
                "{}",
                "Deleting your config will download the updated one."
                    .yellow()
                    .bold()
            );

            std::process::exit(1);
        }
    };
    let config = Rc::new(config);

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

    if config.country_code.is_none() {
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

    let mut url = args.url;
    let mut station_arg = args.station;
    let mut cached_stations = None;
    loop {
        let station = match url {
            None => {
                let (station, internet, updated_cached_stations) =
                    get_station(station_arg, config.clone(), cached_stations.clone());
                if !args.no_station_cache {
                    cached_stations = updated_cached_stations;
                }

                print!("Playing {}", station.station.green());
                print!("\x1B]0;Now playing: {}\x07", station.station);

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

        // Don't play the same station again when returning to the browser
        url = None;
        station_arg = None;

        println!(
            "{}",
            "Info: press 'q' to stop playing this station"
                .italic()
                .bright_black()
        );

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
        .expect("Failed to execute mpv. Is it installed?");

    if !output.status.success() {
        eprintln!("mpv error: {:?}", output.status);
        std::io::stderr().write_all(&output.stderr).unwrap();
    } else {
        std::io::stdout().write_all(&output.stdout).unwrap();
    }

    output.status
}

fn get_station(
    station: Option<String>,
    config: Rc<Config>,
    cached_stations: Option<StationCache>,
) -> (Station, bool, Option<StationCache>) {
    let mut internet = false;

    match station {
        // If the station name is passed as an argument:
        Some(x) => {
            let (url, updated_cached_stations) = match config.get_url_for(&x) {
                Some(u) => (u, None),
                None => {
                    println!(
                        "{}",
                        "Station not found in local config, searching on the internet..."
                            .yellow()
                            .italic()
                    );

                    internet = true;

                    let (brows, updated_cached_stations) =
                        match Browser::new(config, cached_stations) {
                            Ok(b) => b,
                            Err(e) => {
                                error!("Could not connect with the API");

                                debug!("{}", e);

                                std::process::exit(1);
                            }
                        };

                    match brows.get_station(x.clone()) {
                        Ok(s) => (s.url, Some(updated_cached_stations)),
                        Err(e) => {
                            error!("This station was not found :(");
                            debug!("{}", e);

                            std::process::exit(1);
                        }
                    }
                }
            };

            (
                Station { station: x, url },
                internet,
                updated_cached_stations,
            )
        }

        // Otherwise
        None => {
            // And let the user choose one
            match prompt(config, cached_stations) {
                Ok((s, b, cached)) => (s, b, cached),
                Err(error) => {
                    println!("\n\t{}", "Bye!".bold().green());

                    info!("({:?})", error);

                    std::process::exit(0);
                }
            }
        }
    }
}

/// Prompts the user to select a station.
/// Returns a station and if the station was taken from the internet.
pub fn prompt(
    config: Rc<Config>,
    cached_stations: Option<StationCache>,
) -> Result<(Station, bool, Option<StationCache>), InquireError> {
    let max_lines: usize = match config.max_lines {
        Some(x) => x,
        None => Select::<Station>::DEFAULT_PAGE_SIZE,
    };

    let res = Select::new(&"Select a station to play:".bold(), config.data.clone())
        .with_page_size(max_lines)
        .prompt();

    let internet: bool;
    let (station, updated_cached_stations) = match res {
        Ok(s) => {
            if s.station.eq("Other") {
                internet = true;
                let result = Browser::new(config, cached_stations);

                let (brow, updated_cached_stations) = match result {
                    Ok((b, updated_cached_stations)) => (b, updated_cached_stations),
                    Err(_e) => return Err(InquireError::OperationInterrupted),
                };

                match brow.prompt() {
                    Ok(r) => (r, Some(updated_cached_stations)),
                    Err(e) => return Err(e),
                }
            } else {
                internet = false;
                (s, None)
            }
        }
        Err(e) => return Err(e),
    };

    Ok((station, internet, updated_cached_stations))
}
