use crate::http_utils::status::ParseError;
use crate::http_utils::api::ApiRequest;

pub struct HttpRequest{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub enum ParsedRequest {
    Api(ApiRequest),
    HTTP(HttpRequest),
}

pub fn parse_web_request(buffer: &[u8]) -> Result<HttpRequest, ParseError> {
    let request_string = String::from_utf8_lossy(buffer);
    let mut sections = request_string.split("\r\n\r\n"); // Split headers/body
    
    // Parse headers section
    let headers_section = sections.next().ok_or(ParseError::MalformedRequest)?;
    let mut lines = headers_section.lines();
    
    // Parse request line
    let request_line = lines.next().ok_or(ParseError::MalformedRequest)?;
    let mut parts = request_line.split_whitespace();

    let method = parts.next().ok_or(ParseError::MalformedRequest)?.to_string();
    let path = parts.next().ok_or(ParseError::MalformedRequest)?.to_string();
    let version = parts.next().ok_or(ParseError::MalformedRequest)?.to_string();
    
    // Get headers
    let headers: Vec<_> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();
    
    // Get body (if exists)
    let body = sections.next().unwrap_or("").to_string();

    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
    })
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
