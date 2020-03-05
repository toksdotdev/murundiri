use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Config {
    port: String,
    services: HashMap<String, Service>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Service {
    rules: HashMap<String, Rule>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Rule {
    timeframe: u64,
    fields: RuleFields,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct RuleFields {
    body: Option<Vec<String>>,
    query: Option<Vec<String>>,
    header: Option<Vec<String>>,
}

impl Config {
    pub fn new(file_path: String) -> Result<Self, String> {
        let content =
            fs::read_to_string(file_path).map_err(|_| "Invalid config path".to_string())?;

        Self::parse(&content).map_err(|_| "Failed to parse config file.".to_string())
    }

    fn parse(content: &str) -> Result<Self, String> {
        let parsed: Self = serde_yaml::from_str(&content).map_err(|e| match e.location() {
            Some(_) => format!("The following error occured while parsing: {}.", e),
            None => "Failed to parse config file.".to_string(),
        })?;

        Ok(parsed)
    }
}

#[test]
fn parses_config() {
    use crate::hashmap_populate as hashmap;

    let yaml = r#"
port: 8080
services:
  billing_service:
    rules:
      ^/api/v1/bill/uba:
        timeframe: 2 #ms
        fields:
          body:
            - trx_id
            - apple_id

  payment_service:
    rules:
      \w+/api/cashout:
        timeframe: 3
        fields:
          query:
            - users
            - password
          header:
            - Authorization
            - Content-Type
"#;

    let parsed_config = Config::parse(yaml).unwrap();

    let expected_config = Config {
        port: "8080".to_string(),
        services: hashmap![
            "billing_service".to_string() => Service {
                rules: hashmap![
                    "^/api/v1/bill/uba".to_string() => Rule {
                        timeframe: 2,
                        fields: RuleFields {
                            body: Some(["trx_id".to_string(), "apple_id".to_string()].to_vec()),
                            query: None,
                            header: None,
                        },
                    }
                ],
            },
            "payment_service".to_string() => Service {
                rules: hashmap![
                    r#"\w+/api/cashout"#.to_string() => Rule {
                        timeframe: 3,
                        fields: RuleFields {
                            body: None,
                            query: Some(["users".to_string(), "password".to_string()].to_vec()),
                            header: Some(["Authorization".to_string(), "Content-Type".to_string()].to_vec()),
                        }
                    }
                ]
            }
        ],
    };

    assert_eq!(parsed_config, expected_config);
}

