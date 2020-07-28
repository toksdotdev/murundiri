use super::{url_regex::UriRegex, Rule};
use crate::config::errors::ConfigParseError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    #[serde(default = "default_ip")]
    ip: IpAddr,

    #[serde(default = "default_port")]
    port: u16,

    #[serde(default = "default_docker_client")]
    docker_client: String,

    rules: HashMap<UriRegex, Rule>,
}

impl Config {
    pub fn new(
        ip: IpAddr,
        port: u16,
        docker_client: String,
        rules: HashMap<UriRegex, Rule>,
    ) -> Self {
        Self {
            ip,
            port,
            docker_client,
            rules,
        }
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn from_file(file_path: &str) -> Result<Self, ConfigParseError> {
        Self::parse(&fs::read_to_string(file_path).map_err(|err| ConfigParseError::Io(err))?)
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

fn default_ip() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
}

fn default_port() -> u16 {
    80
}

fn default_docker_client() -> String {
    "/var/run/docker.sock".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: default_ip(),
            port: default_port(),
            docker_client: "/var/run/docker.sock".to_string(),
            rules: HashMap::new(),
        }
    }
}
