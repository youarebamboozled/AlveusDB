mod logger;
mod listener;
mod utils;
mod http_response;
mod http_header;
mod http_handler;
mod json;

use crate::logger::log_level::LogLevel;

fn main() {
    logger::Builder::new()
        .level(LogLevel::Debug)
        .build();
    debug!("Logger initialized");

    let listener = listener::Listener::new();
    listener.listen();
}
