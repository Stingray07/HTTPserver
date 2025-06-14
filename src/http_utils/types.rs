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

#[derive(Serialize)]
pub struct ApiResponse {
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

#[derive(Debug, Clone)]
pub enum UniversalBody {
    Json(Value),
    Binary(Vec<u8>),
    Text(String)
}