use crate::http_utils::types::{ParsedRequest, UniversalBody};
use std::collections::HashMap;
use crate::http_utils::status::ParseError;
use crate::http_utils::request::request_logic::query_to_map;

pub fn extract_path_from_buffer(buffer: &[u8]) -> Option<String> {
    let request_line = buffer
        .split(|&b| b == b'\r' || b == b'\n')
        .next()?; // Get the first line (request line)

    let request_line_str = std::str::from_utf8(request_line).ok()?;
    let mut parts = request_line_str.split_whitespace();

    let _method = parts.next()?; // Skip method
    let path = parts.next()?;    // This is the path
    Some(path.to_string())
}

pub fn extract_request_parts(parsed_request: ParsedRequest) -> Result<(UniversalBody, String, String, HashMap<String, String>, HashMap<String, String>), ParseError>{
    let (request_path, request_method, body, headers) = match &parsed_request {
        ParsedRequest::Api(api_req) => (api_req.path.as_str(), api_req.method.as_str(), api_req.body.clone(), api_req.headers.clone()),
        ParsedRequest::HTTP(http_req) => (http_req.path.as_str(), http_req.method.as_str(), http_req.body.clone(), http_req.headers.clone()),
    };

    println!("Body: {:?}", body);

    let (path, query) = match request_path.find('?') {
        Some(i) => {(&request_path[..i], &request_path[i + 1..])}
        None => (request_path, ""),
    };

    println!("Path: {}", path);
    println!("Query: {}", query);

    let query_map = query_to_map(query);

    println!("Query Map: {:#?}", query_map);

    Ok((body, path.to_string(), request_method.to_string(), query_map, headers))
}
