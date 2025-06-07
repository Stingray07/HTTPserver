use crate::http_utils::status::Status;
use crate::http_utils::response::api_response;
use crate::http_utils::request::UniversalBody;

pub fn handle_post_post(body: UniversalBody) -> Vec<u8> {
    
    api_response(Status::Ok, b"{\"name\": \"Stingray Post Post\"}")
}