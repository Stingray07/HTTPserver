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