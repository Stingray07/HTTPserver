use serde::Serialize;

#[derive(Clone)]
#[derive(Serialize)]
pub enum Status {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    NotFound = 404,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    RequestTimeout = 408,
    MethodNotAllowed = 405,
    InternalError = 500,
    ServiceUnavailable = 503,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ParseError {
    MalformedRequest,
    ConnectionAborted,
}

impl Status {
    pub fn line(&self) -> &'static [u8] {
        match self {
            Self::Ok => b"HTTP/1.1 200 OK\r\n",
            Self::Created => b"HTTP/1.1 201 CREATED\r\n",
            Self::NoContent => b"HTTP/1.1 204 NO CONTENT\r\n",
            Self::NotFound => b"HTTP/1.1 404 NOT FOUND\r\n",
            Self::BadRequest => b"HTTP/1.1 400 BAD REQUEST\r\n",
            Self::Unauthorized => b"HTTP/1.1 401 UNAUTHORIZED\r\n",
            Self::Forbidden => b"HTTP/1.1 403 FORBIDDEN\r\n",
            Self::RequestTimeout => b"HTTP/1.1 408 REQUEST TIMEOUT\r\n",
            Self::MethodNotAllowed => b"HTTP/1.1 405 METHOD NOT ALLOWED\r\n",
            Self::InternalError => b"HTTP/1.1 500 INTERNAL SERVER ERROR\r\n",
            Self::ServiceUnavailable => b"HTTP/1.1 503 SERVICE UNAVAILABLE\r\n",
        }
    }
}