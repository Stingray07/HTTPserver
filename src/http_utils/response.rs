use std::net::TcpStream;
use std::io::Write;
use std::fs;
use std::path::Path;
use crate::http_utils::request::request_logic::error_handler;
use crate::http_utils::response;
use crate::http_utils::status::Status;
use crate::http_utils::status::ParseError;
use crate::http_utils::types::{ApiBody, Response, UniversalBody};
use serde_json::Value;
use std::collections::HashMap;


pub fn serve_file(file_path: &str) -> Result<Vec<u8>, ParseError> {
    let base = Path::new("static");
    let path = base.join(file_path.trim_start_matches("/"));

    if !path.starts_with(base) {
        return match build_response(Status::Forbidden, "text/html", b"FORBIDDEN") {
            Ok(response) => Ok(response.convert_to_vec()),
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(e);
            }
        }
    }

    let content_type = get_content_type(file_path);

    match fs::read(&path) {
        Ok(contents) => {
            match build_response(Status::Ok, content_type, &contents) {
                Ok(response) => Ok(response.convert_to_vec()),
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Err(e);
                }
            }
        }
        Err(_) => {
            match build_response(Status::NotFound, "404 Not Found", b"The requested file was not found") {
                Ok(response) => Ok(response.convert_to_vec()),
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Err(e);
                }
            }
        }
    }
}

pub fn get_content_type(file_path: &str) -> &str {
    match file_path {
        p if p.ends_with(".html") => "text/html",
        p if p.ends_with(".css") => "text/css",
        p if p.ends_with(".js") => "application/javascript",
        p if p.ends_with(".jpg") || p.ends_with(".jpeg") => "image/jpeg",
        p if p.ends_with(".png") => "image/png",
        _ => "text/plain",
    }
}

fn build_response_header(status: Status, content_type: &str, body: &[u8]) -> HashMap<String, String> {
    let mut response_header = HashMap::new();
    response_header.insert("Content-Type".to_string(), content_type.to_string());
    response_header.insert("Content-Length".to_string(), body.len().to_string());
    response_header.insert("Status".to_string(), String::from_utf8_lossy(status.line()).to_string());
    response_header
}

// TODO: DRY, ALSO COULD/SHOULD(?) RETURN STRUCT INSTEAD OF Vec<u8>
pub fn build_response(status: Status, content_type: &str, body: &[u8]) -> Result<Response, ParseError> {
    let status_line = String::from_utf8_lossy(status.line()).to_string();
    let response_header = build_response_header(status, content_type, body);
    let response = Response {
        status: status_line,
        headers: response_header,
        body: body.to_vec(),
    };
    Ok(response)
}

pub fn api_response(status: Status, body: &[u8]) -> Vec<u8> {
    let body: Value = serde_json::from_slice(body).unwrap();

    let response_body = ApiBody {
        status: String::from_utf8_lossy(status.line()).to_string(),
        body: body,
    };
    let res = serde_json::to_vec(&response_body).unwrap();
    let response = build_response(status, "application/json", &res).unwrap();
    response.convert_to_vec()
}

pub fn html_response(status: Status, title: &str, message: &str) -> Vec<u8> {
    let body = format!(
        "<html><body><h1>{}</h1><p>{}</p></body></html>",
        title, message
    );
    let response = build_response(status, "text/html", body.as_bytes()).unwrap();
    response.convert_to_vec()
}

pub fn send_response(stream: &mut TcpStream, response: Vec<u8>) -> std::io::Result<()> {
    log_response(&response);
    stream.write_all(&response).unwrap();

    // Send the response right away because it might stay in the buffer
    stream.flush().unwrap();
    Ok(())
}

pub fn log_response(response: &[u8]) {
    // Find the end of headers (\r\n\r\n)
    if let Some(pos) = response.windows(4).position(|w| w == b"\r\n\r\n") {
        println!("=== Response Headers ===");
        println!("{}", String::from_utf8_lossy(&response[..pos]));
        
        let body_start = pos + 4;
        println!("Body length: {} bytes", response.len() - body_start);
        
        // Print text bodies
        if let Ok(text) = String::from_utf8(response[body_start..].to_vec()) {
            if !text.is_empty() {
                println!("Body:\n{}", text);
            }
        }
    } else {
        println!("[Malformed HTTP response]");
    }
    println!("================================");
}

fn build_chunk(body: &Vec<u8>, chunk_size: usize, i: usize) -> Vec<u8> {
    let end = (i + chunk_size).min(body.len());
    let chunk = &body[i..end];
    let chunk_size = end - i;

    let mut response = format!("{:X}\r\n", chunk_size).as_bytes().to_vec();
    response.extend_from_slice(chunk);
    response.extend_from_slice("\r\n".as_bytes());
    response
}

// // TODO: Separate this maybe
// pub fn send_chunky_body(body: &Vec<u8>) {
//     let chunk_size: usize = 8;  
//     let mut i = 0;

//     while i < body.len() {
//         let chunk = build_chunk(body, chunk_size, i);
//         let response = chunk;
//         i += chunk_size;
//     };

//     let _ = send_response(stream, b"0\r\n\r\n".to_vec());
// }