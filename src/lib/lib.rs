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
use directories::ProjectDirs;
pub use errors::{ConfigError, ConfigErrorCode};
pub use station::{Station, add_station, remove_station};
pub use version::Version;

use colored::*;

pub fn perror(msg: &str) {
    println!("{} {}", "Error:".red().bold(), msg);
}

pub const PROJECT_QUALIFIER: &str = "org";
pub const PROJECT_ORGANIZATION: &str = "margual56";
pub const PROJECT_NAME: &str = "radio-cli";
pub fn get_project_dirs() -> ProjectDirs {
    directories::ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_NAME)
        .expect("Error finding Home directory")
}
