use serde_yaml::Error as YamlError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ConfigParseError {
    Io(IoError),
    InvalidSyntax(YamlError),
}
