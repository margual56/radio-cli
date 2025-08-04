use colored::*;
use log::error;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as ResultFmt};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> ResultFmt {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for Version {
    fn default() -> Self {
        match Version::from(String::from(env!("CARGO_PKG_VERSION"))) {
            Some(v) => v,
            None => {
                error!("There was an error parsing the program version");
                Version {
                    major: 0,
                    minor: 0,
                    patch: 0,
                }
            }
        }
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
    pub fn from(v: String) -> Option<Version> {
        let nums: Vec<&str> = v.split('.').collect();

        if nums.len() < 3 {}

        let major = match nums[0].parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "{} ({}): {}",
                    "Version error".bright_red(),
                    "major".italic(),
                    e
                );
                return None;
            }
        };
        let minor = match nums[1].parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "{} ({}): {}",
                    "Version error".bright_red(),
                    "minor".italic(),
                    e
                );
                return None;
            }
        };
        let patch = match nums[2].parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "{} ({}): {}",
                    "Version error".bright_red(),
                    "patch".italic(),
                    e
                );
                return None;
            }
        };

        Some(Version {
            major,
            minor,
            patch,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
