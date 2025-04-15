use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommands,

    /// Flag: --show-video: If *not* present, a flag is passed down to mpv to not show the video and just play the audio.
    #[clap(
        long = "show-video",
        help = "If *not* present, a flag is passed down to mpv to not show the video and just play the audio."
    )]
    pub show_video: bool,

    /// Option: -c --config: Specify a config file other than the default.
    #[clap(
        long,
        short,
        help = "Specify a different config file from the default one."
    )]
    pub config: Option<PathBuf>,

    /// Option: --country-code <CODE>: Specify a country code to filter the search results
    #[clap(
        long = "country-code",
        help = "Specify a country code to filter the search."
    )]
    pub country_code: Option<String>,

    /// Flag: --no-station-cache: Don't cache the station list loaded from the internet.
    #[clap(
        long = "no-station-cache",
        help = "Don't cache the station list loaded from the internet."
    )]
    pub no_station_cache: bool,

    /// Show extra info
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Show debug info
    #[structopt(short, long)]
    pub debug: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Subcommands {
    /// Add a new station to the config file.
    Add {
        /// The name of the station to add.
        #[arg(short, long)]
        name: String,

        /// The URL of the station to add.
        #[arg(short, long)]
        url: String,
    },
    /// Remove a station from the config file.
    Remove {
        /// The name of the station to remove.
        #[arg(short, long)]
        name: String,
    },
    /// List all the countries in the config file.
    ListCountries,

    /// List all the stations in the config file.
    ListStations,
    /// Search for stations by name or country code.
    Search {
        /// The name of the station to search for.
        name: Option<String>,
    },
    /// Play a station from the config file.
    Play {
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
    },
}
