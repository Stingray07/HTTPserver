use crate::http_utils::status::Status;
use crate::http_utils::response::api_response;
use std::collections::HashMap;

pub fn handle_get_user(query_map: HashMap<String, String>) -> Vec<u8> {
    api_response(Status::Ok, b"{\"name\": \"Stingray Get User\"}")
}