use super::{url_regex::UriRegex, Rule};
use crate::config::errors::ConfigParseError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, path::Path};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    #[serde(default = "default_docker_client", rename = "docker")]
    docker_path: String,

    rules: HashMap<UriRegex, Rule>,
}

impl Config {
    pub fn new(docker_path: String, rules: HashMap<UriRegex, Rule>) -> Self {
        Self { docker_path, rules }
    }

    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, ConfigParseError> {
        Self::parse(&String::from_utf8_lossy(
            &fs::read(file_path).map_err(|err| ConfigParseError::ReadError(err))?,
        ))
    }

    pub fn parse(content: &str) -> Result<Self, ConfigParseError> {
        serde_yaml::from_str(&content).map_err(|err| ConfigParseError::InvalidSyntax(err))
    }

    pub fn add_rule(mut self, rgx: &str, rule: Rule) -> Result<Self, ConfigParseError> {
        self.rules.insert(UriRegex::from_str(rgx)?, rule);
        Ok(self)
    }

    pub fn get_rule(&self, path: &str) -> Option<&Rule> {
        Some(
            (&self.rules)
                .into_iter()
                .find(|rgx| rgx.0.match_url(path))?
                .1,
        )
    }
}

fn default_docker_client() -> String {
    "/var/run/docker.sock".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            docker_path: "/var/run/docker.sock".to_string(),
            rules: HashMap::new(),
        }
    }
}
