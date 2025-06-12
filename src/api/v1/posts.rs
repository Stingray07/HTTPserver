use crate::http_utils::status::Status;
use crate::http_utils::response::api_response;
use crate::http_utils::request::UniversalBody;

pub fn handle_post_post(body: UniversalBody) -> Vec<u8> {
    match body {
        UniversalBody::Json(value) => {
            println!("JSON: {:?}", value);
            let name = value.get("name").and_then(|v| v.as_str());
            match name {
                Some(name) => {
                    let res = format!("{{\"namessssss\": \"{}\"}}", name);
                    api_response(Status::Ok, res.as_bytes())
                },
                None => {
                    println!("None");
                    api_response(Status::BadRequest, b"{\"error\": \"BAD REQUEST\"}")
                }
            }
        },
        _ => {
            println!("_");
            api_response(Status::BadRequest, b"{\"error\": \"BAD REQUEST\"}")
        }
    }
}