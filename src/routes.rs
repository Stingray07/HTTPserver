use crate::http_utils::{response::html_response, status};


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

pub fn handle_500() -> Vec<u8> {
    html_response(status::INTERNAL_ERROR, "INTERNAL SERVER ERROR", "SERVER ERROR")
}

pub fn handle_403() -> Vec<u8> {
    html_response(status::FORBIDDEN, "FORBIDDEN", "FORBIDDEN ACCESS")
}

