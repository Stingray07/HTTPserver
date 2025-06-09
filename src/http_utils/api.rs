use std::collections::HashMap;
use serde_json::Value as JsonValue;
use serde::Serialize;

use crate::http_utils::request::UniversalBody;  

#[derive(Debug)]
pub struct ApiRequest {
    pub path: String,
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: UniversalBody,  
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub body: JsonValue,
}