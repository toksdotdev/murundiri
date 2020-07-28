use super::errors::ConfigParseError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UriRegex(#[serde(with = "serde_regex")] Regex);

impl UriRegex {
    pub fn match_url(&self, uri: &str) -> bool {
        self.0.is_match(uri)
    }

    pub fn from_str(rg: &str) -> Result<Self, ConfigParseError> {
        Ok(Self(
            Regex::new(rg).map_err(|_| ConfigParseError::InvalidRegexRule)?,
        ))
    }
}

impl Eq for UriRegex {}

impl PartialEq for UriRegex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl std::hash::Hash for UriRegex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state);
    }
}

// impl From<&str> for UriRegex {
//     fn from(str: &str) -> Self {
//         Self(Regex::new(str).unwrap_or(Regex::new("").unwrap()))
//     }
// }
