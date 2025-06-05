use crate::http_utils::status::Status;
use crate::http_utils::response::api_response;

pub fn handle_post_post() -> Vec<u8> {
    api_response(Status::Ok, b"{\"name\": \"Stingray Post Post\"}")
}