use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::path::Path;


mod http_utils;
mod routes;

use http_utils::api;
use http_utils::response;
use http_utils::request::{self, parse_web_request};
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
        
        if is_api {
            http_utils::api::parse_api_request()
        } else {
            parse_web_request(buffer)
        }

        let parsed_request = match http_utils::request::parse_web_request(&buffer) {
            Ok(req) => req,
            Err(_) => {
                eprintln!("Parse Failed");
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }   
        };



        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("Request Body: {}", parsed_request.body.as_str());
        println!("================================");

        let request_path = parsed_request.path.as_str();
        let request_method = parsed_request.method.as_str();

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

