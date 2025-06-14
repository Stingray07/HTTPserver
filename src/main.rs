use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


mod http_utils;
mod routes;
mod api;

use http_utils::parser;
use http_utils::status::ParseError;
use http_utils::types::ParsedRequest;
use http_utils::request::{self, read_header, read_body};
use routes::web;

use crate::http_utils::request::extract_request_parts;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut dynamo_buffer = Vec::new();
        let mut pre_buffer = [0; 1024]; 

        match read_header(&mut stream, &mut pre_buffer, &mut dynamo_buffer) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to read header from stream: {}", e);
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
        }

        let header_end = dynamo_buffer.windows(4).position(|window| window == b"\r\n\r\n");
        let content_length = parser::get_content_length(&dynamo_buffer);
        let body_start = header_end.unwrap() + 4;
        let already_read_body =  &dynamo_buffer[body_start..];
        let mut full_body = already_read_body.to_vec();

        match read_body(content_length, &mut stream, &mut full_body) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to read body from stream: {}", e);
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
        }
    
        //Combine header and body
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

        let (body, path, request_method, query_map) = match extract_request_parts(parsed_request) {
            Ok((body, path, request_method, query_map)) => {
                (body, path, request_method, query_map)
            }
            Err(e) => {
                eprintln!("Extract Failed: {:?}", e);
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
        };

        let response: Vec<u8> = match request::route_request(&request_method, &path, body, query_map) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Route Failed: {:?}", e);
                let _ = send_response(&mut stream, web::handle_400());
                continue;
            }
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

