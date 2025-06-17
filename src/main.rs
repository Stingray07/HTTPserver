use std::net::{TcpListener, TcpStream};

mod http_utils;
mod routes;
mod api;

use http_utils::parser;
use http_utils::parser::parse_request_by_type;
use http_utils::request::{read_header, read_body, route_request, error_handler};
use http_utils::response::{send_response, log_response};

use crate::http_utils::request::extract_request_parts;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        handle_connection(&mut stream);
        //TODO: Make this HTTP/1.1 compatible with keep-alive
    }
}


fn handle_connection(stream: &mut TcpStream) {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
    stream.set_write_timeout(Some(std::time::Duration::from_secs(10))).unwrap();

    loop {
        println!("Waiting for request...");
        let mut dynamo_buffer = Vec::new();
        let mut pre_buffer = [0; 1024]; 
    
        println!("Reading header...");
        match read_header(stream, &mut pre_buffer, &mut dynamo_buffer) {
            Ok(_) => {
                println!("Header read");
            },
            Err(e) => {
                eprintln!("Error reading header: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler);
                return;
            }
        }
        println!("Header read");
        let header_end = dynamo_buffer.windows(4).position(|window| window == b"\r\n\r\n");
        let content_length = parser::get_content_length(&dynamo_buffer);
        let body_start = header_end.unwrap() + 4;
        let already_read_body =  &dynamo_buffer[body_start..];
        let mut full_body = already_read_body.to_vec();
    
        match read_body(content_length, stream, &mut full_body) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error reading body: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler);
                return;
            }
        }
    
        //Combine header and body
        let mut full_request = dynamo_buffer[..body_start].to_vec();
        full_request.extend_from_slice(&full_body);
    
        println!("FULL Request: {}", String::from_utf8_lossy(&full_request[..]));
        println!("================================");
    
        let is_api = http_utils::request::is_api_request(&full_request);
        println!("Is API: {}", is_api);
        
        let parsed_request = match parse_request_by_type(is_api, &full_request) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error parsing request: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler);
                return;
            }
        };
    
        println!("FULL Parsed Request: {:?}", parsed_request);
    
        let (body, path, request_method, query_map, headers) = match extract_request_parts(parsed_request) {
            Ok((body, path, request_method, query_map, headers)) => {
                (body, path, request_method, query_map, headers)
            }
            Err(e) => {
                eprintln!("Error extracting request parts: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler);
                return; 
            }
        };
    
        let response: Vec<u8> = match route_request(&request_method, &path, body, query_map) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error routing request: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler);
                return;
            }
        };
    
        log_response(&response);
        let _ = send_response(stream, response);

        if headers.get("Connection").unwrap_or(&"keep-alive".to_string()).to_lowercase() == "close" {
            println!("Close");
            return;
        }
    }
}

