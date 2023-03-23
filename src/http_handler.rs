use std::borrow::Cow;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::{error, debug, info, warn, verbose, fatal};
use crate::http_response::{HttpResponse};
use crate::http_header::HttpHeader;
use std::time::Instant;
use crate::json::{Query, QueryResultType, read, write};
use std::io::ErrorKind;


pub(crate) fn handle_request(mut stream: &TcpStream) {
    let start = Instant::now();
    let mut first_buffer = [0; 256];
    stream.read(&mut first_buffer).unwrap();

    // get the content length from the request and read the rest of the request
    let content_length = String::from_utf8_lossy(&first_buffer[..]).split("Content-Length: ").collect::<Vec<&str>>()[1].split("\r\n").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    let mut second_buffer = vec![0; content_length];
    debug!("Content length: {}", content_length);
    stream.read(&mut second_buffer).unwrap();

    let mut buffer = first_buffer.to_vec();
    buffer.append(&mut second_buffer);

    // get the path from the request
    let binding = String::from_utf8_lossy(&buffer[..]);
    //remove every \0 from the string
    let binding = binding.replace("\0", "");
    let req_method = binding.split(" ").collect::<Vec<&str>>()[0];
    let path = binding.split(" ").collect::<Vec<&str>>()[1];

    // route the request to the correct handler
    let response = match path {
        "/" => index(),
        "/sql" => db_handler(binding.clone()),
        _ => unknown_path(),
    };

    // send the response
    let end = Instant::now();
    let duration = end.duration_since(start);
    debug!("Request took {}µs", duration.as_micros());
    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
    // log the request in this format:
    // [IP] [METHOD] [PATH] [STATUS CODE] [TIME]
    let ip = stream.peer_addr().unwrap().ip().to_string();
    info!("{} {} {} {} {}", ip, req_method, path, response.status_code.to_string(),
        response.content.split("time\": \"").collect::<Vec<&str>>()[1].split("\"").collect::<Vec<&str>>()[0]);
}

fn db_handler(binding: String) -> HttpResponse {
    let req_method = binding.split(" ").collect::<Vec<&str>>()[0];
    let header = binding.split("\r\n\r\n").collect::<Vec<&str>>()[0];
    let body = binding.split("\r\n\r\n").collect::<Vec<&str>>()[1];

    let response = match req_method {
        "GET" => db_get_handler(body, header),
        "POST" => db_post_handler(body, header),
        _ => unsupported_method(),
    };

    response
}

fn db_get_handler(body: &str, header: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("application/json".to_string());
    // get the database and table from the request header which are in the form of:
    // DB: test
    // TABLE: test
    if !header.contains("DB: ") || !header.contains("TABLE: ") {
        response.status_code = 400;
        response.content = r#"{"error": "Bad Request. Missing or malformed DB or TABLE header"}"#.to_string();
        return response;
    }
    let database = String::from(header.split("DB: ").collect::<Vec<&str>>()[1].split("\r\n").collect::<Vec<&str>>()[0]);
    let table = String::from(header.split("TABLE: ").collect::<Vec<&str>>()[1].split("\r\n").collect::<Vec<&str>>()[0]);
    let query = Query {
        database,
        table,
        content: None,
    };
    let time = Instant::now();
    let result = read(&query);
    let duration = time.elapsed();
    let time = duration.as_micros();
    match result.status {
        QueryResultType::Success => {
            response.content = r#"{"time": ""#.to_string() + &time.to_string() + r#"μs", "data": "# + &result.query.content.unwrap() + r#"}"#;
        },
        QueryResultType::Error => {
            response.status_code = 500;
            response.content = r#"{"error": ""#.to_string() + &result.message + r#""}"#;
        },
    }

    response
}

fn db_post_handler(body: &str, header: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("application/json".to_string());

    if !header.contains("DB: ") || !header.contains("TABLE: ") {
        response.status_code = 400;
        response.content = r#"{"error": "Bad Request. Missing or malformed DB or TABLE header"}"#.to_string();
        return response;
    }

    let database = String::from(header.split("DB: ").collect::<Vec<&str>>()[1].split("\r\n").collect::<Vec<&str>>()[0]);
    let table = String::from(header.split("TABLE: ").collect::<Vec<&str>>()[1].split("\r\n").collect::<Vec<&str>>()[0]);
    let query = Query {
        database,
        table,
        content: Some(body.to_string()),
    };
    let time = Instant::now();
    let result = write(&query);
    let duration = time.elapsed();
    let time = duration.as_micros();
    match result.status {
        QueryResultType::Success => {
            response.content = r#"{"time": ""#.to_string() + &time.to_string() + r#"μs", "data": "# + &result.query.content.unwrap() + r#"}"#;
        },
        QueryResultType::Error => {
            response.status_code = 500;
            response.content = r#"{"error": ""#.to_string() + &result.message + r#""}"#;
        },
    }

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