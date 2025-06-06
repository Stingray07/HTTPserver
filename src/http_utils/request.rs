use serde_json::Value;

use crate::http_utils::api::ApiRequest;
use std::collections::HashMap;

pub struct HttpRequest{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: ParsedBody,
}

pub enum ParsedRequest {
    Api(ApiRequest),
    HTTP(HttpRequest),
}

#[derive(Debug)]
pub enum ParsedBody{
    Json(Value),
    Text(String),
    Binary(Vec<u8>)
}

pub fn sanitize_path(path: &str) -> Option<&str> {
    if path.contains("..") || path.contains('\0') || path.contains("/.") {
        None
    } else {
        Some(path)
    }
}

fn extract_path_from_buffer(buffer: &[u8]) -> Option<String> {
    let request_line = buffer
        .split(|&b| b == b'\r' || b == b'\n')
        .next()?; // Get the first line (request line)

    let request_line_str = std::str::from_utf8(request_line).ok()?;
    let mut parts = request_line_str.split_whitespace();

    let _method = parts.next()?; // Skip method
    let path = parts.next()?;    // This is the path
    Some(path.to_string())
}


pub fn is_api_request(buffer: &[u8]) -> bool {
    let res = match extract_path_from_buffer(buffer) {
        Some(path) => path.starts_with("/api/"),
        None => false,
    };
    res
}
