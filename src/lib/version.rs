use colored::*;
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as ResultFmt};

#[derive(Deserialize, Debug, Clone)]
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
}
