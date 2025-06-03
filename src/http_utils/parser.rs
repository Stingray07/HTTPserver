use crate::api_utils::ApiRequest;
use crate::http_utils::status::ParseError;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use crate::request::HttpRequest;

pub fn parse_request_line(line: &str) -> Result<(String, String, String), ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(ParseError::MalformedRequest);
    }
    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();
    Ok((method, path, version))
}

pub fn parse_headers(lines: &[&str]) -> Result<HashMap<String, String>, ParseError> {
    let mut headers = HashMap::new();
    for line in lines {
        let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
        if header_parts.len() == 2 {
            headers.insert(header_parts[0].to_string(), header_parts[1].trim().to_string());
        }
    }
    Ok(headers)
}

pub fn parse_body(lines: &[&str]) -> Result<String, ParseError> {
    let body = lines.join("\r\n");
    Ok(body)
}

pub fn parse_web_request(buffer: &[u8]) -> Result<HttpRequest, ParseError> {
    let request_str = std::str::from_utf8(buffer).map_err(|_| ParseError::MalformedRequest)?;
    
    // Split into lines
    let lines: Vec<&str> = request_str.split('\n').collect();
    
    // Parse request line (first line)
    let request_line = lines.first().ok_or(ParseError::MalformedRequest)?;
    let (method, path, version) = parse_request_line(request_line)?;
    
    // Parse headers
    let headers = parse_headers(&lines[1..])?;
    let body_start = None;
    
    // Get body
    let body_str = parse_body(&lines[body_start.unwrap_or(1)..])?;
    
    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        body: body_str,
    })
}

pub fn parse_api_request(buffer: &[u8]) -> Result<ApiRequest, ParseError> {
    let request_str = std::str::from_utf8(buffer).map_err(|_| ParseError::MalformedRequest)?;
    
    // Split into lines
    let lines: Vec<&str> = request_str.split('\n').collect();
    
    // Parse request line (first line)
    let request_line = lines.first().ok_or(ParseError::MalformedRequest)?;
    let (method, path, version) = parse_request_line(request_line)?;
    
    // Parse headers
    let headers = parse_headers(&lines[1..])?;
    let body_start = None;
    
    //Get body
    let body_str = if let Some(start) = body_start {
        lines[start..].join("\n")
    } else {
        String::new()
    };
    let body_str = body_str.replace("\r\n", "");
    
    let body: JsonValue = if !body_str.is_empty() {
        match serde_json::from_str(&body_str) {
            Ok(val) => val,
            Err(e) => {
                println!("serde_json error: {}", e);
                return Err(ParseError::MalformedRequest);
            }
        }
    } else {
        JsonValue::Null
    };

    println!("Path: >{:?}<", path);
    println!("Method: >{:?}<", method);
    println!("Version: >{:?}<", version);
    println!("Headers: >{:?}<", headers);
    println!("Body: >{:?}<", body);
    
    Ok(ApiRequest {
        path,
        method,
        version,
        headers,
        body,
    })
}