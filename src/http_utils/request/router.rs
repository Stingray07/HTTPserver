use crate::http_utils::types::{UniversalBody};
use std::collections::HashMap;
use crate::http_utils::status::ParseError;
use crate::http_utils::request::request_logic::sanitize_path;
use crate::routes::web;
use crate::api::v1;
use crate::http_utils::response;

pub fn route_request(request_method: &str, path: &str, body: UniversalBody, query_map: HashMap<String, String>) -> Result<Vec<u8>, ParseError> {
    //MATCH FOR BOTH API AND HTTP
    let response: Vec<u8> = match (request_method, sanitize_path(path)) {
        (_, Some("400")) => web::handle_400(),
        ("GET", Some("/api/v1/users")) => v1::users::handle_get_user(query_map),
        ("POST", Some("/api/v1/posts")) => v1::posts::handle_post_post(query_map, body),
        ("GET", Some("/")) => web::handle_home(),
        ("GET", Some("/about")) => web::handle_about(),
        ("GET",  Some("/submit")) => web::handle_submit_get(query_map),
        ("POST", Some("/submit/json")) => web::submit_post_handler(query_map, body),
        ("POST", Some("/submit/text")) => web::submit_post_handler(query_map, body),
        ("POST", Some("/submit/binary")) => web::submit_post_handler(query_map, body),
        ("GET", Some("/chunky")) => web::handle_transfer_chunk_encoding(),
        ("GET", Some(path)) => response::serve_file(path),

        (_, None) => web::handle_403(),
        _ => web::handle_404(),
    };
    Ok(response)
}