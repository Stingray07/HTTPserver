use serde_json::Value;

use crate::http_utils::api::ApiRequest;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: UniversalBody,
}

#[derive(Debug)]
pub enum ParsedRequest {
    Api(ApiRequest),
    HTTP(HttpRequest),
}

#[derive(Debug, Clone)]
pub enum UniversalBody {
    Json(Value),
    Binary(Vec<u8>),
    Text(String)
}

use std::net::TcpStream;
use std::io::{Read, Write};

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

pub fn read_until_body<'a>(stream: &mut TcpStream, pre_buffer: &mut [u8], dynamo_buffer: &'a mut Vec<u8>) -> Result<&'a mut Vec<u8>, std::io::Error> {
    loop {
        match stream.read(pre_buffer) {
            Ok(0) => {
                eprintln!("Connection closed before complete headers");
                return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Connection closed before complete headers"))
            }
            Ok(n) => {
                dynamo_buffer.extend_from_slice(&pre_buffer[..n]); // Only use bytes read
                if dynamo_buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return Err(e); 
            }
        }
    }
    Ok(dynamo_buffer)
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
