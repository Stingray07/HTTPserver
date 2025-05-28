use std::io::{Read, Write};
use std::net::TcpStream;

#[test]
fn test_server_error_handling() {
    // Test malformed request
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write_all(b"GARBAGE DATA\r\n").unwrap();
    
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    // Verify 400 response
    assert!(
        String::from_utf8_lossy(&buffer).starts_with("HTTP/1.1 400")
    );
}

fn send_test_request(path: &str, method: &str) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let request = format!("{} {} HTTP/1.1\r\nHost: localhost\r\n\r\n", method, path);
    stream.write_all(request.as_bytes()).unwrap();
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer).into_owned()
}

#[test]
fn test_routes_integration() {
    // Start your server in a separate thread if not already running
    // std::thread::spawn(|| your_server::main());

    let test_cases = [
        ("GET", "/", "HOME", "200 OK"),
        ("GET", "/about", "ABOUT", "200 OK"),
        ("GET", "/submit", "SUBMIT GET", "200 OK"),
        ("POST", "/submit", "SUBMIT POST", "200 OK"),
        ("GET", "/nonexistent", "NOT FOUND", "404 NOT FOUND"),
    ];

    for (method, path, content, status) in test_cases {
        let response = send_test_request(path, method);
        assert!(response.contains(content), "Failed for {} {}", method, path);
        assert!(response.starts_with(&format!("HTTP/1.1 {}", status)));
    }
}

#[test]
fn test_error_conditions() {
    let test_cases = [
        ("GET", "/../etc/passwd", "403 FORBIDDEN"),
        ("GARBAGE", "/", "400 BAD REQUEST"),
    ];

    for (method, path, status) in test_cases {
        let response = send_test_request(path, method);
        assert!(response.starts_with(&format!("HTTP/1.1 {}", status)));
    }
}