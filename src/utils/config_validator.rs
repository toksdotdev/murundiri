use crate::utils::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_yaml;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum RuleAction {
    #[serde(rename = "redirect")]
    Redirect { url: String },

    #[serde(rename = "respond")]
    Respond {
        success: Option<Json>,
        failure: Option<Json>,
    },
}

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
    action: RuleAction,
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

impl Default for Rule {
    fn default() -> Self {
        Self {
            timeframe: 2,
            fields: RuleFields::default(),
            action: RuleAction::default(),
        }
    }
}

impl Default for RuleFields {
    fn default() -> Self {
        Self {
            body: None,
            query: None,
            header: None,
        }
    }
}

impl Default for RuleAction {
    fn default() -> Self {
        RuleAction::Respond {
            failure: Some(
                json!({ "status": "failed", "message": "Idepotency guarantee failed." }).into(),
            ),

            success: Some(
                json!({ "status": "success", "message": "Idepotency guaranteed uniqueness." })
                    .into(),
            ),
        }
    }
}

impl Into<Json> for Value {
    fn into(self) -> Json {
        Json(self)
    }
}

#[test]
fn parses_config() {
    use crate::hashmap_populate as hashmap;

    let yaml = r#"
port: 8080
services:
  book_service:
    rules:
      ^/api/v1/books:
        timeframe: 2 #ms
        action:
          respond:
            success: { status: true, "message": "Is idempotent." } # data about the request is appended automatically.
            failure: { status: false, "message": "Not idempotent." } # data about the request is appended automatically.
        fields:
          body:
            - trx_id
            - apple_id
  
#   billing_service:
#     rules:
#       ^/api/v1/recipient:
#         timeframe: 5 #ms
#         action:
#           respond:
#            sucess:
#            failure:
#         fields:
#           body:

  shipping_service:
    rules:
      \w+/api/ships:
        timeframe: 3
        action:
          redirect: 
            url: https://google.com:9000/v2
        fields:
          query:
            - ship_id
            - reference
          header:
            - Authorization
            - Content-Type
"#;

    let parsed_config = Config::parse(yaml).unwrap();

    let expected_config = Config {
        port: "8080".to_string(),
        services: hashmap![
            "book_service".to_string() => Service {
                rules: hashmap![
                    "^/api/v1/books".to_string() => Rule {
                        timeframe: 2,
                        action: RuleAction::Respond {
                            success: Some(json!({"status":true, "message": "Is idempotent."}).into()),
                            failure: Some(json!({"status":false, "message": "Not idempotent."}).into()),
                        },
                        fields: RuleFields {
                            body: Some(["trx_id".to_string(), "apple_id".to_string()].to_vec()),
                            query: None,
                            header: None,
                        },
                    }
                ],
            },


            // "billing_service".to_string() => Service {
            //     rules: hashmap![
            //         "^/api/v1/recipient".to_string() => Rule {
            //             timeframe: 5,
            //             action: RuleAction::default(),
            //             fields: RuleFields::default(),
            //         }
            //     ],
            // },

            "shipping_service".to_string() => Service {
                rules: hashmap![
                    r#"\w+/api/ships"#.to_string() => Rule {
                        timeframe: 3,
                        action: RuleAction::Redirect {
                            url: "https://google.com:9000/v2".to_string()
                        },
                        fields: RuleFields {
                            body: None,
                            query: Some(["ship_id".to_string(), "reference".to_string()].to_vec()),
                            header: Some(["Authorization".to_string(), "Content-Type".to_string()].to_vec()),
                        }
                    }
                ]
            }
        ],
    };

    assert_eq!(parsed_config, expected_config);
}
