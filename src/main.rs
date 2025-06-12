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
        let mut dynamo_buffer = Vec::new();
        let mut pre_buffer = [0; 1024]; 

        loop {
            match stream.read(&mut pre_buffer) {
                Ok(0) => {
                    eprintln!("Connection closed before complete headers");
                    break;
                }
                Ok(n) => {
                    dynamo_buffer.extend_from_slice(&pre_buffer[..n]); // Only use bytes read
                    if dynamo_buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    break; 
                }
            }
        }

        let header_end = dynamo_buffer.windows(4).position(|window| window == b"\r\n\r\n");
        let content_length = parser::get_content_length(&dynamo_buffer);
        let body_start = header_end.unwrap() + 4;
        let already_read_body =  &dynamo_buffer[body_start..];
        let mut full_body = already_read_body.to_vec();

        match content_length {
            Ok(content_length) => {
                if content_length == 0 {
                    full_body = Vec::new();
                } else {
                    let mut body_buffer = vec![0; content_length - already_read_body.len()];
                    let _ = stream.read_exact(&mut body_buffer);

                    full_body.extend_from_slice(&body_buffer);
                }
            }
            Err(_) => {
                eprintln!("Failed to get content length");
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
        }
    
        let mut full_request = dynamo_buffer[..body_start].to_vec();
        full_request.extend_from_slice(&full_body);

        println!("Request: {}", String::from_utf8_lossy(&full_request[..]));
        println!("================================");

        let is_api = http_utils::request::is_api_request(&full_request);
        println!("Is API: {}", is_api);
        
        let parsed_request = match parse_request_by_type(is_api, &full_request) {
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

fn parse_request_by_type(is_api: bool, buffer: &[u8]) -> Result<ParsedRequest, ParseError> {

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

