pub const OK: &[u8] = b"HTTP/1.1 200 OK\r\n";
pub const CREATED: &[u8] = b"HTTP/1.1 201 CREATED\r\n";
pub const NO_CONTENT: &[u8] = b"HTTP/1.1 201 NO BODY\r\n";


pub const NOT_FOUND: &[u8] = b"HTTP/1.1 404 NOT FOUND\r\n";
pub const BAD_REQUEST: &[u8] = b"HTTP/1.1 400 BAD REQUEST\r\n";
pub const UNAUTHORIZED: &[u8] = b"HTTP/1.1 401 UNAUTHORIZED\r\n";
pub const FORBIDDEN: &[u8] = b"HTTP/1.1 403 FORBIDDEN\r\n";
pub const METHOD_NOT_ALLOWED: &[u8] = b"HTTP/1.1 405 METHOD NOT ALLOWED\r\n";


pub const INTERNAL_ERROR: &[u8] = b"HTTP/1.1 500 INTERNAL SERVER ERROR\r\n";
pub const SERVICE_UNAVAILABLE: &[u8] = b"HTTP/1.1 503 SERVICE UNAVAILABLE\r\n";