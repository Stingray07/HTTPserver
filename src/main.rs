use std::net::TcpListener;
use std::io::{Read, Write};

mod routes;

struct HttpRequest{
    method: String,
    path: String, 
    version: String,
    body: String,
    headers: Vec<(String, String)>,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        // this is where the request was put
        let mut buffer = [0; 512];

        // read the request into the buffer
        stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        println!("================================");

        let parsed_request = parse_request(&mut buffer);

        // matching for routing
        let response: String = match parsed_request.path.as_str() {
            "/" => routes::handle_home(),
            "/about" => routes::handle_about(),
            _ => routes::handle_404(),
        };

        stream.write(response.as_bytes()).unwrap();

        //send the response right away because it might stay in the buffer
        stream.flush().unwrap();
    }
}

fn parse_request(buffer: & [u8]) -> HttpRequest {
    let request_string = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request_string.lines();

    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    
    let req = HttpRequest {
    method: parts.next().unwrap().to_string(),
    path: parts.next().unwrap().to_string(),
    version: parts.next().unwrap().to_string(),
    body: "".to_string(),
    headers: lines
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect(),
    };

    return req
}

