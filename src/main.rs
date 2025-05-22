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

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("================================");

        let parsed_request = parse_request(&mut buffer);

        println!("body: {}", parsed_request.body.as_str());

        let request_path = parsed_request.path.as_str();
        let request_method = parsed_request.method.as_str();


        // match for routing
        let response: String = match (request_method, request_path) {
            ("GET", "/") => routes::handle_home(),
            ("GET", "/about") => routes::handle_about(),
            ("GET",  "/submit") => routes::handle_submit_get(),
            ("POST", "/submit") => routes::handle_submit_post(),

            _ => routes::handle_404(),
        };

        stream.write(response.as_bytes()).unwrap();

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

