use std::collections::HashMap;
use serde_json::Value as JsonValue;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug)]
pub struct ApiRequest {
    pub path: String,
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: UniversalBody,  
}

pub struct Response {
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}


#[derive(Serialize)]
pub struct ApiBody {
    pub status: String,
    pub body: JsonValue,
}

#[derive(Debug)]
pub struct HttpRequest{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: UniversalBody,
}

#[derive(Debug)]
pub enum ParsedRequest {
    Api(ApiRequest),
    HTTP(HttpRequest),
}

impl Response {
    pub fn convert_to_vec(&self) -> Vec<u8> {
        match self {
            Response { status, headers, body } => {
                let header_vec = Response::header_to_vec(headers.clone());
                let body_vec = body.clone();
                let mut response_vec = Vec::new();
                response_vec.extend_from_slice(status.as_bytes());
                response_vec.extend_from_slice(&header_vec);
                response_vec.extend_from_slice(b"\r\n");
                response_vec.extend_from_slice(&body_vec);
                response_vec
            }
        }
    }

    fn header_to_vec(header: HashMap<String, String>) -> Vec<u8> {
        let mut header_vec = Vec::new();
        for (key, value) in header {
            header_vec.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
        }
        header_vec.extend_from_slice(b"\r\n");
        header_vec
    }
}

#[derive(Debug, Clone)]
pub enum UniversalBody {
    Json(Value),
    Binary(Vec<u8>),
    Text(String)
}