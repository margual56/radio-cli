pub mod browser;
mod cache;
mod cli_args;
mod config;
mod errors;
mod station;
mod version;

pub use cache::Cache;
pub use cli_args::{Cli, Subcommands};
pub use config::Config;
pub use errors::{ConfigError, ConfigErrorCode};
pub use station::Station;
pub use version::Version;

use colored::*;

pub fn perror(msg: &str) {
    println!("{} {}", "Error:".red().bold(), msg);
}
