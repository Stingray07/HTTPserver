mod http_utils;
mod routes;
mod api;

use http_utils::parser::parse_request_by_type;
use http_utils::request::reader::full_read_request;
use http_utils::response::{send_response, log_response};

use crate::http_utils::request::request_logic::{is_api_request, error_handler};
use crate::http_utils::request::extractor::extract_request_parts;
use crate::http_utils::request::router::route_request;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

// TODO: Add timeout
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(&mut stream).await;
        });
    }
}


async fn handle_connection(stream: &mut TcpStream) {
    // stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
    // stream.set_write_timeout(Some(std::time::Duration::from_secs(10))).unwrap();

    loop {
        println!("Waiting for request...");
        let mut dynamo_buffer = Vec::new();
        let mut pre_buffer = [0; 1024]; 
    
        let full_request = match full_read_request(stream, &mut pre_buffer, &mut dynamo_buffer).await {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error reading request: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler).await;
                return;
            }
        };
    
        let is_api = is_api_request(&full_request);
        println!("Is API: {}", is_api);
        
        let parsed_request = match parse_request_by_type(is_api, &full_request) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error parsing request: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler).await;
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
                let _ = send_response(stream, handler).await;
                return; 
            }
        };
    
        let response: Vec<u8> = match route_request(&request_method, &path, body, query_map) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error routing request: {:?}", e);
                let handler = error_handler(e);
                let _ = send_response(stream, handler).await;
                return;
            }
        };
    
        log_response(&response);
        let _ = send_response(stream, response).await;

        if headers.get("Connection").unwrap_or(&"keep-alive".to_string()).to_lowercase() == "close" {
            println!("Close");
            return;
        }
    }
}

