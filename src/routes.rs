pub fn handle_about() -> String {
    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>About</h1></body></html>".to_string()
}

pub fn handle_home() -> String {
    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>HOME</h1></body></html>".to_string()
}

pub fn handle_404() -> String {
    "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h1>404 Page Not Found</h1></body></html>".to_string()
}

pub fn handle_submit_get() -> String {
    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>Submit Here</h1></body></html>".to_string()
}

pub fn handle_submit_post() -> String {
    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
    <html><body><h1>HEHEH</h1></body></html>".to_string()
}