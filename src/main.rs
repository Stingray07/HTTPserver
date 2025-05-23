use std::net::TcpListener;
use std::io::{Read, Write};

mod routes;

struct HttpRequest{
    method: String,
    path: String, 
    version: String,
    headers: Vec<(String, String)>,
    body: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        // this is where the request was put
        let mut buffer = [0; 1024];

        // read the request into the buffer
        stream.read(&mut buffer).unwrap();

        let parsed_request = parse_request(&mut buffer);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("Reqeust Body: {}", parsed_request.body.as_str());
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

fn parse_request(buffer: &[u8]) -> HttpRequest {
    let request_string = String::from_utf8_lossy(buffer);
    let mut sections = request_string.split("\r\n\r\n"); // Split headers/body
    
    // Parse headers section
    let headers_section = sections.next().unwrap();
    let mut lines = headers_section.lines();
    
    // Parse request line
    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    
    // Get headers
    let headers: Vec<_> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();
    
    // Get body (if exists)
    let body = sections.next().unwrap_or("").to_string();

    HttpRequest {
        method: parts.next().unwrap().to_string(),
        path: parts.next().unwrap().to_string(),
        version: parts.next().unwrap().to_string(),
        headers,
        body,
    }
}


fn get_content_type(file_path: &str) -> &str {
    match file_path {
        p if p.ends_with(".html") => "text/html",
        p if p.ends_with(".css") => "text/css",
        p if p.ends_with(".js") => "application/javascript",
        p if p.ends_with(".jpg") || p.ends_with(".jpeg") => "image/jpeg",
        p if p.ends_with(".png") => "image/png",
        _ => "text/plain",
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

