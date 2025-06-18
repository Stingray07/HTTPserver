use crate::http_utils::status::ParseError;
use crate::routes::web;
use std::collections::HashMap;

use crate::http_utils::request::extractor::extract_path_from_buffer;

pub fn sanitize_path(path: &str) -> Option<&str> {
    if path.contains("..") || path.contains('\0') || path.contains("/.") {
        None
    } else {
        Some(path)
    }
}

pub fn is_api_request(buffer: &[u8]) -> bool {
    let res = match extract_path_from_buffer(buffer) {
        Some(path) => path.starts_with("/api/"),
        None => false,
    };
    res
}

pub fn query_to_map(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&') {
        let (key, value) = match pair.split_once('=') {
            None => (pair, ""),
            Some((key, value)) => (key, value),
        };
        map.insert(key.to_string(), value.to_string());
    }
    map
}


pub fn error_handler(error: ParseError) -> Vec<u8> {
    match error {
        ParseError::MalformedRequest => web::handle_400(),
        ParseError::ConnectionAborted => web::handle_408(),
        _ => web::handle_500(),
    }
}
