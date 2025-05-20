use std::net::TcpListener;
use std::io::{Read, Write};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        // this is where the request was put
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                                      <html><body><h1>Hello, world!</h1></body></html>";

        stream.write(response.as_bytes()).unwrap();

        //send the response right away because it might stay in the buffer
        stream.flush().unwrap();
    }
}
