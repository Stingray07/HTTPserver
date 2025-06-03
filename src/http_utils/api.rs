use std::collections::HashMap;
use serde_json::Value as JsonValue;
use serde::Serialize;  

pub struct ApiRequest {
    pub path: String,
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: JsonValue,  // Using serde_json::Value for parsed JSON
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: JsonValue,
}