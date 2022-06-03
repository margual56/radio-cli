mod config;
mod errors;
mod browser;
mod station;
mod version;

pub use config::Config;
pub use errors::{ConfigError, ConfigErrorCode};
pub use station::Station;
pub use version::Version;
pub use browser::get_station;

use colored::*;

pub fn perror(msg: &str) {
    println!("{} {}", "Error:".red().bold(), msg);
}
