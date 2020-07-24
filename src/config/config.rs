use crate::config::{constructs::Service, errors::ConfigParseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    port: String,
    services: HashMap<String, Service>,
}

impl Config {
    pub fn new(port: String, services: HashMap<String, Service>) -> Self {
        Self { port, services }
    }

    pub fn from_file(file_path: &str) -> Result<Self, ConfigParseError> {
        Self::parse(&fs::read_to_string(file_path).map_err(|err| ConfigParseError::Io(err))?)
    }

    pub fn parse(content: &str) -> Result<Self, ConfigParseError> {
        serde_yaml::from_str(&content).map_err(|err| ConfigParseError::InvalidSyntax(err))
    }
}
