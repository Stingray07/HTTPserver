use serde_json::Value;
use std::collections::HashMap;

use crate::http_utils::{types::{HttpRequest, ParsedRequest, UniversalBody}, response::html_response, status::Status};


pub fn handle_about() -> Vec<u8> {
    html_response(Status::Ok, "ABOUT", "about")
}

pub fn handle_home() -> Vec<u8> {
    html_response(Status::Ok, "HOME", "HOME")
}

pub fn handle_404() -> Vec<u8> {
    html_response(Status::NotFound, "NOT FOUND", "The requested file was not found")
}

pub fn handle_submit_get(query_map: HashMap<String, String>) -> Vec<u8> {
    html_response(Status::Ok, "SUBMIT GET", "SUBMIT GET")
}

pub fn submit_post_handler(query_map: HashMap<String, String>, body: UniversalBody) -> Vec<u8> {
    match body {
        UniversalBody::Json(json) => {
            handle_submit_post_json(json)
        }
        UniversalBody::Text(text) => {
            handle_submit_post_text(text)
        }
        UniversalBody::Binary(binary) => {
            handle_submit_post_binary(binary)
        }
        _ => {
            html_response(Status::BadRequest, "UNKNOWN BODY TYPE", "UNKNOWN BODY TYPE")
        }
    }
}

pub fn handle_submit_post_json(json: Value) -> Vec<u8> {
    let message = json.to_string();
    html_response(Status::Ok, "SUBMIT POST", message.as_str())
}

pub fn handle_submit_post_text(body: String) -> Vec<u8> {
    html_response(Status::Ok, "SUBMIT POST", body.as_str())
}

pub fn handle_submit_post_binary(body: Vec<u8>) -> Vec<u8> {
    html_response(Status::Ok, "SUBMIT POST", "BINARY")
}

pub fn handle_transfer_chunk_encoding() -> Vec<u8> {
    html_response(Status::Ok, "TRANSFER CHUNK ENCODING", "TRANSFER CHUNK ENCODING")
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

pub fn handle_408() -> Vec<u8> {
    html_response(Status::RequestTimeout, "REQUEST TIMEOUT", "REQUEST TIMEOUT")
}
