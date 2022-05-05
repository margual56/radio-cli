use std::fmt::{Debug, Display, Formatter, Result};

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
        write!(
            f,
            "{{ code: {:?}, message: \"{}\", info: {} }}",
            self.code, self.message, self.extra
        ) // programmer-facing output
    }
}
