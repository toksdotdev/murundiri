use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Json(pub Value);

impl PartialEq for Json {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for Json {}

impl Into<Json> for Value {
    fn into(self) -> Json {
        Json(self)
    }
}
