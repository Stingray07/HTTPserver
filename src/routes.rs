use crate::http_utils::{request::HttpRequest, response::html_response, status::Status};


pub fn handle_about() -> Vec<u8> {
    html_response(Status::Ok, "ABOUT", "about")
}

pub fn handle_home() -> Vec<u8> {
    html_response(Status::Ok, "HOME", "HOME")
}

pub fn handle_404() -> Vec<u8> {
    html_response(Status::NotFound, "NOT FOUND", "The requested file was not found")
}

pub fn handle_submit_get() -> Vec<u8> {
    html_response(Status::Ok, "SUBMIT GET", "SUBMIT GET")
}

pub fn handle_submit_post(request: &HttpRequest) -> Vec<u8> {
    html_response(Status::Ok, "SUBMIT POST", "SUBMIT POST")
}

pub fn handle_500() -> Vec<u8> {
    html_response(Status::InternalError, "INTERNAL SERVER ERROR", "SERVER ERROR")
}

pub fn handle_403() -> Vec<u8> {
    html_response(Status::Forbidden, "FORBIDDEN", "FORBIDDEN ACCESS")
}

pub fn handle_400() -> Vec<u8> {
    html_response(Status::BadRequest, "BAD_REQUEST", "BAD REQUEST")
}

