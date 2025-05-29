use std::collections::HashMap;
use serde_json::Value as JsonValue;
use crate::http_utils::status::ParseError;  // Your existing error type


pub struct ApiRequest {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: JsonValue,  // Using serde_json::Value for parsed JSON
}

pub fn parse_api_request(buffer: &[u8]) -> Result<ApiRequest, ParseError> {
    let request_str = std::str::from_utf8(buffer).map_err(|_| ParseError)?;
    let body: JsonValue = serde_json::from_str(request_str).map_err(|_| ParseError)?;
    
    Ok(ApiRequest {
        path: "/api/endpoint".to_string(),  // Extract from buffer
        method: "POST".to_string(),         // Extract from buffer
        headers: HashMap::new(),            // Parse headers
        body,
    })
}