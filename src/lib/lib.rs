mod errors;
mod station;
mod version;
mod config;

pub use station::Station;
pub use version::Version;
pub use errors::{ConfigError, ConfigErrorCode};
pub use config::Config;

use colored::*;

pub fn perror(msg: &str) {
	println!("{} {}", "Error:".red().bold(), msg);
}