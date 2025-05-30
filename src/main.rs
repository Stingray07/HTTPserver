use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::path::Path;


mod http_utils;
mod routes;

use http_utils::status::ParseError;
use http_utils::api;
use http_utils::response;
use http_utils::request::{self, ParsedRequest};
use routes::web;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        // this is where the request was put
        let mut buffer = [0; 1024];

        // read the request into the buffer
        if let Err(e) = stream.read(&mut buffer) {
            eprintln!("Failed to read from stream: {}", e);
            continue;
        }

        let is_api = http_utils::request::is_api_request(&buffer);

        let parsed_request: ParsedRequest;
        let request_path: &str;
        let request_method: &'static str;
        

        let (request_path, request_method) = match some_helper(is_api, &buffer) {
            Ok(ParsedRequest::Api(ref api_req)) => (api_req.path.as_str(), api_req.method.as_str()),
            Ok(ParsedRequest::HTTP(ref http_req)) => (http_req.path.as_str(), http_req.method.as_str()),
            Err(_) => {
                eprintln!("Parse Failed");
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
        };
        


        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("================================");

        //MATCH FOR BOTH API AND HTTP
        
        // match for routing
        let response: Vec<u8> = match (request_method, request::sanitize_path(request_path)) {
            (_, Some("400")) => web::handle_400(),
            ("GET", Some("/")) => web::handle_home(),
            ("GET", Some("/about")) => web::handle_about(),
            ("GET",  Some("/submit")) => web::handle_submit_get(),
            ("POST", Some("/submit")) => web::handle_submit_post(&parsed_request),
            ("GET", Some(request_path)) => response::serve_file(request_path),

            (_, None) => web::handle_403(),
            
            _ => web::handle_404(),
        };

        // Modify later
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

fn some_helper(is_api: bool, buffer: &[u8]) -> (Result<ParsedRequest, ParseError>) {

    if is_api {
        match api::parse_api_request(buffer) {
            Ok(req) => Ok(ParsedRequest::Api(req)),
            Err(e) => Err(e),
        }
    } else {
        match request::parse_web_request(buffer) {
            Ok(req) => Ok(ParsedRequest::HTTP(req)),
            Err(e) => Err(e),
        }
    }
}

