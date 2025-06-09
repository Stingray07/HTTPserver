use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


mod http_utils;
mod routes;
mod api;

use http_utils::parser;
use http_utils::status::ParseError;
use http_utils::api as api_utils;
use http_utils::response;
use http_utils::request::{self, ParsedRequest};
use routes::web;
use api::v1;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];

        if let Err(e) = stream.read(&mut buffer) {
            eprintln!("Failed to read from stream: {}", e);
            continue;
        }

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("================================");

        let is_api = http_utils::request::is_api_request(&buffer);
        println!("Is API: {}", is_api);
        
        let parsed_request = match parse_helper(is_api, &buffer) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Parse Failed: {:?}", e);
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
        };

        println!("Parsed Request: {:?}", parsed_request);

        let (request_path, request_method, body) = match &parsed_request {
            ParsedRequest::Api(api_req) => (api_req.path.as_str(), api_req.method.as_str(), api_req.body.clone()),
            ParsedRequest::HTTP(http_req) => (http_req.path.as_str(), http_req.method.as_str(), http_req.body.clone()),
        };

        println!("Body: {:?}", body);

        //MATCH FOR BOTH API AND HTTP
        let response: Vec<u8> = match (request_method, request::sanitize_path(request_path)) {
            (_, Some("400")) => web::handle_400(),
            ("GET", Some("/api/v1/users")) => v1::users::handle_get_user(),
            ("POST", Some("/api/v1/posts")) => v1::posts::handle_post_post(body),
            ("GET", Some("/")) => web::handle_home(),
            ("GET", Some("/about")) => web::handle_about(),
            ("GET",  Some("/submit")) => web::handle_submit_get(),
            ("POST", Some("/submit/json")) => web::submit_post_handler(body),
            ("POST", Some("/submit/text")) => web::submit_post_handler(body),
            ("POST", Some("/submit/binary")) => web::submit_post_handler(body),
            ("GET", Some(request_path)) => response::serve_file(request_path),

            (_, None) => web::handle_403(),
            _ => web::handle_404(),
        };

        let _ = send_response(&mut stream, response);
    }
}

fn send_response(stream: &mut TcpStream, response: Vec<u8>) -> std::io::Result<()> {
    log_response(&response);
    stream.write_all(&response).unwrap();

    // Send the response right away because it might stay in the buffer
    stream.flush().unwrap();
    Ok(())
}


fn log_response(response: &[u8]) {
    // Find the end of headers (\r\n\r\n)
    if let Some(pos) = response.windows(4).position(|w| w == b"\r\n\r\n") {
        println!("=== Response Headers ===");
        println!("{}", String::from_utf8_lossy(&response[..pos]));
        
        let body_start = pos + 4;
        println!("Body length: {} bytes", response.len() - body_start);
        
        // Print text bodies
        if let Ok(text) = String::from_utf8(response[body_start..].to_vec()) {
            if !text.is_empty() {
                println!("Body:\n{}", text);
            }
        }
    } else {
        println!("[Malformed HTTP response]");
    }
    println!("================================");
}

fn parse_helper(is_api: bool, buffer: &[u8]) -> Result<ParsedRequest, ParseError> {

    if is_api {
        match parser::parse_api_request(buffer) {
            Ok(req) => Ok(ParsedRequest::Api(req)),
            Err(e) => Err(e),
        }
    } else {
        match parser::parse_web_request(buffer) {
            Ok(req) => Ok(ParsedRequest::HTTP(req)),
            Err(e) => Err(e),
        }
    }
}

