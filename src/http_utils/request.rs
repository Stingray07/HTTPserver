pub struct HttpRequest{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub fn parse_request(buffer: &[u8]) -> HttpRequest {
    let request_string = String::from_utf8_lossy(buffer);
    let mut sections = request_string.split("\r\n\r\n"); // Split headers/body
    
    // Parse headers section
    let headers_section = sections.next().unwrap();
    let mut lines = headers_section.lines();
    
    // Parse request line
    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    
    // Get headers
    let headers: Vec<_> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();
    
    // Get body (if exists)
    let body = sections.next().unwrap_or("").to_string();

    HttpRequest {
        method: parts.next().unwrap().to_string(),
        path: parts.next().unwrap().to_string(),
        version: parts.next().unwrap().to_string(),
        headers,
        body,
    }
}