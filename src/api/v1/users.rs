use crate::http_utils::status::Status;
use crate::http_utils::response::api_response;

pub fn handle_get_user() -> Vec<u8> {
    api_response(Status::Ok, b"{\"name\": \"Stingray Get User\"}")
}