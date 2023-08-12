use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    /// Option: -u --url <URL>: Specifies an url to be played.
    #[clap(short, long, help = "Specifies an url to be played.")]
    pub url: Option<String>,

    /// Option: -s --station <station name>: Specifies the name of the station to be played
    #[clap(
        short,
        long,
        conflicts_with = "url",
        help = "Specifies the name of the station to be played."
    )]
    pub station: Option<String>,

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

    /// Flag: --list-countries: List all the available countries and country codes to put in the config.
    #[clap(
        long = "list-countries",
        help = "List all the available countries and country codes to put in the config."
    )]
    pub list_countries: bool,

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
