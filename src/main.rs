mod logger;
use crate::logger::log_level::LogLevel;

fn main() {
    logger::Builder::new()
        .level(LogLevel::Debug)
        .build();
    info!("This is an info message");
    debug!("This is a debug message");
    verbose!("This is a verbose message");
    warn!("This is a warn message");
    error!("This is an error message");
    fatal!("This is a fatal message");
}
