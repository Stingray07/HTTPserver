pub struct HttpRequest{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub enum ParseError {
    MalformedRequest
}


pub fn parse_request(buffer: &[u8]) -> Result<HttpRequest, ParseError> {
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