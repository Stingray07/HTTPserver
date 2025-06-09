use crate::api_utils::ApiRequest;
use crate::http_utils::request::UniversalBody;
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


pub fn trim_by_content_length(headers: HashMap<String, String>, buffer: &[u8], body_start: Option<usize>, method: &str) -> Result<Vec<u8>, ParseError> {
    if headers.get("Content-Length").is_none() && method != "POST" {
        return Ok(Vec::new());
    }

    let content_length = headers.get("Content-Length").ok_or(ParseError::MalformedRequest)?;
    let content_length = content_length.parse::<usize>().map_err(|_| ParseError::MalformedRequest)?;
    let start = body_start.unwrap_or(0);
    let end = start + content_length;
    Ok(buffer[start..end].to_vec())
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

    let header_end = buffer.windows(4).position(|window| window == b"\r\n\r\n");
    let body_start = header_end.map(|pos| pos + 4);

    let body = trim_by_content_length(headers.clone(), buffer, body_start, method.as_str())?;
    let body = deserialize_body(&body, headers.get("Content-Type").map_or("text/plain", |v| v))?;
    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
    })
}

//debating if I should merge parse_web_request and parse_api_request into one function

pub fn parse_api_request(buffer: &[u8]) -> Result<ApiRequest, ParseError> {
    let request_str = std::str::from_utf8(buffer).map_err(|_| ParseError::MalformedRequest)?;
    
    // Split into lines
    let lines: Vec<&str> = request_str.split('\n').collect();
    
    // Parse request line (first line)
    let request_line = lines.first().ok_or(ParseError::MalformedRequest)?;
    let (method, path, version) = parse_request_line(request_line)?;
    
    // Parse headers
    let headers = parse_headers(&lines[1..])?;

    let header_end = buffer.windows(4).position(|window| window == b"\r\n\r\n");
    let body_start = header_end.map(|pos| pos + 4);

    let body = trim_by_content_length(headers.clone(), buffer, body_start, method.as_str())?;
    let body = deserialize_body(&body, headers.get("Content-Type").map_or("text/plain", |v| v))?;

    println!("Path: >{:?}<", path);
    println!("Method: >{:?}<", method);
    println!("Version: >{:?}<", version);
    println!("Headers: >{:?}<", headers);
    println!("Body: >{:?}<", body);

    Ok(ApiRequest {
        method,
        path,
        version,
        headers,
        body,
    })
}


pub fn deserialize_body(body: &[u8], content_type: &str) -> Result<UniversalBody, ParseError> {
    match content_type {
        "application/json" => {
            println!("JSON: {:?}", body);
            let res = serde_json::from_slice(body)
                .map(UniversalBody::Json)
                .map_err(|_| ParseError::MalformedRequest);
            res
        }
        "text/plain" | "application/x-www-form-urlencoded" => {
            String::from_utf8(body.to_vec())
                .map(UniversalBody::Text)
                .map_err(|_| ParseError::MalformedRequest)
        }
        "application/octet-stream" => Ok(UniversalBody::Binary(body.to_vec())),
        _ => {
            String::from_utf8(body.to_vec())
                .map(UniversalBody::Text)
                .map_err(|_| ParseError::MalformedRequest)
        }
    }
}