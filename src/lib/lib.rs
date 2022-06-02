mod config;
mod errors;
mod radiobrowser;
mod station;
mod version;

pub use config::Config;
pub use errors::{ConfigError, ConfigErrorCode};
pub use station::Station;
pub use version::Version;

use colored::*;

pub fn perror(msg: &str) {
    println!("{} {}", "Error:".red().bold(), msg);
}
