use std::borrow::Cow;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::{error, debug, info, warn, verbose, fatal};
use crate::http_response::{HttpResponse};
use crate::http_header::HttpHeader;
use std::time::Instant;


pub(crate) fn handle_request(mut stream: TcpStream) {
    let start = Instant::now();
    // get the path from the request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // get the path from the request
    let binding = String::from_utf8_lossy(&buffer[..]);
    let req_method = binding.split(" ").collect::<Vec<&str>>()[0];
    let path = binding.split(" ").collect::<Vec<&str>>()[1];
    debug!("Path: {}", path);
    debug!("Method: {}", req_method);

    // route the request to the correct handler
    let response = match path {
        "/" => index(),
        "/sql" => db_handler(binding),
        _ => unknown_path(),
    };

    // send the response
    let end = Instant::now();
    let duration = end.duration_since(start);
    debug!("Request took {}µs", duration.as_micros());
    let response = response.to_string();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn db_handler(binding: Cow<str>) -> HttpResponse {
    debug!("Request: {}", binding);
    let req_method = binding.split(" ").collect::<Vec<&str>>()[0];
    let header = binding.split("\r\n\r\n").collect::<Vec<&str>>()[0];
    debug!("Header: {}", header);
    let body = binding.split("\r\n\r\n").collect::<Vec<&str>>()[1];
    debug!("Body: {}", body);

    let response = match req_method {
        "GET" => db_get_handler(body, header),
        "POST" => db_post_handler(body, header),
        _ => unsupported_method(),
    };

    response
}

fn db_get_handler(body: &str, header: &str) -> HttpResponse {
    debug!("Body: {}", body);
    debug!("Header: {}", header);
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = format!("Connecting to database: {} and table: {}", "test", "test");
    response
}

fn db_post_handler(body: &str, header: &str) -> HttpResponse {
    debug!("Body: {}", body);
    debug!("Header: {}", header);
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = format!("Connecting to database: {} and table: {}", "test", "test");
    response
}

fn unsupported_method() -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 405;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = "<h1>405 - Method Not Allowed</h1>".to_string();
    response
}

fn unknown_path() -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 404;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = "<h1>404 - Not Found</h1>".to_string();
    response
}

fn index() -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = "<h1>Hello World!</h1>".to_string();
    response
}

fn dummy_response(path: &str, method: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = format!("{} - {} - {}", path, method, response.status_code.to_string()).to_string();
    response
}