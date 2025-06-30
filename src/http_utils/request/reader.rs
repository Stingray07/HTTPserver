use crate::http_utils::status::ParseError;
use crate::http_utils::parser;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::time::timeout;
use tokio::time::Duration;


async fn read_header<'a>(stream: &mut TcpStream, pre_buffer: &mut [u8], dynamo_buffer: &'a mut Vec<u8>) -> Result<&'a mut Vec<u8>, ParseError> {
    loop {
        match stream.read(pre_buffer).await {
            Ok(0) => {
                eprintln!("Connection closed before complete headers");
                return Err(ParseError::ConnectionAborted)
            }
            Ok(n) => {
                dynamo_buffer.extend_from_slice(&pre_buffer[..n]); // Only use bytes read
                if dynamo_buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("Connection timed out");
                return Err(ParseError::ConnectionAborted);
            }

            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return Err(ParseError::MalformedRequest); 
            }
        }
    }
    Ok(dynamo_buffer)
}

async fn read_body<'a>(content_length: usize, stream: &mut TcpStream, full_body: &'a mut Vec<u8>) -> Result<&'a mut Vec<u8>, ParseError> {
    if content_length == 0 {
        return Ok(full_body);
    }
    
    let mut body_buffer = vec![0; content_length - full_body.len()];
    let read_result = stream.read_exact(&mut body_buffer);

    match read_result.await {
        Ok(_) => {
            full_body.extend_from_slice(&body_buffer);
            Ok(full_body)
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
            eprintln!("Connection timed out");
            return Err(ParseError::ConnectionAborted);
        }
        Err(e) => {
            eprintln!("Failed to read body: {}", e);
            return Err(ParseError::MalformedRequest);
        }
    }
}

pub async fn full_read_request(stream: &mut TcpStream, pre_buffer: &mut [u8], dynamo_buffer: &mut Vec<u8>) -> Result<Vec<u8>, ParseError> {

    println!("Reading header...");
    match timeout(Duration::from_secs(10), read_header(stream, pre_buffer, dynamo_buffer)).await {
        Ok(_) => {
            println!("Header read");
        },
        Err(e) => {
            eprintln!("Error reading header: {:?}", e);
            return Err(ParseError::ConnectionAborted);
        }
    }
    
    let header_end = dynamo_buffer.windows(4).position(|window| window == b"\r\n\r\n");
    let content_length = parser::get_content_length(&dynamo_buffer);
    let body_start = header_end.unwrap() + 4;
    let already_read_body =  &dynamo_buffer[body_start..];
    let mut full_body = already_read_body.to_vec();

    let content_length = match content_length {
        Ok(content_length) => {
            content_length
        }
        Err(e) => {
            eprintln!("Error getting content length: {:?}", e);
            let error = e;
            return Err(error);
        }
    };

    match timeout(Duration::from_secs(10), read_body(content_length, stream, &mut full_body)).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading body: {:?}", e);
            return Err(ParseError::ConnectionAborted);
        }
    }

    let mut full_request = dynamo_buffer[..body_start].to_vec();
    full_request.extend_from_slice(&full_body);

    println!("FULL Request: {}", String::from_utf8_lossy(&full_request[..]));
    println!("================================");
    Ok(full_request)
}
