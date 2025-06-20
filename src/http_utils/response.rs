use std::net::TcpStream;
use std::io::Write;
use std::fs;
use std::path::Path;
use crate::http_utils::{response, status::{self, Status}, types::{ApiBody, ApiHttpResponse, HttpResponse, Response, UniversalBody}};
use serde_json::Value;
use std::collections::HashMap;


pub fn serve_file(file_path: &str) -> Vec<u8> {
    let base = Path::new("static");
    let path = base.join(file_path.trim_start_matches("/"));

    if !path.starts_with(base) {
        return build_response(Status::Forbidden, "text/html", b"FORBIDDEN")
    }

    let content_type = response::get_content_type(file_path);

    match fs::read(&path) {
        Ok(contents) => {
            response::build_response(Status::Ok, content_type, &contents)
        }
        Err(_) => {
            response::build_response(Status::NotFound, "404 Not Found", b"The requested file was not found")
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

fn match_content_type(content_type: &str, body: &[u8]) -> UniversalBody {
    match content_type {
        "text/plain" => UniversalBody::Text(String::from_utf8_lossy(body).to_string()),
        "application/json" => {
            UniversalBody::Json(Value::String(String::from_utf8_lossy(body).to_string()))
        },
        "application/octet-stream" => UniversalBody::Binary(Vec::from(body)),
        _ => UniversalBody::Text(String::from_utf8_lossy(body).to_string()),
    }
}

// TODO: DRY, ALSO COULD/SHOULD(?) RETURN STRUCT INSTEAD OF Vec<u8>
pub fn build_response(status: Status, content_type: &str, body: &[u8], response_type: Response) -> Vec<u8> {
    let status_line = String::from_utf8_lossy(status.line()).to_string();
    let mut response_header = HashMap::new();
    match response_type {
        Response::HTTP(_) => {
            response_header.insert("Content-Type".to_string(), content_type.to_string());
            response_header.insert("Content-Length".to_string(), body.len().to_string());
            response_header.insert("Status".to_string(), status_line);

            // TODO: Convert to HTTP format

            let response = HttpResponse {
                headers: response_header,
                body: match_content_type(content_type, body),
            };
            response 

            // TODO: Convert to Vec<u8>
        }
        Response::Api(_) => {
            response_header.insert("Content-Type".to_string(), content_type.to_string());
            response_header.insert("Content-Length".to_string(), body.len().to_string());
            response_header.insert("Status".to_string(), status_line);

            // TODO: Convert to HTTP format

            let response = ApiHttpResponse {
                headers: response_header,
                body: ApiBody {
                    status: status_line,
                    body: Value::String(String::from_utf8_lossy(body).to_string()),
                },
            };
            response 

            // TODO: Convert to Vec<u8>
        }
    }
}

pub fn api_response(status: Status, body: &[u8]) -> Vec<u8> {
    let body: JsonValue = serde_json::from_slice(body).unwrap();

    let response_body = ApiBody {
        status: String::from_utf8_lossy(status.line()).to_string(),
        body: body,
    };
    let res = serde_json::to_vec(&response_body).unwrap();
    build_response(status, "application/json", &res)
}

pub fn html_response(status: Status, title: &str, message: &str) -> Vec<u8> {
    let body = format!(
        "<html><body><h1>{}</h1><p>{}</p></body></html>",
        title, message
    );
    build_response(status, "text/html", body.as_bytes())
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

// TODO: Separate this maybe
pub fn send_chunky_body(body: &Vec<u8>) {
    let chunk_size: usize = 8;  
    let mut i = 0;

    while i < body.len() {
        let chunk = build_chunk(body, chunk_size, i);
        let response = chunk;
        i += chunk_size;
    };

    let _ = send_response(stream, b"0\r\n\r\n".to_vec());
}