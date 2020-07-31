use crate::config::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    #[serde(default = "default_ttl")]
    pub ttl: usize,
    pub fields: RuleFields,
    pub action: RuleAction,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleFields {
    pub body: Option<Vec<String>>,
    pub query: Option<Vec<String>>,
    pub header: Option<Vec<String>>,
}

impl Stringify for RuleFields {
    fn stringify(&self) -> String {
        let mut key = "".to_string();

        if let Some(ref body) = self.body {
            key += &format!("body::{},", body.stringify());
        }

        if let Some(ref query) = self.query {
            key += &format!("query::{},", query.stringify());
        }

        if let Some(ref header) = self.header {
            key += &format!("header::{},", header.stringify());
        }

        key
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RuleAction {
    #[serde(rename = "redirect")]
    Redirect { uri: String },

    #[serde(rename = "respond")]
    Respond { success: Json, failure: Json },
}

impl Default for RuleAction {
    fn default() -> Self {
        RuleAction::Respond {
            success: json!({
                "status": "success",
                "message": "Idempotency guaranteed uniqueness."
            })
            .into(),

            failure: json!({
                "status": "failed",
                "message": "Idempotency guarantee failed."
            })
            .into(),
        }
    }
}

pub fn default_ttl() -> usize {
    86400000 // 1 day
}

pub trait Stringify {
    fn stringify(&self) -> String;
}

impl Stringify for Vec<String> {
    fn stringify(&self) -> String {
        self.iter().fold(String::new(), |acc, s| acc + s)
    }
}
