use serde_yaml::Error as YamlError;
use std::fmt::{Display, Formatter, Result};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ConfigParseError {
    ReadError(IoError),
    InvalidSyntax(YamlError),
    InvalidRegexRule,
}

impl Display for ConfigParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let content = match self {
            ConfigParseError::ReadError(err) => format!(
                "The following error occured while reading config file: {}",
                err.to_string()
            ),
            ConfigParseError::InvalidSyntax(err) => match err.location() {
                None => err.to_string(),
                Some(location) => format!(
                    "Invalid syntax at line: {} col: {} with error: {}",
                    location.line(),
                    location.column(),
                    err.to_string()
                ),
            },
            ConfigParseError::InvalidRegexRule => "Invalid regex rule".to_string(),
        };

        f.write_str(&content)
    }
}
