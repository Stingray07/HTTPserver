use std::net::TcpListener;
use std::io::{Read, Write};

mod routes;
mod http_utils;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        // this is where the request was put
        let mut buffer = [0; 1024];

        // read the request into the buffer
        stream.read(&mut buffer).unwrap();

        let parsed_request = http_utils::request::parse_request(&mut buffer);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("Request Body: {}", parsed_request.body.as_str());
        println!("================================");

        let request_path = parsed_request.path.as_str();
        let request_method = parsed_request.method.as_str();


        // match for routing
        let response: Vec<u8> = match (request_method, request_path) {
            ("GET", "/") => routes::handle_home(),
            ("GET", "/about") => routes::handle_about(),
            ("GET",  "/submit") => routes::handle_submit_get(),
            ("POST", "/submit") => routes::handle_submit_post(),
            ("GET", "/index") => routes::serve_file("index.html"),
            ("GET", "/style.css") => routes::serve_file("style.css"),
            ("GET", "/scripts.js") => routes::serve_file("scripts.js"),
            ("GET", "/images/yasuo.jpg") => routes::serve_file("images/yasuo.jpg"),

            _ => routes::handle_404(),
        };

        log_response(&response);
        stream.write_all(&response).unwrap();

        //send the response right away because it might stay in the buffer
        stream.flush().unwrap();
    }
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

