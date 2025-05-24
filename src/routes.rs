use std::fs;
use std::path::Path;

use crate::http_utils::{response::{self, html_response}, status};

pub fn serve_file(file_path: &str) -> Vec<u8> {
    let path = Path::new("static").join(file_path.trim_start_matches("/"));
    let content_type = response::get_content_type(file_path);

    match fs::read(&path) {
        Ok(contents) => {
            response::build_response(status::OK, content_type, &contents)
        }
        Err(_) => {
            response::build_response(status::NOT_FOUND, "404 Not Found", b"The requested file was not found")
        }
    }
}

pub fn handle_about() -> Vec<u8> {
    html_response(status::OK, "ABOUT", "about")
}

pub fn handle_home() -> Vec<u8> {
    html_response(status::OK, "HOME", "HOME")
}

pub fn handle_404() -> Vec<u8> {
    html_response(status::NOT_FOUND, "NOT FOUND", "The requested file was not found")
}

pub fn handle_submit_get() -> Vec<u8> {
    html_response(status::OK, "SUBMIT GET", "SUBMIT GET")
}

pub fn handle_submit_post() -> Vec<u8> {
    html_response(status::OK, "SUBMIT POST", "SUBMIT POST")
}
