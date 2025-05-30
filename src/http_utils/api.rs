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
    let request_str = std::str::from_utf8(buffer).map_err(|_| ParseError::MalformedRequest)?;
    
    // Split into lines
    let lines: Vec<&str> = request_str.split('\n').collect();
    
    // Parse request line (first line)
    let request_line = lines.first().ok_or(ParseError::MalformedRequest)?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err(ParseError::MalformedRequest);
    }
    
    let method = parts[0].to_string();
    let path = parts[1].to_string();
    
    // Parse headers
    let mut headers = HashMap::new();
    let mut body_start = None;
    
    // Find where headers end and body starts
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim().is_empty() {
            body_start = Some(i + 1);
            break;
        }
        let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
        if header_parts.len() == 2 {
            headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
        }
    }
    
    // Get body
    let body_str = if let Some(start) = body_start {
        lines[start..].join("\n")
    } else {
        String::new()
    };
    
    let body: JsonValue = if !body_str.trim().is_empty() {
        serde_json::from_str(&body_str).map_err(|_| ParseError::MalformedRequest)?
    } else {
        JsonValue::Null
    };
    
    Ok(ApiRequest {
        path,
        method,
        headers,
        body,
    })
}
