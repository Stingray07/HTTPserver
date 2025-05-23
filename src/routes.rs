use std::fs;
use std::path::Path;

use crate::get_content_type;

pub fn serve_file(file_path: &str) -> Vec<u8> {
    let path = Path::new("static").join(file_path.trim_start_matches("/"));
    let content_type = get_content_type(file_path);

    match fs::read(&path) {
        Ok(contents) => {
            let mut response  = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                content_type,
                contents.len()
            ).into_bytes();
            response.extend(contents);
            response
        }
        Err(_) => {
            let not_found_html = b"HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n\
                <html><body><h1>File Not Found</h1></body></html>";
            not_found_html.to_vec()
        }
    }
}

pub fn handle_about() -> Vec<u8> {
    let response: &'static [u8; 84] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>About</h1></body></html>";
    response.to_vec()
}

pub fn handle_home() -> Vec<u8> {
    let response: &'static [u8; 83] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>HOME</h1></body></html>";
    response.to_vec()
}

pub fn handle_404() -> Vec<u8> {
    let response: &'static [u8; 92] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>404 not found</h1></body></html>";
    response.to_vec()
}

pub fn handle_submit_get() -> Vec<u8> {
    let response: &'static [u8; 89] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>SUBMIT GET</h1></body></html>";
    response.to_vec()
}

pub fn handle_submit_post() -> Vec<u8> {
    let response: &'static [u8; 90] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>SUBMIT POST</h1></body></html>";
    response.to_vec()
}
