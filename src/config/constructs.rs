use crate::config::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub timeframe: u64,
    pub fields: RuleFields,
    pub action: RuleAction,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleFields {
    pub body: Option<Vec<String>>,
    pub query: Option<Vec<String>>,
    pub header: Option<Vec<String>>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RuleAction {
    #[serde(rename = "redirect")]
    Redirect { url: String },

    #[serde(rename = "respond")]
    Respond { success: Json, failure: Json },
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Service {
    rules: HashMap<String, Rule>,
}

impl Service {
    pub fn add_rule(mut self, name: &str, rule: Rule) -> Self {
        self.rules.insert(name.to_string(), rule);
        self
    }
}

impl Default for Service {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
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
