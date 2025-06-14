use std::{collections::HashMap, fs};
use std::path::Path;
use crate::http_utils::{response, status::Status};
use serde_json::Value as JsonValue;
use crate::http_utils::types::ApiResponse;


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

pub fn build_response(status: Status, content_type: &str, body: &[u8]) -> Vec<u8> {
    let mut response = status.line().to_vec();
    response.extend_from_slice(format!(
        "Content-Type: {}\r\nContent-Length: {}\r\n",
        content_type,
        body.len()
    ).as_bytes());
    response.extend_from_slice(format!("\r\n").as_bytes());
    response.extend(body);
    response
}

pub fn api_response(status: Status, body: &[u8]) -> Vec<u8> {
    let body: JsonValue = serde_json::from_slice(body).unwrap();

    let response_body = ApiResponse {
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