use std::collections::HashMap;
use serde_json::Value as JsonValue;

pub struct ApiRequest {
    pub path: String,
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: JsonValue,  // Using serde_json::Value for parsed JSON
}
