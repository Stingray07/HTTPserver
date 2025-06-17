use crate::api::v1;
use crate::http_utils::response;
use crate::http_utils::status::ParseError;
use crate::routes::web;
use crate::http_utils::types::{ParsedRequest, UniversalBody};
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{Read};

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

pub fn read_header<'a>(stream: &mut TcpStream, pre_buffer: &mut [u8], dynamo_buffer: &'a mut Vec<u8>) -> Result<&'a mut Vec<u8>, ParseError> {
    loop {
        match stream.read(pre_buffer) {
            Ok(0) => {
                eprintln!("Connection closed before complete headers");
                return Err(ParseError::ConnectionAborted)
            }
            Ok(n) => {
                dynamo_buffer.extend_from_slice(&pre_buffer[..n]); // Only use bytes read
                if dynamo_buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("Connection timed out");
                return Err(ParseError::ConnectionAborted);
            }

            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return Err(ParseError::MalformedRequest); 
            }
        }
    }
    Ok(dynamo_buffer)
}


//TODO: TIMEOUT CHECK HERE
pub fn read_body<'a>(content_length: Result<usize, ParseError>, stream: &mut TcpStream, full_body: &'a mut Vec<u8>) -> Result<&'a mut Vec<u8>, ParseError> {
    match content_length {
        Ok(content_length) => {
            if content_length == 0 {
                full_body.clear();
            } else {
                let mut body_buffer = vec![0; content_length - full_body.len()];
                let _ = stream.read_exact(&mut body_buffer);

                full_body.extend_from_slice(&body_buffer);
            }
            Ok(full_body)
        }
        Err(ParseError::ConnectionAborted) => {
            eprintln!("Connection timed out");
            return Err(ParseError::ConnectionAborted);
        }
        Err(ParseError::MalformedRequest) => {
            eprintln!("Failed to get content length");
            return Err(ParseError::MalformedRequest);
        }
    }
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

pub fn route_request(request_method: &str, path: &str, body: UniversalBody, query_map: HashMap<String, String>) -> Result<Vec<u8>, ParseError> {
    //MATCH FOR BOTH API AND HTTP
    let response: Vec<u8> = match (request_method, sanitize_path(path)) {
        (_, Some("400")) => web::handle_400(),
        ("GET", Some("/api/v1/users")) => v1::users::handle_get_user(query_map),
        ("POST", Some("/api/v1/posts")) => v1::posts::handle_post_post(query_map, body),
        ("GET", Some("/")) => web::handle_home(),
        ("GET", Some("/about")) => web::handle_about(),
        ("GET",  Some("/submit")) => web::handle_submit_get(query_map),
        ("POST", Some("/submit/json")) => web::submit_post_handler(query_map, body),
        ("POST", Some("/submit/text")) => web::submit_post_handler(query_map, body),
        ("POST", Some("/submit/binary")) => web::submit_post_handler(query_map, body),
        ("GET", Some(path)) => response::serve_file(path),

        (_, None) => web::handle_403(),
        _ => web::handle_404(),
    };
    Ok(response)
}

pub fn error_handler(error: ParseError) -> Vec<u8> {
    match error {
        ParseError::MalformedRequest => web::handle_400(),
        ParseError::ConnectionAborted => web::handle_408(),
        _ => web::handle_500(),
    }
}