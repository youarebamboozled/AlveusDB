use std::borrow::Cow;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::{error, debug, info, warn, verbose, fatal};
use crate::http_response::{HttpResponse};
use crate::http_header::HttpHeader;
use crate::http_handler::handle_request;

pub(crate) struct Listener {
    address: String,
    port: u16,
}

impl Listener {
    pub(crate) fn new() -> Listener {
        Listener {
            address: "127.0.0.1".to_string(),
            port: 8080,
        }
    }

    pub(crate) fn listen(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port.to_string())).unwrap();
        println!("Listening on {}:{}", self.address, self.port.to_string());
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_request(stream);
                }
                Err(e) => {
                    error!("Error while trying to listen: {}", e); //TODO: change the err message
                }
            }
        }
    }
}

