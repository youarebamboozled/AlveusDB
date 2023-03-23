use std::net::{TcpStream};
use std::io::{Read, Write};
use crate::{error, debug, info};
use crate::http_response::{HttpResponse};
use crate::http_header::HttpHeader;
use std::time::Instant;
use crate::json::{Query, QueryResultType, read, write};


pub(crate) fn handle_request(mut stream: &TcpStream) {
    let start = Instant::now();
    let mut first_buffer = [0; 1024];
    match stream.read(&mut first_buffer) {
        Ok(_) => {}
        Err(e) => {
            error!("Error while trying to read the stream: {}", e);
            return;
        }
    }
    let mut buffer = first_buffer.to_vec();

    // get the content length from the request and read the rest of the request
    if String::from_utf8_lossy(&first_buffer[..]).contains("Content-Length: ") == false  && String::from_utf8_lossy(&first_buffer[..]).contains("GET") == false {
        error!("Content-Length header not found");
        let response = HttpResponse::new()
            .with_status_code(400)
            .with_header(HttpHeader::new().with_content_type("application/json".to_string()))
            .with_content("{\"status\": \"error\", \"message\": \"Content-Length header not found\"}".to_string());
        match stream.write(response.to_string().as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                error!("Error while trying to write to the stream: {}", e);
            }
        }
        match stream.flush() {
            Ok(_) => {}
            Err(e) => {
                error!("Error while trying to flush the stream: {}", e);
            }
        }
        return;
    }
    if String::from_utf8_lossy(&first_buffer[..]).contains("GET") == false {
        let content_length = match String::from_utf8_lossy(&first_buffer[..]).split("Content-Length: ").collect::<Vec<&str>>()[1].split("\r\n").collect::<Vec<&str>>()[0].parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                error!("Error while trying to parse the content length: {}", e);
                let response = HttpResponse::new()
                    .with_status_code(400)
                    .with_header(HttpHeader::new().with_content_type("application/json".to_string()))
                    .with_content("{\"status\": \"error\", \"message\": \"Error while trying to parse the content length\"}".to_string());
                match stream.write(response.to_string().as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error while trying to write to the stream: {}", e);
                    }
                }
                match stream.flush() {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error while trying to flush the stream: {}", e);
                    }
                }
                return;
            }
        };
        if content_length <= 0 {
            error!("No content found");
            let response = HttpResponse::new()
                .with_status_code(400)
                .with_header(HttpHeader::new().with_content_type("application/json".to_string()))
                .with_content("{\"status\": \"error\", \"message\": \"No content found\"}".to_string());
            match stream.write(response.to_string().as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    error!("Error while trying to write to the stream: {}", e);
                }
            }
            match stream.flush() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error while trying to flush the stream: {}", e);
                }
            }
            return;
        }
        let mut second_buffer = vec![0; content_length];
        match stream.read(&mut second_buffer) {
            Ok(_) => {}
            Err(e) => {
                error!("Error while trying to read the stream: {}", e);
                return;
            }
        }

        buffer = first_buffer.to_vec();
        buffer.append(&mut second_buffer);
    }
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
    match stream.write(response.to_string().as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            error!("Error while trying to write to the stream: {}", e);
        }
    }
    match stream.flush() {
        Ok(_) => {}
        Err(e) => {
            error!("Error while trying to flush the stream: {}", e);
        }
    }
    // log the request in this format:
    // [IP] [METHOD] [PATH] [STATUS CODE] [TIME]
    let ip = match stream.peer_addr() {
        Ok(addr) => addr.ip().to_string(),
        Err(e) => {
            error!("Error while trying to get the ip address: {}", e);
            "unknown".to_string()
        }
    };
    if response.status_code == 200 && response.content.contains("time\": \"") {
        info!("{} {} {} {} {}", ip, req_method, path, response.status_code.to_string(),
            response.content.split("time\": \"").collect::<Vec<&str>>()[1].split("\"").collect::<Vec<&str>>()[0]);
    } else {
        info!("{} {} {} {}", ip, req_method, path, response.status_code.to_string());
    }
}

fn db_handler(binding: String) -> HttpResponse {
    let req_method = binding.split(" ").collect::<Vec<&str>>()[0];
    let header = binding.split("\r\n\r\n").collect::<Vec<&str>>()[0];
    let body = binding.split("\r\n\r\n").collect::<Vec<&str>>()[1];

    let response = match req_method {
        "GET" => db_get_handler(body, header),
        "POST" => db_post_handler(body, header),
        "/favicon.ico" => favicon(),
        _ => unsupported_method(),
    };

    response
}

fn db_get_handler(_body: &str, header: &str) -> HttpResponse {
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
            response.content = r#"{"time": ""#.to_string() + &time.to_string() + r#"μs", "data": "# + match &result.query.content {
                Some(content) => content,
                None => "",
            } + r#"}"#;
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
            response.content = r#"{"time": ""#.to_string() + &time.to_string() + r#"μs", "data": "# + match &result.query.content {
                Some(content) => content,
                None => "",
            } + r#"}"#;
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
    response.content = "<h1> AlveusDB </h1> <p> A simple database server written in Rust </p> <p> The corresponding <a href=\"https://github.com/youarebamboozled/AlveusDB\">Github</a> </p>".to_string();
    response
}

#[allow(dead_code)]
fn dummy_response(path: &str, method: &str) -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("text/html".to_string());
    response.content = format!("{} - {} - {}", path, method, response.status_code.to_string()).to_string();
    response
}

fn favicon() -> HttpResponse {
    let mut response = HttpResponse::new();
    response.protocol = "HTTP/1.1".to_string();
    response.status_code = 200;
    response.header = HttpHeader::new()
        .with_content_type("image/x-icon".to_string());
    response.content = "".to_string();
    response
}