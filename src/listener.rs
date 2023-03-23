use std::net::{TcpListener};
use crate::{error, info};
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
        let listener = match TcpListener::bind(format!("{}:{}", self.address, self.port.to_string())) {
            Ok(listener) => listener,
            Err(e) => {
                error!("Error while trying to listen: {}", e);
                return;
            }
        };
        info!("Listening on {}:{}", self.address, self.port.to_string());
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_request(&stream);
                }
                Err(e) => {
                    error!("Error while trying to listen: {}", e); //TODO: change the err message
                }
            }
        }
    }
}

