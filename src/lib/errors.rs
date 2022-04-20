use std::fmt::{Display, Debug, Formatter, Result};

#[derive(Debug)]
pub enum ConfigErrorCode {
    OpenError,
    ReadError,
    CloseError,
    ParseError,
}

pub struct ConfigError {
    pub code: ConfigErrorCode,
    pub message: String,
    pub extra: String,
}

// Implement std::fmt::Display for ConfigError
impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for ConfigError
impl Debug for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ code: {:?}, message: \"{}\" }}", self.code, self.message) // programmer-facing output
    }
}